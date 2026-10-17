/// Returns whether `n` is a perfect power. This includes zero and one.
///
/// Port of MPU 0.75 `perfect_powers.c`, `is_perfect_power`.
///
/// ```
/// use math_prime_util::is_perfect_power;
/// assert!(is_perfect_power(64));
/// assert!(!is_perfect_power(65));
/// ```
pub fn is_perfect_power(n: u64) -> bool {
    n <= 1 || crate::int::is_power_exp(n) > 1
}
