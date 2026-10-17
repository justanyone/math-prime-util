use super::nth_prime;
use crate::real::riemann_r;

/// Largest prime that fits in 64 bits, MPU `constants.h` `MPU_MAX_PRIME`.
pub(super) const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
/// Index of [`MAX_PRIME`], MPU `constants.h` `MPU_MAX_PRIME_IDX`.
pub(super) const MAX_PRIME_IDX: u64 = 425_656_284_035_217_743;

/// Return an upper bound on the `n`th prime, with the first prime as index 1.
///
/// Port of MPU 0.75 `prime_counts.c`, `nth_prime_upper`. For `n` below
/// 15360 this port returns the exact `n`th prime, which is a valid bound.
/// For larger `n` it uses the Axler and Dusart polynomial bounds with the
/// same coefficient table as MPU, evaluated in `f64` instead of `long double`.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`, and for `n`
/// above the largest prime index that fits in 64 bits.
///
/// ```
/// use math_prime_util::nth_prime_upper;
/// assert!(nth_prime_upper(1_000_000).unwrap() >= 15_485_863);
/// assert_eq!(nth_prime_upper(0), None);
/// ```
#[must_use]
pub fn nth_prime_upper(n: u64) -> Option<u64> {
    if n == 0 || n > MAX_PRIME_IDX {
        return None;
    }
    if n == MAX_PRIME_IDX {
        return Some(MAX_PRIME);
    }
    if n < 15360 {
        return nth_prime(n);
    }
    // MPU keeps the bound in long double; f64 keeps the same polynomial.
    #[allow(clippy::cast_precision_loss)]
    let fnn = n as f64;
    let flogn = fnn.ln();
    let flog2n = flogn.ln();
    let (c, d) = axler_coefficients(n);
    let upper = fnn
        * (flogn + flog2n - 1.0 + ((flog2n - c) / flogn)
            - (flog2n * flog2n - 6.0 * flog2n + d) / (2.0 * flogn * flogn));
    // 2^64 as f64 is exact; MPU returns MAX_PRIME when the bound overflows.
    if upper >= 18_446_744_073_709_551_616.0 {
        return Some(MAX_PRIME);
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let bound = upper.floor() as u64;
    Some(bound)
}

/// Return the `(c, d)` pair of MPU `nth_prime_upper` for the range of `n`.
///
/// Ranges and coefficients are copied from MPU 0.75 `prime_counts.c`,
/// `nth_prime_upper`, which cites Axler 2013, Axler 2017 and Dusart 2010.
fn axler_coefficients(n: u64) -> (f64, f64) {
    if n >= 46_254_381 {
        (2.00, 10.667)
    } else if n >= 8_009_824 {
        (2.00, 10.273)
    } else if n >= 5_450_000 {
        (2.00, 10.1335)
    } else if n >= 3_906_280 {
        (1.67, 20.2675)
    } else if n >= 2_110_840 {
        (2.51, -5.5714)
    } else if n >= 876_700 {
        (2.49, -4.5129)
    } else if n >= 688_382 {
        (3.31, -26.3858)
    } else if n >= 575_750 {
        (-0.79, 83.5215)
    } else if n >= 467_650 {
        (0.93, 37.1597)
    } else if n >= 382_440 {
        (2.92, -15.4768)
    } else if n >= 301_130 {
        (5.92, -91.3415)
    } else if n >= 138_630 {
        (2.01, 7.2842)
    } else if n >= 85_820 {
        (2.07, 5.2103)
    } else if n >= 39_016 {
        (2.76, -11.5918)
    } else if n >= 31_490 {
        (1.49, 15.1821)
    } else if n >= 25_070 {
        (11.89, -197.8951)
    } else {
        (4.80, -51.5928)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_is_exact() {
        assert_eq!(nth_prime_upper(1), Some(2));
        assert_eq!(nth_prime_upper(100), Some(541));
        assert_eq!(nth_prime_upper(0), None);
    }

    #[test]
    fn large_bound_holds() {
        assert!(nth_prime_upper(15_360).unwrap() >= 169_553);
        assert!(nth_prime_upper(1_000_000).unwrap() >= 15_485_863);
        assert!(nth_prime_upper(1_000_000_000).unwrap() >= 22_801_763_489);
    }

    #[test]
    fn limits_and_overflow() {
        assert_eq!(nth_prime_upper(MAX_PRIME_IDX), Some(MAX_PRIME));
        assert_eq!(nth_prime_upper(MAX_PRIME_IDX + 1), None);
        assert_eq!(axler_coefficients(10), (4.80, -51.5928));
        assert_eq!(axler_coefficients(50_000_000).0, 2.00);
    }
}
