/// Return the upper bound for the `n`th prime power.
/// Returns `None` for zero or an index beyond the native prime limit.
/// Port of MPU 0.75 `prime_powers.c`, `nth_prime_power_upper`.
/// ```
/// assert_eq!(math_prime_util::nth_prime_power_upper(10), Some(16));
/// ```
pub fn nth_prime_power_upper(n: u64) -> Option<u64> {
    super::nth_prime_power_approx::estimate(n, 2)
}
