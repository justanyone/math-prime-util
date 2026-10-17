//! Bernoulli, primorials, class numbers, and permutation rank.
//! Ports of `util.c`, `intcomplexity.c`, and `congruent_numbers.c`.

use std::sync::{Mutex, OnceLock};

use super::arith::{binomial_mag, factorial_u128, gcd_u128, mod_rank};

fn gcd_u64(x: u64, y: u64) -> u64 {
    crate::gcd(x, y)
}

fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn next_prime(p: u64) -> u64 {
    if p < 2 {
        return 2;
    }
    let mut x = if p == 2 { 3 } else { p + 2 };
    if x % 2 == 0 {
        x += 1;
    }
    loop {
        if is_prime_u64(x) {
            return x;
        }
        x = x.saturating_add(2);
        if x <= 2 {
            return 2;
        }
    }
}

/// Bernoulli number `B_n` as a reduced fraction `(numerator, denominator)`.
///
/// Port of `bernfrac` in `util.c`. `B_1` is `1/2`. Odd `n > 1` is `0/1`.
/// `None` when a term does not fit in `i128` / `u128`.
///
/// ```
/// use math_prime_util::bernfrac;
/// assert_eq!(bernfrac(0), Some((1, 1)));
/// assert_eq!(bernfrac(1), Some((1, 2)));
/// assert_eq!(bernfrac(2), Some((1, 6)));
/// assert_eq!(bernfrac(4), Some((-1, 30)));
/// ```
#[must_use]
pub fn bernfrac(n: u64) -> Option<(i128, u128)> {
    if n == 1 {
        return Some((1, 2));
    }
    if n > 4096 {
        return None;
    }
    let all = bernoulli_through(n)?;
    let f = all[n as usize];
    Some((f.0, f.1))
}

fn bernoulli_through(n: u64) -> Option<Vec<(i128, u128)>> {
    // B_0 = 1. Internal B_1 = -1/2 so later terms match the standard recurrence.
    // The public value of B_1 is flipped to +1/2 by `bernfrac`.
    let mut b = Vec::with_capacity((n as usize) + 1);
    b.push((1i128, 1u128));
    for m in 1..=n {
        if m == 1 {
            b.push((-1, 2));
            continue;
        }
        if m % 2 == 1 {
            b.push((0, 1));
            continue;
        }
        let mut sum_n = 0i128;
        let mut sum_d = 1u128;
        for k in 0..m {
            let c = binomial_mag(u128::from(m + 1), u128::from(k))?;
            let (tn, td) = frac_mul_u(b[k as usize], c)?;
            (sum_n, sum_d) = frac_add((sum_n, sum_d), (tn, td))?;
        }
        let num = sum_n.checked_neg()?;
        let den = sum_d.checked_mul(u128::from(m + 1))?;
        b.push(reduce(num, den)?);
    }
    Some(b)
}

fn frac_mul_u(a: (i128, u128), k: u128) -> Option<(i128, u128)> {
    let n = a.0.checked_mul(i128::try_from(k).ok()?)?;
    reduce(n, a.1)
}

fn frac_add(a: (i128, u128), b: (i128, u128)) -> Option<(i128, u128)> {
    let ad = i128::try_from(a.1).ok()?;
    let bd = i128::try_from(b.1).ok()?;
    let left = a.0.checked_mul(bd)?;
    let right = b.0.checked_mul(ad)?;
    let num = left.checked_add(right)?;
    let den = a.1.checked_mul(b.1)?;
    reduce(num, den)
}

fn reduce(n: i128, d: u128) -> Option<(i128, u128)> {
    if d == 0 {
        return None;
    }
    let neg = n < 0;
    let mut an = n.unsigned_abs();
    let mut d = d;
    let g = gcd_u128(an, d);
    an /= g;
    d /= g;
    let n = i128::try_from(an).ok()?;
    Some((if neg { n.checked_neg()? } else { n }, d))
}

