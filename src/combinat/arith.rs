//! Binomial coefficients, factorials, Stirling numbers, and partitions.
//! Ports of the matching functions in `reference/mpu/util.c`.

use std::sync::OnceLock;

/// Greatest common divisor of two `u128` values.
pub(crate) fn gcd_u128(mut x: u128, mut y: u128) -> u128 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

/// `base.pow(exp)` in `u128`, or `None` on overflow.
fn pow_u128(base: u64, mut exp: u64) -> Option<u128> {
    let mut b = u128::from(base);
    let mut r = 1u128;
    if exp == 0 {
        return Some(1);
    }
    while exp > 0 {
        if exp & 1 == 1 {
            r = r.checked_mul(b)?;
        }
        exp >>= 1;
        if exp > 0 {
            b = b.checked_mul(b)?;
        }
    }
    Some(r)
}

/// Multiply `terms` then divide by `div`. The division is exact.
fn product_div(terms: &[u128], mut div: u128) -> Option<u128> {
    let mut acc = 1u128;
    for &term in terms {
        let mut term = term;
        let g = gcd_u128(term, div);
        term /= g;
        div /= g;
        let g = gcd_u128(acc, div);
        acc /= g;
        div /= g;
        acc = acc.checked_mul(term)?;
    }
    if div == 1 {
        Some(acc)
    } else if acc % div == 0 {
        Some(acc / div)
    } else {
        None
    }
}

/// `n!` while it fits in `u128` (`n <= 34`).
pub(crate) fn factorial_u128(n: u64) -> Option<u128> {
    let mut r = 1u128;
    for i in 2..=n {
        r = r.checked_mul(u128::from(i))?;
    }
    Some(r)
}

/// Binomial coefficient `C(n, k)` as a full `u128`, or `None` on overflow.
pub(crate) fn binomial_mag(mut n: u128, mut k: u128) -> Option<u128> {
    if k == 0 || k == n {
        return Some(1);
    }
    if k > n {
        return Some(0);
    }
    if k > n - k {
        k = n - k;
    }
    let mut r = 1u128;
    for d in 1..=k {
        let g1 = gcd_u128(n, d);
        let nr = n / g1;
        let mut dr = d / g1;
        let g2 = gcd_u128(r, dr);
        r /= g2;
        dr /= g2;
        r = r.checked_mul(nr)? / dr;
        n -= 1;
    }
    Some(r)
}

/// Finish a binomial magnitude with the sign `(-1)^exp` when `negative`.
fn binomial_finish(mag: u128, negative: bool) -> Option<i128> {
    if negative {
        i128::try_from(mag).ok()?.checked_neg()
    } else if mag > u128::from(u64::MAX) {
        None
    } else {
        Some(mag as i128)
    }
}

/// Binomial coefficient `C(n, k)`.
///
/// Port of `binomial` in `util.c`. A non-negative value that does not fit in
/// `u64` is `None`. A negative upper index uses
/// `C(-n, k) = (-1)^k C(n + k - 1, k)`. A negative lower index uses
/// `C(-a, -b) = (-1)^{a - b} C(b - 1, a - 1)` (and 0 when the upper index is
/// non-negative). `None` when a negative value does not fit in `i128`.
///
/// ```
/// use math_prime_util::binomial;
/// assert_eq!(binomial(10, 3), Some(120));
/// assert_eq!(binomial(5, 8), Some(0));
/// assert_eq!(binomial(-4, 2), Some(10));
/// assert_eq!(binomial(-9, -10), Some(-9));
/// ```
#[must_use]
pub fn binomial(n: i128, k: i128) -> Option<i128> {
    if k < 0 {
        if n >= 0 {
            return Some(0);
        }
        let top = k.checked_neg()?.checked_sub(1)?;
        let choose = n.checked_neg()?.checked_sub(1)?;
        if choose < 0 || top < choose {
            return Some(0);
        }
        let mag = binomial_mag(u128::try_from(top).ok()?, u128::try_from(choose).ok()?)?;
        let negative = n.checked_sub(k)?.rem_euclid(2) == 1;
        return binomial_finish(mag, negative);
    }
    let k = u128::try_from(k).ok()?;
    if k == 0 {
        return Some(1);
    }
    if n >= 0 {
        let mag = binomial_mag(u128::try_from(n).ok()?, k)?;
        return binomial_finish(mag, false);
    }
    // C(-m, k) = (-1)^k * C(m + k - 1, k).
    let m = u128::try_from(n.checked_neg()?).ok()?;
    let top = m.checked_add(k)?.checked_sub(1)?;
    let mag = binomial_mag(top, k)?;
    binomial_finish(mag, k % 2 == 1)
}

