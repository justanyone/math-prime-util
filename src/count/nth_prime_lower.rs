use super::long_double::Ld;
use super::nth_prime_upper::{
    MAX_PRIME, MAX_PRIME_IDX, NPRIMES_SMALL, search_count_bound, simple_nth_limits,
};
use super::{nth_prime, prime_count_upper};

/// Return a lower bound on the `n`th prime, with the first prime as index 1.
///
/// Port of MPU 0.75 `prime_counts.c`, `nth_prime_lower`. Below index 306 the
/// result is the exact prime. Below 2,000,000 it is the smallest `x` with
/// [`prime_count_upper`]`(x) >= n`, a binary search. From 2,000,000 up it is
/// the Axler 2017 Corollary 1.4 bound
/// `n (ln n + ln ln n - 1 + (ln ln n - 2) / ln n - (ln² ln n - 6 ln ln n + b) / (2 ln² n))`
/// with `b = 11.2` below 56,000,000 and `b = 11.508` from there, evaluated in
/// `double` as MPU does. The bound is not monotone in `n` where `b` changes
/// or the method changes, exactly as in MPU.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`, and for `n` above
/// `425_656_284_035_217_743`, the index of the largest 64-bit prime, where
/// the `n`th prime does not fit in a `u64` (MPU answers those with bigints).
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_prime_lower;
/// assert_eq!(nth_prime_lower(10_000), Some(104_540));
/// assert!(nth_prime_lower(1_000_000).unwrap() <= 15_485_863);
/// assert_eq!(nth_prime_lower(0), None);
/// ```
#[must_use]
pub fn nth_prime_lower(n: u64) -> Option<u64> {
    if n == 0 || n > MAX_PRIME_IDX {
        return None;
    }
    if n < NPRIMES_SMALL {
        return nth_prime(n);
    }
    if n == MAX_PRIME_IDX {
        return Some(MAX_PRIME);
    }
    #[allow(clippy::cast_precision_loss)]
    let fnn = n as f64;
    let flogn = fnn.ln();
    let flog2n = flogn.ln();
    if n < 2_000_000 {
        // MPU passes its `double` values to the `long double` limits.
        let (lo, hi) = simple_nth_limits(
            n,
            Ld::from_f64(fnn),
            Ld::from_f64(flogn),
            Ld::from_f64(flog2n),
        );
        return Some(search_count_bound(lo, hi, n, prime_count_upper));
    }
    let b1 = if n < 56_000_000 {
        11.200
    } else {
        11.508_000_000_02
    };
    let lower = fnn
        * (flogn + flog2n - 1.0 + ((flog2n - 2.00) / flogn)
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
        assert_eq!(nth_prime_lower(305), Some(2011));
        assert_eq!(nth_prime_lower(0), None);
    }

    #[test]
    fn search_range_values() {
        // MPU 0.75 values (C `nth_prime_lower` on x86-64).
        assert_eq!(nth_prime_lower(306), Some(2017));
        assert_eq!(nth_prime_lower(999), Some(7_907));
        assert_eq!(nth_prime_lower(7_919), Some(80_725));
        assert_eq!(nth_prime_lower(1_999_999), Some(32_450_549));
    }

    #[test]
    fn formula_values() {
        // MPU 0.75 values; the bound drops where the method or `b` changes.
        assert_eq!(nth_prime_lower(2_000_000), Some(32_448_866));
        assert_eq!(nth_prime_lower(55_999_999), Some(1_107_023_901));
        assert_eq!(nth_prime_lower(56_000_000), Some(1_106_996_828));
        assert_eq!(
            nth_prime_lower(400_000_000_000_000_000),
            Some(17_309_348_184_244_834_304)
        );
    }

    #[test]
    fn limits() {
        assert_eq!(
            nth_prime_lower(MAX_PRIME_IDX - 1),
            Some(18_446_686_113_192_781_824)
        );
        assert_eq!(nth_prime_lower(MAX_PRIME_IDX), Some(MAX_PRIME));
        assert_eq!(nth_prime_lower(MAX_PRIME_IDX + 1), None);
        assert_eq!(nth_prime_lower(u64::MAX), None);
    }
}
