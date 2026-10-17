/// Returns the product of `m` descending integers from `n`.
///
/// Returns `None` if the result exceeds `u64`. Uses `util.c`'s
/// `falling_factorial` from MPU 0.75.
///
/// ```
/// assert_eq!(math_prime_util::falling_factorial(5, 3), Some(60));
/// ```
pub fn falling_factorial(n: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return Some(1);
    }
    if m > n {
        return Some(0);
    }
    let mut result = n;
    for i in 1..m {
        result = result.checked_mul(n - i)?;
    }
    Some(result)
}
