use super::long_double::Ld;
use super::{nth_prime, prime_count_lower};

/// Largest prime that fits in 64 bits, MPU `constants.h` `MPU_MAX_PRIME`.
pub(super) const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
/// Index of [`MAX_PRIME`], MPU `constants.h` `MPU_MAX_PRIME_IDX`.
pub(super) const MAX_PRIME_IDX: u64 = 425_656_284_035_217_743;
/// MPU `constants.h` `NPRIMES_SMALL`: below this index MPU reads its table.
pub(super) const NPRIMES_SMALL: u64 = 306;

/// Return an upper bound on the `n`th prime, with the first prime as index 1.
///
/// Port of MPU 0.75 `prime_counts.c`, `nth_prime_upper`. Below index 306 the
/// result is the exact prime. Below 15360 it is the smallest `x` with
/// [`prime_count_lower`]`(x) >= n`, a binary search. From 15360 up it is the
/// Axler and Dusart bound
/// `n (ln n + ln ln n - 1 + (ln ln n - c) / ln n - (ln² ln n - 6 ln ln n + d) / (2 ln² n))`
/// with MPU's table of `(c, d)` per range, evaluated in x87 `long double`
/// as MPU does, so that the result matches MPU to the last digit.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`, and for `n` above
/// `425_656_284_035_217_743`, the index of the largest 64-bit prime, where
/// the `n`th prime does not fit in a `u64` (MPU answers those with bigints).
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_prime_upper;
/// assert_eq!(nth_prime_upper(10_000), Some(104_925));
/// assert!(nth_prime_upper(1_000_000).unwrap() >= 15_485_863);
/// assert_eq!(nth_prime_upper(0), None);
/// ```
#[must_use]
pub fn nth_prime_upper(n: u64) -> Option<u64> {
    if n == 0 || n > MAX_PRIME_IDX {
        return None;
    }
    if n < NPRIMES_SMALL {
        return nth_prime(n);
    }
    if n == MAX_PRIME_IDX {
        return Some(MAX_PRIME);
    }
    let fnn = Ld::from_u64(n);
    let flogn = fnn.ln();
    let flog2n = flogn.ln();
    if n < 15_360 {
        let (lo, hi) = simple_nth_limits(n, fnn, flogn, flog2n);
        return Some(search_count_bound(lo, hi, n, prime_count_lower));
    }
    let (c, d) = axler_coefficients(n);
    let (c, d) = (Ld::from_f64(c), Ld::from_f64(d));
    let one = Ld::ONE;
    let two = Ld::from_u64(2);
    let six = Ld::from_u64(6);
    let upper = fnn
        * (flogn + flog2n - one + ((flog2n - c) / flogn)
            - (flog2n * flog2n - six * flog2n + d) / (two * flogn * flogn));
    // MPU returns MPU_MAX_PRIME when the bound reaches UV_MAX.
    if upper >= Ld::from_u64(u64::MAX) {
        return Some(MAX_PRIME);
    }
    Some(upper.floor_u64())
}

/// MPU 0.75 `prime_counts.c`, `simple_nth_limits`: the search interval for
/// the `n`th prime, from `n`, `ln n` and `ln ln n` as the caller has them.
pub(super) fn simple_nth_limits(n: u64, fnn: Ld, logn: Ld, loglogn: Ld) -> (u64, u64) {
    let a = if n < 228 {
        0.6483
    } else if n < 948 {
        0.8032
    } else if n < 2195 {
        0.8800
    } else if n < 39_017 {
        0.9019
    } else {
        0.9484
    };
    let lo = fnn * (logn + loglogn - Ld::ONE + ((loglogn - Ld::from_f64(2.10)) / logn));
    let hi = fnn * (logn + loglogn - Ld::from_f64(a));
    let (lo, hi) = (lo.floor_u64(), hi.floor_u64());
    (lo, if hi < lo { MAX_PRIME } else { hi })
}

/// The binary searches of MPU 0.75 `nth_prime_upper` and `nth_prime_lower`:
/// the smallest `x` in `lo..=hi` with `count(x) >= n`.
pub(super) fn search_count_bound(mut lo: u64, mut hi: u64, n: u64, count: fn(u64) -> u64) -> u64 {
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if count(mid) < n {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// The `(c, d)` pair of MPU 0.75 `nth_prime_upper` for `n >= 15360`.
///
/// Ranges and coefficients are copied from `prime_counts.c`, which cites
/// Axler 2013, Axler 2017 and Dusart 2010. MPU's last row (`n < 15359`) is
/// not reached: smaller `n` take the binary search.
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
        assert_eq!(nth_prime_upper(305), Some(2011));
        assert_eq!(nth_prime_upper(0), None);
    }

    #[test]
    fn search_range_values() {
        // MPU 0.75 values (C `nth_prime_upper` on x86-64).
        assert_eq!(nth_prime_upper(306), Some(2017));
        assert_eq!(nth_prime_upper(7_919), Some(81_069));
        assert_eq!(nth_prime_upper(15_359), Some(168_406));
    }

    #[test]
    fn polynomial_values() {
        // MPU 0.75 values (C `nth_prime_upper` on x86-64); one per range.
        for (n, want) in [
            (15_360, 168_432),
            (25_070, 288_444),
            (31_490, 369_885),
            (39_016, 467_496),
            (85_820, 1_101_519),
            (138_630, 1_851_418),
            (301_130, 4_274_975),
            (382_440, 5_527_395),
            (467_650, 6_859_064),
            (575_750, 8_573_264),
            (688_382, 10_384_259),
            (876_700, 13_453_009),
            (2_110_840, 34_378_685),
            (3_906_280, 66_182_122),
            (5_450_000, 94_274_086),
            (8_009_824, 141_836_550),
            (46_254_381, 905_060_142),
            (100_000_000_000_000_000, 4_185_309_580_126_018_227),
        ] {
            assert_eq!(nth_prime_upper(n), Some(want), "n = {n}");
        }
    }

    #[test]
    fn limits() {
        assert_eq!(nth_prime_upper(MAX_PRIME_IDX - 1), Some(MAX_PRIME));
        assert_eq!(nth_prime_upper(MAX_PRIME_IDX), Some(MAX_PRIME));
        assert_eq!(nth_prime_upper(MAX_PRIME_IDX + 1), None);
        assert_eq!(nth_prime_upper(u64::MAX), None);
    }

    #[test]
    fn limits_interval() {
        let n = 1_000;
        let fnn = Ld::from_u64(n);
        let (lo, hi) = simple_nth_limits(n, fnn, fnn.ln(), fnn.ln().ln());
        assert!(lo < 7_919 && 7_919 <= hi);
        assert_eq!(
            search_count_bound(lo, hi, n, super::super::prime_count),
            7_919
        );
    }
}
