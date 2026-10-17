/// Return an upper bound for the positive perfect powers at most `n`.
///
/// Port of `perfect_power_count_upper` in MPU 0.75 `perfect_powers.c`.
/// MPU returns the exact count, including 1.
///
/// ```
/// use math_prime_util::perfect_power_count_upper;
/// assert_eq!(perfect_power_count_upper(100), 13);
/// ```
#[must_use]
pub fn perfect_power_count_upper(n: u64) -> u64 {
    super::perfect_power_count_approx::count(n)
}
