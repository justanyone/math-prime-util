//! Divisor sums, discriminants, and arithmetic in residue rings.
use crate::{gcd, invmod, is_power, is_prime, is_square, next_prime, powmod};

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43] {
        while n > 1 && n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    let mut pending = vec![n];
    while let Some(n) = pending.pop() {
        if n < 2 {
            continue;
        }
        if is_prime(n) != 0 {
            factors.push(n);
            continue;
        }
        let mut a = 1;
        let divisor = loop {
            if let Some(d) = brent(n, a) {
                break d;
            }
            a += 1;
        };
        pending.push(divisor);
        pending.push(n / divisor);
    }
    factors.sort_unstable();
    factors
}

#[allow(clippy::cast_possible_truncation)] // Remainders modulo u64 fit u64.
fn brent(n: u64, mut a: u64) -> Option<u64> {
    let step =
        |x: u64, a: u64| ((u128::from(x) * u128::from(x) + u128::from(a)) % u128::from(n)) as u64;
    let inner = if n <= 4_000_000_000 { 32 } else { 160 };
    let (mut r, mut xi, mut xm) = (1_u64, 1, 1);
    let mut rounds = 500_000;
    let mut fails = 6;
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = xi;
        let mut f = 1;
        while left > 0 {
            let count = left.min(inner);
            saved = xi;
            left -= count;
            rounds -= count;
            xi = step(xi, a);
            let mut m = xi.abs_diff(xm);
            for _ in 1..count {
                xi = step(xi, a);
                m = (u128::from(m) * u128::from(xi.abs_diff(xm)) % u128::from(n)) as u64;
            }
            f = gcd(m, n);
            if f != 1 {
                break;
            }
        }
        if f == 1 {
            r *= 2;
            xm = xi;
            continue;
        }
        if f == n {
            xi = saved;
            loop {
                xi = step(xi, a);
                f = gcd(xi.abs_diff(xm), n);
                if f != 1 || r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        if f == n || f == 0 {
            if fails == 0 {
                break;
            }
            fails -= 1;
            xm = ((u128::from(xm) + 11) % u128::from(n)) as u64;
            xi = xm;
            a += 1;
        } else {
            return Some(f);
        }
    }
    None
}

fn factor_powers(n: u64) -> Vec<(u64, u32)> {
    let mut result: Vec<(u64, u32)> = Vec::new();
    for p in prime_factors(n) {
        if let Some(last) = result.last_mut() {
            if last.0 == p {
                last.1 += 1;
                continue;
            }
        }
        result.push((p, 1));
    }
    result
}
fn sigma(n: u64, k: u64) -> Option<u128> {
    if n <= 1 {
        return Some(u128::from(n));
    }
    let mut product = 1_u128;
    for (p, e) in factor_powers(n) {
        if k == 0 {
            product *= u128::from(e + 1);
            continue;
        }
        let pk = u128::from(p).checked_pow(u32::try_from(k).ok()?)?;
        let (mut power, mut sum) = (1_u128, 1_u128);
        for _ in 0..e {
            power = power.checked_mul(pk)?;
            sum = sum.checked_add(power)?;
        }
        product = product.checked_mul(sum)?;
    }
    Some(product)
}

/// Return the sum of the k-th powers of the divisors of n.
/// Return `None` if the result exceeds u64.
///
/// Port of MPU 0.75 `factor.c::divisor_sum`.
///
/// ```
/// assert_eq!(math_prime_util::divisor_sum(12, 1), Some(28));
/// ```
pub fn divisor_sum(n: u64, k: u64) -> Option<u64> {
    u64::try_from(sigma(n, k)?).ok()
}

const TAU: &[i64] = &[
    0,
    1,
    -24,
    252,
    -1472,
    4830,
    -6048,
    -16_744,
    84_480,
    -113_643,
    -115_920,
    534_612,
    -370_944,
    -577_738,
    401_856,
    1_217_160,
    987_136,
    -6_905_934,
    2_727_432,
    10_661_420,
    -7_109_760,
    -4_219_488,
    -12_830_688,
    18_643_272,
    21_288_960,
    -25_499_225,
    13_865_712,
    -73_279_080,
    24_647_168,
    128_406_630,
    -29_211_840,
    -52_843_168,
    -196_706_304,
    134_722_224,
    165_742_416,
    -80_873_520,
    167_282_496,
    -182_213_314,
    -255_874_080,
    -145_589_976,
    408_038_400,
    308_120_442,
    101_267_712,
    -17_125_708,
    -786_948_864,
    -548_895_690,
    -447_438_528,
];

/// Return the native Ramanujan tau value.
/// Return `None` if the native calculation overflows.
///
/// Port of MPU 0.75 `util.c::ramanujan_tau` and `PP.pm::_tauprime`, `_taupower`.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_tau(2), Some(-24));
/// ```
pub fn ramanujan_tau(n: u64) -> Option<i64> {
    if let Some(&value) = TAU.get(usize::try_from(n).ok()?) {
        return Some(value);
    }
    let mut result = 1_i128;
    for (p, e) in factor_powers(n) {
        let tp = tau_prime(p)?;
        let p11 = i128::from(p).checked_pow(11)?;
        let (mut previous, mut current) = (1_i128, tp);
        for _ in 1..e {
            let next = tp
                .checked_mul(current)?
                .checked_sub(p11.checked_mul(previous)?)?;
            (previous, current) = (current, next);
        }
        result = result.checked_mul(current)?;
    }
    i64::try_from(result).ok()
}

// MPU PP.pm::_tauprime and _taupower supply values beyond util.c's table.
fn tau_prime(p: u64) -> Option<i128> {
    if let Some(&value) = TAU.get(usize::try_from(p).ok()?) {
        return Some(i128::from(value));
    }
    let x = i128::from(p);
    let leading = 28_i128
        .checked_mul(x.checked_pow(6)?)?
        .checked_sub(28_i128.checked_mul(x.checked_pow(5)?)?)?
        .checked_sub(90_i128.checked_mul(x.checked_pow(4)?)?)?
        .checked_sub(35_i128.checked_mul(x.checked_pow(3)?)?)?
        .checked_sub(1)?;
    let mut sum = 0_i128;
    for t in 1..=crate::sqrtint(p) {
        let t2 = t * t;
        let v = p - t2;
        let h = match v % 4 {
            0 => 2 * class_number(v) + class_number(4 * v),
            3 => (if v % 8 == 3 { 6 } else { 4 }) * class_number(v),
            _ => class_number(4 * v),
        };
        let t2 = i128::from(t2);
        let term = t2
            .checked_pow(3)?
            .checked_mul(4 * t2 * t2 - 9 * x * t2 + 7 * x * x)?
            .checked_mul(h)?;
        sum = sum.checked_add(term)?;
    }
    leading.checked_sub(32_i128.checked_mul(sum / 3)?)
}
// util.c::hclassno and _count_class_div, in the bounded tau-prime path.
fn class_number(n: u64) -> i128 {
    if n == 0 {
        return -1;
    }
    if matches!(n % 4, 1 | 2) {
        return 0;
    }
    if n == 3 {
        return 4;
    }
    let mut b = n & 1;
    let mut b2 = (n + 1) >> 2;
    let square = is_square(b2);
    let mut h = divisor_sum(b2, 0).unwrap_or(0) >> 1;
    if b == 1 {
        h = 1 + u64::from(square) + 2 * (h - 1);
    }
    b += 2;
    loop {
        b2 = (n + b * b) >> 2;
        if 3 * b2 >= n {
            break;
        }
        let limit = crate::sqrtint(b2) - u64::from(is_square(b2));
        let divisors = (b + 1..=limit).filter(|d| b2 % d == 0).count();
        h += u64::from(b2 % b == 0)
            + u64::from(is_square(b2))
            + 2 * u64::try_from(divisors).unwrap_or(0);
        b += 2;
    }
    let correction = if b2 * 3 == n {
        4
    } else if square && n % 2 == 0 {
        6
    } else {
        0
    };
    i128::from(12 * h + correction)
}

/// Return the sum of the proper divisors of n.
/// Return `None` if the result exceeds u64.
///
/// Port of MPU 0.75 `factor.c::aliquot_sum`.
///
/// ```
/// assert_eq!(math_prime_util::aliquot_sum(12), Some(16));
/// ```
pub fn aliquot_sum(n: u64) -> Option<u64> {
    if n <= 1 {
        return Some(0);
    }
    u64::try_from(sigma(n, 1)? - u128::from(n)).ok()
}

/// Return the aliquot sum minus n as a native signed word.
/// MPU converts the difference to i64 with wraparound.
/// Return `None` if the aliquot sum exceeds u64.
///
/// Port of MPU 0.75 `XS.xs::abundance` and `factor.c::aliquot_sum`.
///
/// ```
/// assert_eq!(math_prime_util::abundance(12), Some(4));
/// ```
pub fn abundance(n: u64) -> Option<i64> {
    let sum = aliquot_sum(n)?;
    Some(i64::from_ne_bytes(sum.wrapping_sub(n).to_ne_bytes()))
}

/// Return the sum of prime factors with repetition.
///
/// Port of MPU 0.75 `factor.c::sopfr`.
///
/// ```
/// assert_eq!(math_prime_util::sopfr(12), 7);
/// ```
pub fn sopfr(n: u64) -> u64 {
    prime_factors(n).iter().sum()
}

/// Return the sum of distinct prime factors.
///
/// Port of MPU 0.75 `factor.c::sopf`.
///
/// ```
/// assert_eq!(math_prime_util::sopf(12), 5);
/// ```
pub fn sopf(n: u64) -> u64 {
    factor_powers(n).iter().map(|&(p, _)| p).sum()
}

fn kronecker_unsigned(a: u64, mut b: u64) -> i32 {
    if b == 0 {
        return i32::from(a == 1);
    }
    let mut s = 1;
    let r = b.trailing_zeros();
    if r > 0 {
        if a % 2 == 0 {
            return 0;
        }
        if r % 2 == 1 && matches!(a % 8, 3 | 5) {
            s = -s;
        }
        b >>= r;
    }
    kronecker_odd(a, b, s)
}
fn kronecker_odd(mut a: u64, mut b: u64, mut s: i32) -> i32 {
    while a != 0 {
        let r = a.trailing_zeros();
        if r % 2 == 1 && matches!(b % 8, 3 | 5) {
            s = -s;
        }
        a >>= r;
        if a & b & 2 != 0 {
            s = -s;
        }
        (a, b) = (b % a, a);
    }
    if b == 1 { s } else { 0 }
}

/// Return the signed Kronecker symbol (a/b).
///
/// Port of MPU 0.75 `util.c::kronecker_ss, kronecker_su, kronecker_uu_sign`.
///
/// ```
/// assert_eq!(math_prime_util::kronecker(-1, 3), -1);
/// ```
pub fn kronecker(a: i64, b: i64) -> i32 {
    let mut s = if a < 0 && b < 0 { -1 } else { 1 };
    let mut b = b.unsigned_abs();
    if a >= 0 {
        return s * kronecker_unsigned(a.unsigned_abs(), b);
    }
    if b == 0 {
        return i32::from(a == -1);
    }
    let r = b.trailing_zeros();
    if r > 0 {
        if a % 2 == 0 {
            return 0;
        }
        if r % 2 == 1 && matches!(a.rem_euclid(8), 3 | 5) {
            s = -s;
        }
        b >>= r;
    }
    let rem = a.unsigned_abs() % b;
    kronecker_odd(if rem == 0 { 0 } else { b - rem }, b, s)
}

/// Test whether n is a fundamental discriminant.
///
/// Port of MPU 0.75 `util.c::is_fundamental`.
///
/// ```
/// assert!(math_prime_util::is_fundamental(-4));
/// ```
pub fn is_fundamental(n: i64) -> bool {
    let r = n.rem_euclid(16);
    let m = n.unsigned_abs();
    let square_free = |x| x != 0 && factor_powers(x).iter().all(|&(_, e)| e == 1);
    if r != 0 && r % 4 == 0 && r != 4 {
        square_free(m >> 2)
    } else {
        r % 4 == 1 && square_free(m)
    }
}

/// Solve simultaneous congruences. Each pair is (residue, modulus).
/// Return `None` for a zero modulus, inconsistent inputs, or modulus overflow.
///
/// Port of MPU 0.75 `util.c::chinese and _simple_chinese`.
///
/// ```
/// assert_eq!(math_prime_util::chinese(&[(2, 3), (3, 5)]), Some(8));
/// ```
pub fn chinese(congruences: &[(u64, u64)]) -> Option<u64> {
    chinese2(congruences).map(|(r, _)| r)
}

/// Return the least solution and the combined modulus.
/// Return `None` for a zero modulus, inconsistent inputs, or modulus overflow.
/// An empty input gives `(0, 0)`.
///
/// Port of MPU 0.75 `util.c::chinese and _simple_chinese`.
///
/// ```
/// assert_eq!(math_prime_util::chinese2(&[(2, 3), (3, 5)]), Some((8, 15)));
/// ```
pub fn chinese2(congruences: &[(u64, u64)]) -> Option<(u64, u64)> {
    if congruences.is_empty() {
        return Some((0, 0));
    }
    let mut pairs = congruences.to_vec();
    pairs.sort_unstable_by_key(|a| std::cmp::Reverse(a.1));
    let (mut r, mut m) = pairs[0];
    if m == 0 {
        return None;
    }
    r %= m;
    for &(a, n) in &pairs[1..] {
        if n == 0 {
            return None;
        }
        let g = gcd(m, n);
        if r % g != a % g {
            return None;
        }
        let ng = n / g;
        let modulus = m.checked_mul(ng)?;
        let delta = (i128::from(a) - i128::from(r)) / i128::from(g);
        let delta = u64::try_from(delta.rem_euclid(i128::from(ng))).ok()?;
        let t = crate::mulmod(delta, invmod(m / g, ng)?, ng)?;
        r = u64::try_from((u128::from(r) + u128::from(m) * u128::from(t)) % u128::from(modulus))
            .ok()?;
        m = modulus;
    }
    Some((r, m))
}

fn lambda(n: u64) -> u64 {
    let mut result = 1;
    for (p, e) in factor_powers(n) {
        let term = if p == 2 && e > 2 {
            1 << (e - 2)
        } else {
            (p - 1) * p.pow(e - 1)
        };
        result = result / gcd(result, term) * term;
    }
    result
}

/// Return the multiplicative order of a modulo n.
/// Return `None` if n is zero or a is not a unit modulo n.
///
/// Port of MPU 0.75 `util.c::znorder`.
///
/// ```
/// assert_eq!(math_prime_util::znorder(2, 7), Some(3));
/// ```
pub fn znorder(a: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(1);
    }
    if gcd(a, n) != 1 {
        return None;
    }
    let mut k = lambda(n);
    for (p, e) in factor_powers(k) {
        k /= p.pow(e);
        let mut a1 = powmod(a, k, n)?;
        while a1 != 1 {
            k *= p;
            a1 = powmod(a1, p, n)?;
        }
    }
    Some(k)
}