/// Harmonic number `H_n` as a reduced fraction.
///
/// Port of `harmfrac` in `util.c` (binary split). `H_0 = 0/1`.
/// `None` when the fraction does not fit in `u128`.
///
/// ```
/// use math_prime_util::harmfrac;
/// assert_eq!(harmfrac(0), Some((0, 1)));
/// assert_eq!(harmfrac(2), Some((3, 2)));
/// ```
#[must_use]
pub fn harmfrac(n: u64) -> Option<(i128, u128)> {
    if n == 0 {
        return Some((0, 1));
    }
    let (num, den) = harmonic_split(1, u128::from(n) + 1)?;
    let g = gcd_u128(num, den);
    let num = num / g;
    let den = den / g;
    Some((i128::try_from(num).ok()?, den))
}

fn harmonic_split(a: u128, b: u128) -> Option<(u128, u128)> {
    if b - a == 1 {
        return Some((1, a));
    }
    if b - a == 2 {
        return Some((a + a + 1, a.checked_mul(a)?.checked_add(a)?));
    }
    let m = (a + b) >> 1;
    let (p, q) = harmonic_split(a, m)?;
    let (r, s) = harmonic_split(m, b)?;
    let num = p.checked_mul(s)?.checked_add(q.checked_mul(r)?)?;
    let den = q.checked_mul(s)?;
    let g = gcd_u128(num, den);
    Some((num / g, den / g))
}

/// Primorial `n#`: product of the primes `<= n`.
///
/// Port of `primorial` in `util.c`. `None` when the product does not fit in
/// `u128`. `n < 2` gives 1.
///
/// ```
/// use math_prime_util::primorial;
/// assert_eq!(primorial(5), Some(30));
/// assert_eq!(primorial(1), Some(1));
/// ```
#[must_use]
pub fn primorial(n: u64) -> Option<u128> {
    if n < 2 {
        return Some(1);
    }
    let mut acc = 1u128;
    let mut p = 2u64;
    while p <= n {
        acc = acc.checked_mul(u128::from(p))?;
        let nxt = next_prime(p);
        if nxt <= p {
            break;
        }
        p = nxt;
    }
    Some(acc)
}

/// Product of the first `n` primes.
///
/// Port of `pn_primorial` in `util.c`. `None` when the product does not fit
/// in `u128`. `n = 0` gives 1.
///
/// ```
/// use math_prime_util::pn_primorial;
/// assert_eq!(pn_primorial(0), Some(1));
/// assert_eq!(pn_primorial(3), Some(30));
/// ```
#[must_use]
pub fn pn_primorial(n: u64) -> Option<u128> {
    if n == 0 {
        return Some(1);
    }
    let mut acc = 1u128;
    let mut p = 2u64;
    for _ in 0..n {
        acc = acc.checked_mul(u128::from(p))?;
        p = next_prime(p);
    }
    Some(acc)
}

/// Least common multiple of `1, 2, ..., n`. `n = 0` gives 1.
///
/// Port of `consecutive_integer_lcm` in `util.c`. `None` when the value does
/// not fit in `u128`.
///
/// ```
/// use math_prime_util::consecutive_integer_lcm;
/// assert_eq!(consecutive_integer_lcm(5), Some(60));
/// assert_eq!(consecutive_integer_lcm(0), Some(1));
/// ```
#[must_use]
pub fn consecutive_integer_lcm(n: u64) -> Option<u128> {
    if n <= 1 {
        return Some(1);
    }
    // log lcm(1..=n) ~ n, so n above ~300 cannot fit in u128.
    if n > 400 {
        return None;
    }
    let mut acc = 1u128;
    let mut p = 2u64;
    while p <= n {
        let mut pk = u128::from(p);
        loop {
            let next = pk.checked_mul(u128::from(p))?;
            if next > u128::from(n) {
                break;
            }
            pk = next;
        }
        acc = acc.checked_mul(pk)?;
        p = next_prime(p);
    }
    Some(acc)
}