/// Catalan number `C(2n, n) / (n + 1)`.
///
/// Port of `catalan_number` in `util.c`. `None` when the value does not fit
/// in `u128`.
///
/// ```
/// use math_prime_util::catalan_number;
/// assert_eq!(catalan_number(4), Some(14));
/// assert_eq!(catalan_number(0), Some(1));
/// ```
#[must_use]
pub fn catalan_number(n: u64) -> Option<u128> {
    if n == 0 {
        return Some(1);
    }
    let bin = binomial_mag(u128::from(n) * 2, u128::from(n))?;
    Some(bin / u128::from(n + 1))
}

/// One row of Stirling numbers of the second kind, `S(n, 0..=n)`.
fn stirling2_row(n: u64) -> Option<Vec<u128>> {
    if n > 4096 {
        return None;
    }
    let mut prev = vec![1u128];
    for i in 1..=n {
        let mut row = vec![0u128; (i as usize) + 1];
        for k in 1..=i {
            let left = if k == i { 0 } else { prev[k as usize] };
            let right = prev[(k as usize) - 1];
            row[k as usize] = u128::from(k).checked_mul(left)?.checked_add(right)?;
        }
        prev = row;
    }
    Some(prev)
}

/// Signed Stirling numbers of the first kind, one row `s(n, 0..=n)`.
fn stirling1_row(n: u64) -> Option<Vec<i128>> {
    if n > 4096 {
        return None;
    }
    let mut prev = vec![1i128];
    for i in 1..=n {
        let mut row = vec![0i128; (i as usize) + 1];
        let coef = i128::from(i - 1);
        for k in 1..=i {
            let right = if k == i { 0 } else { prev[k as usize] };
            let left = prev[(k as usize) - 1];
            let prod = coef.checked_mul(right)?;
            row[k as usize] = left.checked_sub(prod)?;
        }
        prev = row;
    }
    Some(prev)
}

/// Lah number `L(n, k)`.
fn lah(n: u64, k: u64) -> Option<i128> {
    if k == n {
        return Some(1);
    }
    if k == 0 || k > n {
        return Some(0);
    }
    let c1 = binomial_mag(u128::from(n), u128::from(k))?;
    let c2 = binomial_mag(u128::from(n - 1), u128::from(k - 1))?;
    let f = factorial_u128(n - k)?;
    let v = c1.checked_mul(c2)?.checked_mul(f)?;
    i128::try_from(v).ok()
}

/// Stirling number of type 1 (signed first kind), 2 (second kind), or 3 (Lah).
///
/// Port of `stirling1`, `stirling2`, and `stirling3` in `util.c`.
/// `None` when the value does not fit in `i128`.
///
/// # Panics
///
/// Panics when `typ` is not 1, 2, or 3.
///
/// ```
/// use math_prime_util::stirling;
/// assert_eq!(stirling(4, 1, 1), Some(-6));
/// assert_eq!(stirling(5, 2, 2), Some(15));
/// assert_eq!(stirling(4, 2, 3), Some(36));
/// ```
#[must_use]
pub fn stirling(n: u64, m: u64, typ: u32) -> Option<i128> {
    if typ != 1 && typ != 2 && typ != 3 {
        panic!("stirling: type must be 1, 2, or 3");
    }
    if m > n && !(n == 0 && m == 0) {
        return Some(0);
    }
    match typ {
        1 => {
            let row = stirling1_row(n)?;
            row.get(m as usize).copied()
        }
        2 => {
            let row = stirling2_row(n)?;
            let v = *row.get(m as usize)?;
            i128::try_from(v).ok()
        }
        _ => lah(n, m),
    }
}