/// Return the least primitive root modulo n.
/// Return `None` if n is zero or no root exists.
///
/// Port of MPU 0.75 `util.c::znprimroot`.
///
/// ```
/// assert_eq!(math_prime_util::znprimroot(7), Some(3));
/// ```
pub fn znprimroot(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 4 {
        return Some(n - 1);
    }
    if n % 4 == 0 {
        return None;
    }
    let even = n % 2 == 0;
    let odd = if even { n / 2 } else { n };
    let factors = factor_powers(odd);
    if factors.len() != 1 {
        return None;
    }
    let (p, e) = factors[0];
    if p == 3 && even {
        return Some(5);
    }
    let phi = p - 1;
    let phif = factor_powers(phi);
    for a in 2..p {
        if (even && a % 2 == 0) || matches!(a, 4 | 8 | 9) {
            continue;
        }
        if kronecker_unsigned(a, p) != -1 {
            continue;
        }
        if phif
            .iter()
            .skip(1)
            .any(|&(q, _)| powmod(a, phi / q, p) == Some(1))
        {
            continue;
        }
        if e == 1 || powmod(a, phi, p * p) != Some(1) {
            return Some(a);
        }
    }
    None
}

/// Test whether a is a primitive root modulo n.
/// Return `None` if n is zero.
///
/// Port of MPU 0.75 `util.c::is_primitive_root`.
///
/// ```
/// assert_eq!(math_prime_util::is_primitive_root(3, 7), Some(true));
/// ```
pub fn is_primitive_root(a: u64, n: u64) -> Option<bool> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(true);
    }
    let a = a % n;
    if a <= 1 {
        return Some(a == 1 && n <= 2);
    }
    if n <= 4 {
        return Some(a == n - 1);
    }
    if n % 4 == 0 || (n % 2 == 0 && a % 2 == 0) || is_square(a) || gcd(a, n) != 1 {
        return Some(false);
    }
    let odd = if n % 2 == 0 { n / 2 } else { n };
    let f = factor_powers(odd);
    if f.len() != 1 {
        return Some(false);
    }
    let (p, e) = f[0];
    if e > 1 && powmod(a, p - 1, p * p) == Some(1) {
        return Some(false);
    }
    if kronecker_unsigned(a, p) != -1 {
        return Some(false);
    }
    let phi = p - 1;
    if is_power(a, 3) && gcd(3, phi) != 1 {
        return Some(false);
    }
    Some(
        factor_powers(phi)
            .iter()
            .all(|&(q, _)| powmod(a, phi / q, p) != Some(1)),
    )
}