/// Frobenius coin problem for a set of positive integers.
///
/// Port of `frobenius_number` in `util.c` (two-variable closed form, otherwise
/// the Round Robin algorithm of Böcker and Lipták). A set that contains 1
/// gives `-1`. An empty set or a single integer other than 1 gives `None`.
///
/// # Panics
///
/// Panics when an argument is 0, when the set is not coprime, or when the
/// smallest element is too large to run the algorithm (`> 10_000_000`).
///
/// ```
/// use math_prime_util::frobenius_number;
/// assert_eq!(frobenius_number(&[2, 5]), Some(3));
/// assert_eq!(frobenius_number(&[1, 4]), Some(-1));
/// assert_eq!(frobenius_number(&[7]), None);
/// ```
#[must_use]
pub fn frobenius_number(vals: &[u64]) -> Option<i128> {
    if vals.iter().any(|&x| x == 0) {
        panic!("frobenius_number: arguments must be positive");
    }
    if vals.iter().any(|&x| x == 1) {
        return Some(-1);
    }
    if vals.len() <= 1 {
        return None;
    }
    let mut a = vals.to_vec();
    a.sort_unstable();
    let mut g = a[0];
    for &x in &a[1..] {
        g = gcd_u64(g, x);
    }
    if g != 1 {
        panic!("frobenius_number: set must be coprime");
    }
    if a.len() == 2 {
        let a0 = u128::from(a[0]);
        let a1 = u128::from(a[1]);
        let v = a0.checked_mul(a1)?.checked_sub(a0)?.checked_sub(a1)?;
        return i128::try_from(v).ok();
    }
    if a[0] > 10_000_000 {
        panic!("frobenius_number: input too large");
    }
    round_robin(&a)
}

fn round_robin(a: &[u64]) -> Option<i128> {
    let nlen = a[0] as usize;
    let mut nvec = vec![u128::MAX; nlen];
    nvec[0] = 0;
    for i in 1..a.len() {
        let ai = u128::from(a[i]);
        let slot = (a[i] % a[0]) as usize;
        if nvec[slot] != u128::MAX && nvec[slot] <= ai {
            continue;
        }
        let d = gcd_u64(a[0], a[i]) as usize;
        for r in 0..d {
            let mut n = 0u128;
            if r > 0 {
                n = u128::MAX;
                let mut q = r;
                while q < nlen {
                    if nvec[q] < n {
                        n = nvec[q];
                    }
                    q += d;
                }
            }
            if n == u128::MAX {
                continue;
            }
            let steps = nlen / d;
            for _ in 1..steps {
                n = n.checked_add(ai)?;
                let p = (n % u128::from(a[0])) as usize;
                if nvec[p] >= n {
                    nvec[p] = n;
                } else {
                    n = nvec[p];
                }
            }
        }
    }
    let mut max = 0u128;
    for &v in &nvec {
        if v == u128::MAX || v > max {
            max = v;
        }
    }
    if max == u128::MAX {
        return None;
    }
    let ans = max.checked_sub(u128::from(a[0]))?;
    i128::try_from(ans).ok()
}

/// `true` when `n` is a congruent number.
///
/// Port of `is_congruent_number` in `congruent_numbers.c`. The square part is
/// removed, then Tunnell's criterion decides the rest (weak BSD, as in MPU).
///
/// # Panics
///
/// Panics when `n` is negative.
///
/// ```
/// use math_prime_util::is_congruent_number;
/// assert!(is_congruent_number(5));
/// assert!(is_congruent_number(6));
/// assert!(!is_congruent_number(1));
/// ```
#[must_use]
pub fn is_congruent_number(n: i64) -> bool {
    if n < 0 {
        panic!("is_congruent_number: negative argument");
    }
    let n = n as u64;
    if n == 0 {
        return false;
    }
    let n = square_free_kernel(n);
    if n < 13 {
        return (5..=7).contains(&n);
    }
    // ACK: n = 5, 6, or 7 (mod 8) is congruent. Same early return as filter 1.
    if n % 8 >= 5 {
        return true;
    }
    tunnell(n)
}

fn square_free_kernel(mut n: u64) -> u64 {
    let mut k = 1u64;
    let mut e = 0u32;
    while n % 2 == 0 {
        n /= 2;
        e += 1;
    }
    if e % 2 == 1 {
        k *= 2;
    }
    let mut p = 3u64;
    while p <= n / p {
        if n % p == 0 {
            let mut e = 0u32;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            if e % 2 == 1 {
                k = k.saturating_mul(p);
            }
        }
        p += 2;
    }
    if n > 1 {
        k = k.saturating_mul(n);
    }
    k
}

