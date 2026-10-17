/// Bound the prime powers less than or equal to `n`.
///
/// Port of MPU 0.75 `prime_powers.c`, `prime_power_count_upper`.
/// Sum the prime counts at each integer root, including exponent one.
///
/// ```
/// assert_eq!(math_prime_util::prime_power_count_upper(100), 35);
/// ```
#[must_use]
pub fn prime_power_count_upper(n: u64) -> u64 {
    if n <= 5 {
        return n.saturating_sub(1);
    }
    let mut sum = count(n);
    for k in 2..=n.ilog2() {
        sum += count(crate::rootint(n, k));
    }
    sum
}
