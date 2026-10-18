use super::nth_prime_upper::{MAX_PRIME, MAX_PRIME_IDX};
use super::{nth_prime, prime_count};

/// Return a lower bound on the `n`th prime, with the first prime as index 1.
///
/// Port of MPU 0.75 `prime_counts.c`, `nth_prime_lower`. For `n` below
/// 2,000,000 this searches the inverse prime-count upper bound.
/// For larger `n` it uses the Axler 2017 Corollary 1.4 bound, with
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
    if n < 306 {
        return nth_prime(n);
    }
    if n < 2_000_000 {
        return Some(super::nth_prime_upper::search_count_bound(n, count_upper));
    }
    // MPU keeps the bound in double; the constant changes above 56,000,000.
    let b1 = if n < 56_000_000 {
        11.2
    } else {
        11.508_000_000_02
    };
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

/// MPU 0.75 `prime_counts.c::prime_count_upper`, restricted to the
/// range used by `nth_prime_lower` (below 40 million).
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn count_upper(n: u64) -> u64 {
    if n < 33_000 {
        return prime_count(n);
    }
    // The C table stores single-precision coefficients before promoting them.
    let thresholds = [
        (59_000, 2.48_f32),
        (355_991, 2.54),
        (3_550_000, 2.51),
        (3_560_000, 2.49),
        (5_000_000, 2.48),
        (8_000_000, 2.47),
        (13_000_000, 2.46),
        (18_000_000, 2.45),
        (31_000_000, 2.44),
        (41_000_000, 2.43),
    ];
    let a = thresholds.iter().find(|&&(limit, _)| n < limit).unwrap().1;
    let x = n as f64;
    let log = x.ln();
    (x / log * (1.0 + 1.0 / log + f64::from(a) / (log * log))).floor() as u64
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
    fn inverse_count_regressions() {
        // MPU 0.75 oracle cases: these bounds differ from the exact prime.
        assert_eq!(nth_prime_lower(7_919), Some(80_725));
        assert_eq!(nth_prime_lower(10_000), Some(104_540));
        assert_eq!(nth_prime_lower(100_000), Some(1_298_980));
        assert_eq!(nth_prime_lower(1_000_000), Some(15_481_618));
        for n in [306, 947, 948, 2_194, 2_195, 39_016, 39_017, 1_999_999] {
            let bound = nth_prime_lower(n).unwrap();
            assert!(count_upper(bound) >= n);
            assert!(count_upper(bound - 1) < n);
        }
    }

    #[test]
    fn count_upper_coefficient_ranges() {
        assert_eq!(count_upper(100), 25);
        for n in [
            33_000, 59_000, 355_991, 3_550_000, 3_560_000, 5_000_000, 8_000_000, 13_000_000,
            18_000_000, 31_000_000,
        ] {
            assert!(count_upper(n) >= prime_count(n));
        }
        assert!(
            nth_prime_lower(56_000_000).unwrap()
                < super::super::nth_prime_upper(56_000_000).unwrap()
        );
    }

    #[test]
    fn limits() {
        assert_eq!(nth_prime_lower(MAX_PRIME_IDX), Some(MAX_PRIME));
        assert_eq!(nth_prime_lower(MAX_PRIME_IDX + 1), None);
    }
}