/// Return the least quadratic nonresidue modulo n.
/// Return `None` if n is zero or no nonresidue exists.
///
/// Port of MPU 0.75 `util.c::qnr`.
///
/// ```
/// assert_eq!(math_prime_util::qnr(7), Some(3));
/// ```
pub fn qnr(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 2 {
        return Some(n);
    }
    if kronecker_unsigned(2, n) == -1 {
        return Some(2);
    }
    if is_prime(n) != 0 {
        return (3..n).step_by(2).find(|&a| kronecker_unsigned(a, n) == -1);
    }
    let mut n = n;
    if n % 2 == 0 {
        let e = n.trailing_zeros();
        n >>= e;
        if e >= 2 || n == 1 {
            return Some(2);
        }
    }
    if [3, 5, 11, 13, 19].iter().any(|p| n % p == 0) {
        return Some(2);
    }
    let factors = factor_powers(n);
    let mut a = 2;
    while a < n {
        if factors
            .iter()
            .any(|&(p, _)| a < p && kronecker_unsigned(a, p) == -1)
        {
            return Some(a);
        }
        a = next_prime(a)?;
    }
    None
}

/// Test whether a is a square modulo n.
/// Return false for modulus zero, as in the MPU wrapper.
///
/// Port of MPU 0.75 `util.c::is_qr and rootmod.c::_sqrtmod_composite`.
///
/// ```
/// assert!(math_prime_util::is_qr(2, 7));
/// ```
pub fn is_qr(a: u64, n: u64) -> bool {
    if n == 0 {
        return false;
    }
    if n <= 2 || a % n <= 1 {
        return true;
    }
    let a = a % n;
    factor_powers(n).iter().all(|&(p, e)| {
        let mut unit = a;
        let mut v = 0;
        while unit % p == 0 && v < e {
            unit /= p;
            v += 1;
        }
        if v == e {
            return true;
        }
        if v % 2 == 1 {
            return false;
        }
        if p != 2 {
            return kronecker_unsigned(unit, p) == 1;
        }
        match e - v {
            1 => true,
            2 => unit % 4 == 1,
            _ => unit % 8 == 1,
        }
    })
}

mod inverse_sigma0;
pub use inverse_sigma0::*;
