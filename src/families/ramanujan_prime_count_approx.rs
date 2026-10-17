use super::ramanujan_primes::SMALL;

/// Estimate the number of Ramanujan primes at most `n`.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `ramanujan_prime_count_approx`.
/// It subtracts two prime-count estimates. The estimate need not be monotone.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_prime_count_approx(1000), 73);
/// ```
pub fn ramanujan_prime_count_approx(n: u64) -> u64 {
    if n <= 241 {
        return SMALL.partition_point(|&p| p <= n) as u64;
    }
    estimate(n) - estimate(n / 2)
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn estimate(n: u64) -> u64 {
    if n < 3_000_000 {
        crate::prime_count(n)
    } else {
        (crate::riemann_r(n as f64) + 0.5) as u64
    }
}
