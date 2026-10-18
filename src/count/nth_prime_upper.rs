use super::{nth_prime, prime_count};

/// Largest prime that fits in 64 bits, MPU `constants.h` `MPU_MAX_PRIME`.
pub(super) const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
/// Index of [`MAX_PRIME`], MPU `constants.h` `MPU_MAX_PRIME_IDX`.
pub(super) const MAX_PRIME_IDX: u64 = 425_656_284_035_217_743;

/// Return an upper bound on the `n`th prime, with the first prime as index 1.
///
/// Port of MPU 0.75 `prime_counts.c`, `nth_prime_upper`. For `n` below
/// 15360 this searches the inverse prime-count lower bound.
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
    if n < 306 {
        return nth_prime(n);
    }
    if n < 15_360 {
        return Some(search_count_bound(n, count_lower));
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

/// MPU 0.75 `prime_counts.c::simple_nth_limits` and the binary searches
/// in `nth_prime_lower` and `nth_prime_upper`.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub(super) fn search_count_bound(n: u64, count: fn(u64) -> u64) -> u64 {
    let x = n as f64;
    let log = x.ln();
    let loglog = log.ln();
    let a = if n < 228 { 0.6483 } else if n < 948 { 0.8032 }
        else if n < 2195 { 0.8800 } else if n < 39_017 { 0.9019 } else { 0.9484 };
    let mut lo = (x * (log + loglog - 1.0 + (loglog - 2.10) / log)) as u64;
    let mut hi = (x * (log + loglog - a)) as u64;
    if hi < lo { hi = MAX_PRIME; }
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if count(mid) < n { lo = mid + 1; } else { hi = mid; }
    }
    lo
}

/// MPU 0.75 `prime_counts.c::prime_count_lower`, restricted to the
/// range used by `nth_prime_upper` (below 180 thousand).
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn count_lower(n: u64) -> u64 {
    if n < 33_000 { return prime_count(n); }
    let a = if n < 69_720 { 905.0 } else if n < 70_120 { 961.0 }
        else if n < 88_800 { 918.2 } else { 887.7 };
    let x = n as f64;
    let log = x.ln();
    let log2 = log * log;
    (x / (log - 1.0 - 1.0 / log - 2.975_666 / log2
        - 13.024_334 / (log * log2) + a / (log2 * log2))).ceil() as u64
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
