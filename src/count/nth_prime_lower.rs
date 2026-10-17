use super::nth_prime;
use super::nth_prime_upper::{MAX_PRIME, MAX_PRIME_IDX};

/// Return a lower bound on the `n`th prime, with the first prime as index 1.
///
/// Port of MPU 0.75 `prime_counts.c`, `nth_prime_lower`. For `n` below
/// 2,000,000 this port returns the exact `n`th prime, which is a valid
/// bound. For larger `n` it uses the Axler 2017 Corollary 1.4 bound, with
/// the same constants as MPU, evaluated in `f64`.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`, and for `n`
/// above the largest prime index that fits in 64 bits.
///
/// ```
/// use math_prime_util::nth_prime_lower;
/// assert!(nth_prime_lower(1_000_000).unwrap() <= 15_485_863);
/// assert_eq!(nth_prime_lower(0), None);
/// ```
#[must_use]
pub fn nth_prime_lower(n: u64) -> Option<u64> {
    if n == 0 || n > MAX_PRIME_IDX {
        return None;
    }
    if n == MAX_PRIME_IDX {
        return Some(MAX_PRIME);
    }
    if n < 2_000_000 {
        return nth_prime(n);
    }
    // MPU keeps the bound in double; the constant changes above 56,000,000.
    let b1 = if n < 56_000_000 { 11.2 } else { 11.508_000_000_02 };
    #[allow(clippy::cast_precision_loss)]
    let fnn = n as f64;
    let flogn = fnn.ln();
    let flog2n = flogn.ln();
    let lower = fnn
        * (flogn + flog2n - 1.0 + ((flog2n - 2.0) / flogn)
            - ((flog2n * flog2n - 6.0 * flog2n + b1) / (2.0 * flogn * flogn)));
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let plower = lower.ceil() as u64;
    Some(plower.min(MAX_PRIME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_is_exact() {
        assert_eq!(nth_prime_lower(1), Some(2));
        assert_eq!(nth_prime_lower(100), Some(541));
        assert_eq!(nth_prime_lower(0), None);
    }

    #[test]
    fn large_bound_holds() {
        assert!(nth_prime_lower(2_000_000).unwrap() <= 32_452_843);
        assert!(nth_prime_lower(1_000_000_000).unwrap() <= 22_801_763_489);
        assert!(nth_prime_lower(1_000_000_000).unwrap() > 1_000_000_000);
    }

    #[test]
    fn limits() {
        assert_eq!(nth_prime_lower(MAX_PRIME_IDX), Some(MAX_PRIME));
        assert_eq!(nth_prime_lower(MAX_PRIME_IDX + 1), None);
    }
}