fn tunnell(n: u64) -> bool {
    let mut sols = [0u64; 2];
    if n % 2 == 1 {
        let limz = integer_sqrt(n / 8);
        for z in 0..=limz {
            let n8z = n - 8 * z * z;
            let limy = integer_sqrt(n8z / 2);
            let mut zsols = 0u64;
            for y in 0..=limy {
                let x = n8z - 2 * y * y;
                if crate::is_square(x) {
                    let shift = 1 + u32::from(y > 0) + u32::from(z > 0);
                    zsols += 1u64 << shift;
                }
            }
            sols[(z & 1) as usize] += zsols;
        }
    } else {
        let limz = integer_sqrt((n / 2) / 8);
        for z in 0..=limz {
            let n8z = n / 2 - 8 * z * z;
            let limx = integer_sqrt(n8z);
            let mut zsols = 0u64;
            let mut x = 1u64;
            while x <= limx {
                let y = n8z - x * x;
                if y == 0 || crate::is_square(y) {
                    let shift = 1 + u32::from(y > 0) + u32::from(z > 0);
                    zsols += 1u64 << shift;
                }
                x += 2;
            }
            sols[(z & 1) as usize] += zsols;
        }
    }
    sols[0] == sols[1]
}

fn integer_sqrt(n: u64) -> u64 {
    crate::sqrtint(n)
}

/// `true` when `n` is a `k`-gonal number. `k >= 3`. `0` and `1` are polygonal.
///
/// Port of `polygonal_root` in `util.c`. A negative `n` is not polygonal.
///
/// # Panics
///
/// Panics when `k < 3`.
///
/// ```
/// use math_prime_util::is_polygonal;
/// assert!(is_polygonal(6, 3));
/// assert!(!is_polygonal(5, 3));
/// assert!(is_polygonal(0, 4));
/// ```
#[must_use]
pub fn is_polygonal(n: i128, k: u64) -> bool {
    if k < 3 {
        panic!("is_polygonal: k must be >= 3");
    }
    if n < 0 {
        return false;
    }
    polygonal_u128(n as u128, u128::from(k))
}

fn polygonal_u128(n: u128, k: u128) -> bool {
    if n <= 1 {
        return true;
    }
    if k == 4 {
        return is_square_u128(n);
    }
    // `k = 3` subtracts 1 from the root (`k - 4 = -1` in the C code).
    let (disc, step, adjust_down) = if k == 3 {
        let Some(d) = n.checked_mul(8).and_then(|v| v.checked_add(1)) else {
            return false;
        };
        (d, 2u128, true)
    } else {
        let Some(coef) = k.checked_mul(8).and_then(|v| v.checked_sub(16)) else {
            return false;
        };
        let Some(sq) = (k - 4).checked_mul(k - 4) else {
            return false;
        };
        let Some(d) = coef.checked_mul(n).and_then(|v| v.checked_add(sq)) else {
            return false;
        };
        (d, k * 2 - 4, false)
    };
    let s = sqrt_u128(disc);
    if s.checked_mul(s) != Some(disc) {
        return false;
    }
    let num = if adjust_down {
        s.checked_sub(1)
    } else {
        s.checked_add(k - 4)
    };
    let Some(num) = num else {
        return false;
    };
    step != 0 && num % step == 0
}

fn is_square_u128(n: u128) -> bool {
    let s = sqrt_u128(n);
    s.checked_mul(s) == Some(n)
}

fn sqrt_u128(n: u128) -> u128 {
    if n <= u128::from(u64::MAX) {
        return u128::from(crate::sqrtint(n as u64));
    }
    // Newton, starting from the top 64 bits.
    let mut x = u128::from(crate::sqrtint((n >> 64) as u64)) << 32;
    if x == 0 {
        x = 1;
    }
    loop {
        let q = n / x;
        let nxt = (x + q) >> 1;
        if nxt >= x {
            break;
        }
        x = nxt;
    }
    while x.checked_mul(x).is_none_or(|s| s > n) {
        x -= 1;
    }
    while (x + 1).checked_mul(x + 1).is_some_and(|s| s <= n) {
        x += 1;
    }
    x
}

