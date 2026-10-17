/// Returns `n!`, or `None` if it exceeds `u64`.
///
/// Port of `factorial` in MPU 0.75 `util.c`, with the 64-bit bound.
///
/// ```
/// use math_prime_util::factorial;
/// assert_eq!(factorial(5), Some(120));
/// assert_eq!(factorial(21), None);
/// ```
pub fn factorial(n: u64) -> Option<u64> {
    if n > 20 {
        return None;
    }
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    Some(result)
}