/// Bell number (sum of Stirling numbers of the second kind).
///
/// Port of `bell_number` in `util.c`. `None` when the value does not fit in
/// `u128`.
///
/// ```
/// use math_prime_util::bell_number;
/// assert_eq!(bell_number(5), Some(52));
/// assert_eq!(bell_number(0), Some(1));
/// ```
#[must_use]
pub fn bell_number(n: u64) -> Option<u128> {
    let row = stirling2_row(n)?;
    let mut sum = 0u128;
    for v in row {
        sum = sum.checked_add(v)?;
    }
    Some(sum)
}

/// Fubini number (ordered Bell number).
///
/// Port of `fubini` in `util.c`. `None` when the value does not fit in `u128`.
///
/// ```
/// use math_prime_util::fubini;
/// assert_eq!(fubini(0), Some(1));
/// assert_eq!(fubini(3), Some(13));
/// ```
#[must_use]
pub fn fubini(n: u64) -> Option<u128> {
    if n == 0 {
        return Some(1);
    }
    let row = stirling2_row(n)?;
    let mut sum = 0u128;
    let mut fac = 1u128;
    for k in 0..=n {
        let term = fac.checked_mul(*row.get(k as usize)?)?;
        sum = sum.checked_add(term)?;
        if k < n {
            fac = fac.checked_mul(u128::from(k + 1))?;
        }
    }
    Some(sum)
}

/// Factorial `n!`.
///
/// Port of `factorial` in `util.c`. `None` when `n!` does not fit in `u128`.
///
/// # Panics
///
/// Panics when `n` is negative.
///
/// ```
/// use math_prime_util::factorial;
/// assert_eq!(factorial(5), Some(120));
/// assert_eq!(factorial(0), Some(1));
/// ```
#[must_use]
pub fn factorial(n: i64) -> Option<u128> {
    if n < 0 {
        panic!("factorial: negative argument");
    }
    factorial_u128(u64::try_from(n).expect("n >= 0"))
}

/// Multifactorial: `n * (n - k) * (n - 2k) * ...` down to a positive term.
///
/// Port of `multifactorial` in `util.c`. `None` when the product does not fit
/// in `u128`.
///
/// # Panics
///
/// Panics when `k` is 0 or when `n` is negative.
///
/// ```
/// use math_prime_util::multifactorial;
/// assert_eq!(multifactorial(7, 2), Some(105));
/// assert_eq!(multifactorial(5, 10), Some(5));
/// ```
#[must_use]
pub fn multifactorial(n: i128, k: u128) -> Option<u128> {
    if k == 0 {
        panic!("multifactorial: k must be positive");
    }
    if n < 0 {
        panic!("multifactorial: negative argument");
    }
    if n == 0 {
        return Some(1);
    }
    let n = u128::try_from(n).ok()?;
    let mut r = 1u128;
    let mut i = n;
    loop {
        r = r.checked_mul(i)?;
        if i <= k {
            break;
        }
        i -= k;
    }
    Some(r)
}

/// Subfactorial `!n` (derangement count).
///
/// Port of `subfactorial` in `util.c`. `None` when the value does not fit in
/// `u128`.
///
/// # Panics
///
/// Panics when `n` is negative.
///
/// ```
/// use math_prime_util::subfactorial;
/// assert_eq!(subfactorial(4), Some(9));
/// assert_eq!(subfactorial(0), Some(1));
/// ```
#[must_use]
pub fn subfactorial(n: i64) -> Option<u128> {
    if n < 0 {
        panic!("subfactorial: negative argument");
    }
    let n = u64::try_from(n).expect("n >= 0");
    subfactorial_rec(n)
}

/// `!n = n * !(n - 1) + (-1)^n`, with `!0 = 1` and `!1 = 0`.
fn subfactorial_rec(n: u64) -> Option<u128> {
    let mut b = 0u128; // !1
    if n == 0 {
        return Some(1);
    }
    if n == 1 {
        return Some(0);
    }
    for i in 2..=n {
        let mut t = b.checked_mul(u128::from(i))?;
        if i % 2 == 0 {
            t = t.checked_add(1)?;
        } else {
            t = t.checked_sub(1)?;
        }
        b = t;
    }
    Some(b)
}