/// Pillai function `v`: smallest `v` with `v! ≡ -1 (mod n)` and `n % v != 1`.
///
/// Port of `pillai_v` in `util.c`. Returns 0 when no such `v` exists.
///
/// ```
/// use math_prime_util::is_pillai;
/// assert_eq!(is_pillai(23), 14);
/// assert_eq!(is_pillai(1), 0);
/// ```
#[must_use]
pub fn is_pillai(n: u64) -> u64 {
    if n < 23 || n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        return 0;
    }
    let mut fac = 5040 % n;
    let mut v = 8u64;
    while v < n - 1 && fac != 0 {
        fac = crate::modular::mulmod_u(fac, v, n);
        if fac == n - 1 && n % v != 1 {
            return v;
        }
        v += 1;
    }
    0
}

/// Happy-number height of `n` in the given base and power.
///
/// Port of `happy_height` in `util.c`. The height is 0 when the iteration
/// does not reach 1. Returns `-1` when a digit power does not fit in `u128`.
///
/// # Panics
///
/// Panics when `base < 2`.
///
/// ```
/// use math_prime_util::is_happy;
/// assert_eq!(is_happy(1, 10, 2), 1);
/// assert_eq!(is_happy(7, 10, 2), 6);
/// assert_eq!(is_happy(2, 10, 2), 0);
/// ```
#[must_use]
pub fn is_happy(n: u128, base: u64, exponent: u32) -> i32 {
    if base < 2 {
        panic!("is_happy: base must be >= 2");
    }
    if base == 10 && exponent == 2 {
        const SH: [i32; 101] = [
            0, 1, 0, 0, 0, 0, 0, 6, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 5, 0, 0, 0, 4, 0, 0, 0, 0, 4,
            0, 0, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 4, 0, 0, 0, 3,
            0, 0, 0, 0, 5, 0, 0, 5, 0, 0, 4, 0, 0, 2,
        ];
        let mut n = n;
        let mut h = 0i32;
        while n > 100 {
            n = sum_squared_digits(n);
            h += 1;
        }
        let s = SH[n as usize];
        return if s == 0 { 0 } else { h + s };
    }
    let mut n = n;
    let mut ncheck = 0u128;
    let mut h = 1i32;
    while n > 1 && n != ncheck {
        if h & (h - 1) == 0 {
            ncheck = n;
        }
        let Some(next) = sum_digit_powers(n, base, exponent) else {
            return -1;
        };
        n = next;
        h += 1;
    }
    if n == 1 { h } else { 0 }
}

fn sum_squared_digits(mut n: u128) -> u128 {
    let mut sum = 0u128;
    while n > 0 {
        let r = n % 10;
        sum += r * r;
        n /= 10;
    }
    sum
}

fn sum_digit_powers(mut n: u128, base: u64, exp: u32) -> Option<u128> {
    let base = u128::from(base);
    let mut sum = 0u128;
    if n == 0 {
        return Some(0);
    }
    while n > 0 {
        let r = n % base;
        let term = if exp == 0 {
            1
        } else if exp == 1 {
            r
        } else {
            let mut t = 1u128;
            for _ in 0..exp {
                t = t.checked_mul(r)?;
            }
            t
        };
        sum = sum.checked_add(term)?;
        n /= base;
    }
    Some(sum)
}

/// `12 * H(n)`, the Hurwitz class number scaled to an integer.
///
/// Port of `hclassno` in `util.c`. `H(0) = -1/12`, so the return is `-1`.
/// A negative `n` gives 0.
///
/// ```
/// use math_prime_util::hclassno;
/// assert_eq!(hclassno(0), -1);
/// assert_eq!(hclassno(-3), 0);
/// assert_eq!(hclassno(3), 4);
/// ```
#[must_use]
pub fn hclassno(n: i64) -> i128 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return -1;
    }
    hclassno_pos(n as u64)
}

