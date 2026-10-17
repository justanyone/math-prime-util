use super::falling_factorial;

/// Returns the product of `m` ascending integers from `n`.
///
/// Returns `None` if the result exceeds `u64`. Uses `util.c`'s
/// `rising_factorial` from MPU 0.75.
///
/// ```
/// assert_eq!(math_prime_util::rising_factorial(5, 3), Some(210));
/// ```
pub fn rising_factorial(n: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return Some(1);
    }
    falling_factorial(n.checked_add(m - 1)?, m)
}