/// Falling factorial `n (n - 1) ... (n - m + 1)`.
///
/// Port of `falling_factorial` and `falling_factorial_s` in `util.c`.
/// `m = 0` gives 1. `None` when the product does not fit in `i128`.
///
/// ```
/// use math_prime_util::falling_factorial;
/// assert_eq!(falling_factorial(7, 3), Some(210));
/// assert_eq!(falling_factorial(-10, 1), Some(-10));
/// ```
#[must_use]
pub fn falling_factorial(n: i128, m: u64) -> Option<i128> {
    if m == 0 {
        return Some(1);
    }
    if m > 200 {
        let last = n.checked_sub(i128::try_from(m - 1).ok()?)?;
        let (lo, hi) = if n <= last { (n, last) } else { (last, n) };
        if lo <= 0 && hi >= 0 {
            return Some(0);
        }
        return None;
    }
    let mut r = 1i128;
    let mut x = n;
    for _ in 0..m {
        r = r.checked_mul(x)?;
        x = x.checked_sub(1)?;
    }
    Some(r)
}

/// Rising factorial `n (n + 1) ... (n + m - 1)`.
///
/// Port of `rising_factorial` and `rising_factorial_s` in `util.c`.
/// `m = 0` gives 1. `None` when the product does not fit in `i128`.
///
/// ```
/// use math_prime_util::rising_factorial;
/// assert_eq!(rising_factorial(7, 3), Some(504));
/// assert_eq!(rising_factorial(5, 0), Some(1));
/// ```
#[must_use]
pub fn rising_factorial(n: i128, m: u64) -> Option<i128> {
    if m == 0 {
        return Some(1);
    }
    if m > 200 {
        let last = n.checked_add(i128::try_from(m - 1).ok()?)?;
        let (lo, hi) = if n <= last { (n, last) } else { (last, n) };
        if lo <= 0 && hi >= 0 {
            return Some(0);
        }
        return None;
    }
    let mut r = 1i128;
    let mut x = n;
    for _ in 0..m {
        r = r.checked_mul(x)?;
        x = x.checked_add(1)?;
    }
    Some(r)
}

/// Power sum `1^k + 2^k + ... + n^k`. `k = 0` returns `n`.
///
/// Port of `powersum` in `util.c`. `None` when the sum does not fit in `u128`.
///
/// ```
/// use math_prime_util::powersum;
/// assert_eq!(powersum(5, 2), Some(55));
/// assert_eq!(powersum(5, 0), Some(5));
/// ```
#[must_use]
pub fn powersum(n: u128, k: u64) -> Option<u128> {
    if k == 0 || n <= 1 {
        return Some(n);
    }
    if n > u128::from(u64::MAX) {
        if k == 1 {
            return n.checked_add(1).and_then(|m| n.checked_mul(m)).map(|p| p / 2);
        }
        return None;
    }
    let n64 = n as u64;
    let nu = n;
    match k {
        1 => Some(nu * (nu + 1) / 2),
        2 => product_div(&[nu, nu + 1, 2 * nu + 1], 6),
        3 => {
            let a = nu * (nu + 1) / 2;
            a.checked_mul(a)
        }
        4 => {
            let poly = 3 * nu * nu + 3 * nu - 1;
            product_div(&[nu, nu + 1, 2 * nu + 1, poly], 30)
        }
        5 => {
            let poly = 2 * nu * nu + 2 * nu - 1;
            product_div(&[nu, nu, nu + 1, nu + 1, poly], 12)
        }
        _ => {
            if pow_u128(n64, k).is_none() {
                return None;
            }
            if n64 > 20_000_000 {
                return None;
            }
            let mut sum = 0u128;
            for i in 1..=n64 {
                sum = sum.checked_add(pow_u128(i, k)?)?;
            }
            Some(sum)
        }
    }
}

/// Partition number `p(n)`.
///
/// Port of `npartitions` in `util.c` (pentagonal-number recurrence).
/// `None` when the value does not fit in `u128`.
///
/// # Panics
///
/// Panics when `n` is negative.
///
/// ```
/// use math_prime_util::partitions;
/// assert_eq!(partitions(5), Some(7));
/// assert_eq!(partitions(0), Some(1));
/// ```
#[must_use]
pub fn partitions(n: i64) -> Option<u128> {
    if n < 0 {
        panic!("partitions: negative argument");
    }
    partition_count(u64::try_from(n).expect("n >= 0"), false)
}