fn hclassno_pos(n: u64) -> i128 {
    let nmod4 = n % 4;
    if nmod4 == 1 || nmod4 == 2 {
        return 0;
    }
    if n == 3 {
        return 4;
    }
    let mut b = n & 1;
    let mut b2 = (n + 1) >> 2;
    let square = crate::is_square(b2);
    let mut h = u128::from(sigma0(b2) / 2);
    if b == 1 {
        let sq = u128::from(square);
        h = 1 + sq + ((h - 1) << 1);
    }
    b += 2;
    loop {
        let bb = b.saturating_mul(b);
        b2 = n.saturating_add(bb) >> 2;
        if b2 > u64::MAX / 3 || 3 * b2 >= n {
            break;
        }
        let div = u128::from(b != 0 && b2 % b == 0);
        let sq = u128::from(crate::is_square(b2));
        let cnt = u128::from(count_class_div(b + 1, b2));
        h += div + sq + (cnt << 1);
        b += 2;
    }
    let corr = if b2.checked_mul(3) == Some(n) {
        4u128
    } else if square && n % 2 == 0 {
        6
    } else {
        0
    };
    i128::try_from(12 * h + corr).unwrap_or(i128::MAX)
}

fn sigma0(mut n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut count = 1u64;
    let mut p = 2u64;
    while p <= n / p {
        if n % p == 0 {
            let mut e = 1u64;
            n /= p;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            count *= e + 1;
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 {
        count *= 2;
    }
    count
}

fn count_class_div(s: u64, b2: u64) -> u64 {
    let mut lim = integer_sqrt(b2);
    if lim.saturating_mul(lim) == b2 && lim > 0 {
        lim -= 1;
    }
    if s > lim {
        return 0;
    }
    let mut h = 0u64;
    let mut i = s;
    while i <= lim {
        if b2 % i == 0 {
            h += 1;
        }
        i += 1;
    }
    h
}

/// Permutation of `0..n-1` with rank `k` (factoriadic / Lehmer code).
///
/// Port of `num_to_perm` in `util.c`. `n = 0` gives an empty vector.
/// `k` is reduced modulo `n!` when `n!` fits in `u128` (`n <= 34`).
///
/// # Panics
///
/// Panics when `n > 4096`, or when `n > 34` and `k` is negative.
///
/// ```
/// use math_prime_util::numtoperm;
/// assert_eq!(numtoperm(3, 2), vec![1, 0, 2]);
/// assert_eq!(numtoperm(0, 0), Vec::<u64>::new());
/// ```
#[must_use]
pub fn numtoperm(n: u64, k: i128) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }
    if n > 4096 {
        panic!("numtoperm: n too large");
    }
    let n_us = n as usize;
    let mut items: Vec<u64> = (0..n).collect();
    let mut k = match factorial_u128(n) {
        Some(f) => mod_rank(k, f),
        None => {
            if k < 0 {
                panic!("numtoperm: negative rank with n > 34");
            }
            k as u128
        }
    };
    for i in 0..n_us {
        let rest = (n_us - 1 - i) as u64;
        let Some(f) = factorial_u128(rest) else {
            continue;
        };
        let digit = k / f;
        k %= f;
        if digit > 0 {
            let j = i + digit as usize;
            let t = items[j];
            for idx in (i + 1..=j).rev() {
                items[idx] = items[idx - 1];
            }
            items[i] = t;
        }
    }
    items
}

