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