/// Number of partitions into distinct parts.
///
/// Port of `npartitionsq` in `util.c`. `None` when the value does not fit in
/// `u128`.
///
/// # Panics
///
/// Panics when `n` is negative.
///
/// ```
/// use math_prime_util::partitionsq;
/// assert_eq!(partitionsq(5), Some(3));
/// assert_eq!(partitionsq(0), Some(1));
/// ```
#[must_use]
pub fn partitionsq(n: i64) -> Option<u128> {
    if n < 0 {
        panic!("partitionsq: negative argument");
    }
    partition_count(u64::try_from(n).expect("n >= 0"), true)
}

fn partition_count(n: u64, distinct: bool) -> Option<u128> {
    if n <= 3 {
        return Some(if distinct {
            u128::from(1 + u64::from(n == 3))
        } else if n == 0 {
            1
        } else {
            u128::from(n)
        });
    }
    // p(n) exceeds u128 near n = 1400. Past the cached limit the value is None.
    let (limit, values) = partition_cache(distinct);
    if n > *limit {
        return None;
    }
    Some(values[n as usize])
}

fn partition_cache(distinct: bool) -> &'static (u64, Vec<u128>) {
    if distinct {
        static Q: OnceLock<(u64, Vec<u128>)> = OnceLock::new();
        Q.get_or_init(|| build_partitions(true))
    } else {
        static P: OnceLock<(u64, Vec<u128>)> = OnceLock::new();
        P.get_or_init(|| build_partitions(false))
    }
}

fn build_partitions(distinct: bool) -> (u64, Vec<u128>) {
    // Build until the recurrence no longer fits in u128.
    let cap = 2000u64;
    let d = integer_sqrt(cap + 1);
    let mut pent = vec![0u64; (2 * d + 3) as usize];
    pent[1] = 1;
    for i in 1..=d {
        pent[(2 * i) as usize] = i * (3 * i + 1) / 2;
        pent[(2 * i + 1) as usize] = (i + 1) * (3 * i + 2) / 2;
    }
    let mut part = vec![0u128; (cap as usize) + 1];
    part[0] = 1;
    let mut last_good = 0u64;
    for j in 1..=cap {
        let mut psum = 0u128;
        let mut ok = true;
        let mut k = 1usize;
        while k < pent.len() && pent[k] != 0 && pent[k] <= j {
            let idx = (j - pent[k]) as usize;
            if (k as u64 + 1) & 2 != 0 {
                match psum.checked_add(part[idx]) {
                    Some(v) => psum = v,
                    None => {
                        ok = false;
                        break;
                    }
                }
            } else {
                match psum.checked_sub(part[idx]) {
                    Some(v) => psum = v,
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            k += 1;
            if k >= pent.len() {
                break;
            }
        }
        if ok && distinct && j % 2 == 0 {
            let hj = j / 2;
            let mut k = 1usize;
            while k < pent.len() && pent[k] <= hj {
                if pent[k] == hj {
                    if (k as u64 + 1) & 2 != 0 {
                        psum = psum.saturating_sub(1);
                    } else {
                        match psum.checked_add(1) {
                            Some(v) => psum = v,
                            None => ok = false,
                        }
                    }
                    break;
                }
                k += 1;
            }
        }
        if !ok {
            break;
        }
        part[j as usize] = psum;
        last_good = j;
    }
    part.truncate((last_good as usize) + 1);
    (last_good, part)
}

fn integer_sqrt(n: u64) -> u64 {
    crate::sqrtint(n)
}

/// Reduce `k` modulo `fact` into `0..fact`.
pub(crate) fn mod_rank(k: i128, fact: u128) -> u128 {
    if fact == 0 {
        return 0;
    }
    if k >= 0 {
        (k as u128) % fact
    } else if fact <= u128::try_from(i128::MAX).unwrap_or(u128::MAX) {
        u128::try_from(k.rem_euclid(fact as i128)).unwrap_or(0)
    } else {
        let mag = k.unsigned_abs() % fact;
        if mag == 0 { 0 } else { fact - mag }
    }
}