/// Rank of a permutation of `0..n-1`.
///
/// Port of `perm_to_num` in `util.c`. The empty permutation has rank 0.
/// `None` when the rank does not fit in `u128`.
///
/// # Panics
///
/// Panics when `perm` is not a permutation of `0..n-1`.
///
/// ```
/// use math_prime_util::permtonum;
/// assert_eq!(permtonum(&[1, 0, 2]), Some(2));
/// assert_eq!(permtonum(&[]), Some(0));
/// ```
#[must_use]
pub fn permtonum(perm: &[u64]) -> Option<u128> {
    let n = perm.len();
    if n == 0 {
        return Some(0);
    }
    let mut seen = vec![false; n];
    for &v in perm {
        if v as usize >= n || seen[v as usize] {
            panic!("permtonum: not a permutation of 0..n-1");
        }
        seen[v as usize] = true;
    }
    let mut num = 0u128;
    for i in 0..n - 1 {
        let rest = (n - 1 - i) as u64;
        let f = factorial_u128(rest)?;
        let mut smaller = 0u128;
        for j in i + 1..n {
            if perm[j] < perm[i] {
                smaller += 1;
            }
        }
        num = num.checked_add(smaller.checked_mul(f)?)?;
    }
    Some(num)
}

/// Integer complexity of `n` (minimum number of 1s with `+` and `*`).
///
/// Port of `integer_complexity` in `intcomplexity.c` (Fuller's algorithm,
/// with a process-wide cache). `n = 0` gives `None`.
///
/// # Panics
///
/// Panics when `n` is `u64::MAX` or `n > 50_000_000`.
///
/// ```
/// use math_prime_util::integer_complexity;
/// assert_eq!(integer_complexity(0), None);
/// assert_eq!(integer_complexity(1), Some(1));
/// assert_eq!(integer_complexity(10), Some(7));
/// ```
#[must_use]
pub fn integer_complexity(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n == u64::MAX || n > 50_000_000 {
        panic!("integer_complexity: input too large");
    }
    let cache = ic_cache();
    let mut guard = cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if guard.len() <= n as usize {
        expand_ic(&mut guard, n);
    }
    Some(u64::from(guard[n as usize]))
}

fn ic_cache() -> &'static Mutex<Vec<u16>> {
    static CACHE: OnceLock<Mutex<Vec<u16>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(Vec::new()))
}

fn calc_g(mut n: u32) -> u64 {
    if n >= 122 {
        return u64::MAX;
    }
    let mut res = 1u64;
    while n >= 5 || n == 3 {
        res = res.saturating_mul(3);
        n -= 3;
    }
    res << (n / 2)
}

fn expand_ic(a: &mut Vec<u16>, n: u64) {
    let to = (n + 32_767) as usize;
    let cur = a.len().saturating_sub(1);
    if !a.is_empty() && cur >= to {
        return;
    }
    let prev = if a.is_empty() { 0 } else { a.len() - 1 };
    a.resize(to + 1, 127);
    let first = if prev < 2 {
        a[0] = 0;
        a[1] = 1;
        2usize
    } else {
        prev + 1
    };
    let mut g = [0u64; 128];
    for i in 0..128 {
        g[i] = calc_g(u32::try_from(i).unwrap_or(0));
    }
    let mut i = 2usize;
    while i < first && i <= to / 2 {
        let mut j = 2usize;
        let mut ij = i.saturating_mul(2);
        while j <= i && ij <= to {
            if ij >= first {
                let s = a[i] + a[j];
                if s < a[ij] {
                    a[ij] = s;
                }
            }
            j += 1;
            ij += i;
        }
        i += 1;
    }
    for i in first..=to {
        let t = u32::from(a[i - 1]);
        if t + 1 < u32::from(a[i]) {
            a[i] = u16::try_from(t + 1).unwrap_or(u16::MAX);
        }
        let t_us = t as usize;
        let mut k = 1usize;
        while k < t_us / 2 && g[k + 1] + g[t_us - k - 1] >= i as u64 {
            k += 1;
            if k + 1 >= g.len() || t_us < k + 1 {
                break;
            }
        }
        let gk = g[k].min(i as u64);
        let mut j = 6u64;
        while j <= gk {
            let ju = j as usize;
            let s = a[ju] + a[i - ju];
            if s < a[i] {
                a[i] = s;
            }
            j += 1;
        }
        if i <= to / 2 {
            let mut j = 2usize;
            let mut ij = i.saturating_mul(2);
            while j <= i && ij <= to {
                let s = a[i] + a[j];
                if s < a[ij] {
                    a[ij] = s;
                }
                j += 1;
                ij += i;
            }
        }
    }
}
