use crate::real::riemann_r;

use super::prime_count;

/// Approximate pi(n), the number of primes up to `n`.
///
/// Port of `prime_counts.c` (MPU 0.75), `prime_count_approx`. Below 3 000 000
/// the exact count is returned. Above that it rounds Riemann's R function.
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::prime_count_approx(10_000_000_000), 455_050_683);
/// ```
#[must_use]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn prime_count_approx(n: u64) -> u64 {
    if n < 3_000_000 {
        return prime_count(n);
    }
    (riemann_r(n as f64) + 0.5) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_below_threshold() {
        assert_eq!(prime_count_approx(0), 0);
        assert_eq!(prime_count_approx(100), 25);
        assert_eq!(prime_count_approx(2_999_999), prime_count(2_999_999));
    }

    #[test]
    fn riemann_branch_is_close_to_exact() {
        // 3 000 000 is above the threshold, so this takes the R(x) branch.
        let approx = prime_count_approx(3_000_000);
        assert!(approx.abs_diff(prime_count(3_000_000)) < 100);
    }
}
