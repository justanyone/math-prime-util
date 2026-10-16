//! Native combinatorial functions with checked results.

/// Return `n!`, or `None` if it exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `factorial`.
/// The C source is absent from this checkout; source verification is pending.
///
/// ```
/// assert_eq!(math_prime_util::factorial(5), Some(120));
/// ```
#[must_use]
pub fn factorial(n: u64) -> Option<u64> {
    multifactorial(n, 1)
}

/// Return the product `n * (n - step) * ...` of positive terms.
/// Return `None` if the result exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `multifactorial`.
/// The C source is absent from this checkout; source verification is pending.
///
/// # Panics
/// Panics if `step` is zero.
///
/// ```
/// assert_eq!(math_prime_util::multifactorial(7, 2), Some(105));
/// ```
#[must_use]
pub fn multifactorial(mut n: u64, step: u64) -> Option<u64> {
    assert!(step != 0, "multifactorial step must be positive");
    let mut product = 1_u64;
    while n > 0 {
        product = product.checked_mul(n)?;
        n = n.saturating_sub(step);
    }
    Some(product)
}

/// Return the number of permutations of `n` items with no fixed point.
/// Return `None` if the result exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `subfactorial`.
/// The C source is absent from this checkout; source verification is pending.
///
/// ```
/// assert_eq!(math_prime_util::subfactorial(5), Some(44));
/// ```
#[must_use]
pub fn subfactorial(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(1);
    }
    let (mut previous, mut current) = (1_u64, 0_u64);
    for i in 2..=n {
        let next = previous.checked_add(current)?.checked_mul(i - 1)?;
        previous = current;
        current = next;
    }
    Some(current)
}

/// Return the product of `count` decreasing integers starting at `n`.
/// Return `None` if the result exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `falling_factorial`.
/// The C source is absent from this checkout; source verification is pending.
///
/// ```
/// assert_eq!(math_prime_util::falling_factorial(5, 3), Some(60));
/// ```
#[must_use]
pub fn falling_factorial(n: u64, count: u64) -> Option<u64> {
    if count > n {
        return Some(0);
    }
    let mut product = 1_u64;
    for i in 0..count {
        product = product.checked_mul(n - i)?;
    }
    Some(product)
}

/// Return the product of `count` increasing integers starting at `n`.
/// Return `None` if the result exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `rising_factorial`.
/// The C source is absent from this checkout; source verification is pending.
///
/// ```
/// assert_eq!(math_prime_util::rising_factorial(3, 3), Some(60));
/// ```
#[must_use]
pub fn rising_factorial(n: u64, count: u64) -> Option<u64> {
    if n == 0 && count > 0 {
        return Some(0);
    }
    let mut product = 1_u64;
    for i in 0..count {
        product = product.checked_mul(n.checked_add(i)?)?;
    }
    Some(product)
}

/// Return the binomial coefficient `C(n, k)`, or `None` if it exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `binomial`. Uses the multiplicative
/// method with `k = min(k, n - k)`. Intermediate products use `u128`.
///
/// ```
/// assert_eq!(math_prime_util::binomial(20, 10), Some(184_756));
/// assert_eq!(math_prime_util::binomial(5, 7), Some(0));
/// ```
#[must_use]
pub fn binomial(n: u64, k: u64) -> Option<u64> {
    if k > n {
        return Some(0);
    }
    let k = k.min(n - k);
    let mut result = 1_u128;
    for i in 1..=u128::from(k) {
        // result * (n - k + i) / i is always an integer.
        result = result * (u128::from(n - k) + i) / i;
    }
    u64::try_from(result).ok()
}

/// Return the Catalan number `C(n) = binomial(2n, n) / (n + 1)`, or `None`
/// if it exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `catalan_number`.
///
/// ```
/// assert_eq!(math_prime_util::catalan_number(10), Some(16_796));
/// ```
#[must_use]
pub fn catalan_number(n: u64) -> Option<u64> {
    // C(i) = C(i - 1) * 2 * (2i - 1) / (i + 1). Catalan numbers grow, so the
    // first value that exceeds u64 settles the answer.
    let mut value = 1_u128;
    for i in 1..=u128::from(n) {
        value = value * 2 * (2 * i - 1) / (i + 1);
        if value > u128::from(u64::MAX) {
            return None;
        }
    }
    u64::try_from(value).ok()
}

/// Return the Bell number `B(n)`: the number of set partitions of `n`
/// items. Return `None` if it exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `bell_number`. Uses the Bell
/// triangle.
///
/// ```
/// assert_eq!(math_prime_util::bell_number(5), Some(52));
/// ```
#[must_use]
pub fn bell_number(n: u64) -> Option<u64> {
    let mut row = vec![1_u64];
    for _ in 0..n {
        let mut next = vec![*row.last()?];
        for &x in &row {
            let v = next.last()?.checked_add(x)?;
            next.push(v);
        }
        row = next;
    }
    row.first().copied()
}

/// Return the ordered Bell (Fubini) number `a(n) = sum C(n, k) a(n - k)`,
/// or `None` if it exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `fubini`.
///
/// ```
/// assert_eq!(math_prime_util::fubini(4), Some(75));
/// ```
#[must_use]
pub fn fubini(n: u64) -> Option<u64> {
    let mut values = vec![1_u64];
    for m in 1..=n {
        let mut total = 0_u64;
        for k in 1..=m {
            let previous = values[usize::try_from(m - k).ok()?];
            let term = binomial(m, k)?.checked_mul(previous)?;
            total = total.checked_add(term)?;
        }
        values.push(total);
    }
    values.last().copied()
}

/// Return `true` if `n` is prime. Trial division, for the small inputs
/// that the primorial functions can reach before `u64` overflows.
fn is_small_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2_u64;
    while d.saturating_mul(d) <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

/// Return the primorial `p#`: the product of all primes `<= n`, or `None`
/// if it exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `primorial`.
///
/// ```
/// assert_eq!(math_prime_util::primorial(10), Some(210));
/// ```
#[must_use]
pub fn primorial(n: u64) -> Option<u64> {
    let mut product = 1_u64;
    for p in 2..=n {
        if is_small_prime(p) {
            product = product.checked_mul(p)?;
        }
    }
    Some(product)
}

/// Return the product of the first `n` primes, or `None` if it exceeds
/// `u64`.
///
/// Reference entry point: MPU `util.c`, `pn_primorial`.
///
/// ```
/// assert_eq!(math_prime_util::pn_primorial(4), Some(210));
/// ```
#[must_use]
pub fn pn_primorial(n: u64) -> Option<u64> {
    let mut product = 1_u64;
    let mut found = 0_u64;
    let mut candidate = 2_u64;
    while found < n {
        if is_small_prime(candidate) {
            product = product.checked_mul(candidate)?;
            found += 1;
        }
        candidate += 1;
    }
    Some(product)
}

/// Return the least common multiple of `1, 2, ..., n`, or `None` if it
/// exceeds `u64`.
///
/// Reference entry point: MPU `util.c`, `consecutive_integer_lcm`.
///
/// ```
/// assert_eq!(math_prime_util::consecutive_integer_lcm(10), Some(2520));
/// ```
#[must_use]
pub fn consecutive_integer_lcm(n: u64) -> Option<u64> {
    let mut lcm = 1_u64;
    for i in 2..=n {
        let mut a = lcm;
        let mut b = i;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        lcm = (lcm / a).checked_mul(i)?;
    }
    Some(lcm)
}
