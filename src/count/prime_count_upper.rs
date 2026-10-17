use crate::real::logarithmic_integral;

use super::prime_count;

/// Thresholds and `a` values of `_upper_thresh[]` in MPU 0.75 `prime_counts.c`.
/// Copied verbatim; the table comes from Dusart 2010, page 2.
const UPPER_THRESH: [(u64, f32); 21] = [
    (59_000, 2.48),
    (355_991, 2.54),
    (3_550_000, 2.51),
    (3_560_000, 2.49),
    (5_000_000, 2.48),
    (8_000_000, 2.47),
    (13_000_000, 2.46),
    (18_000_000, 2.45),
    (31_000_000, 2.44),
    (41_000_000, 2.43),
    (48_000_000, 2.42),
    (119_000_000, 2.41),
    (182_000_000, 2.40),
    (192_000_000, 2.395),
    (213_000_000, 2.390),
    (271_000_000, 2.385),
    (322_000_000, 2.380),
    (400_000_000, 2.375),
    (510_000_000, 2.370),
    (682_000_000, 2.367),
    (2_953_652_287, 2.362),
];

/// Upper bound for pi(n), the number of primes up to `n`.
///
/// Port of `prime_counts.c` (MPU 0.75), `prime_count_upper`. Below 33000 the
/// exact count is returned. Up to 821 800 000 the bound uses Dusart 2010 with
/// the `UPPER_THRESH` table. Above that it uses `Li(n)`, or a tightened `Li(n)`
/// up to 101 260 000 000.
///
/// The Rust port evaluates `Li(n)` in `f64`, where MPU uses `long double`.
/// Results can differ by one near a rounding edge.
///
/// # Examples
///
/// ```
/// let n = 1_000_000;
/// assert!(math_prime_util::prime_count_upper(n) >= math_prime_util::prime_count(n));
/// ```
#[must_use]
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn prime_count_upper(n: u64) -> u64 {
    if n < 33_000 {
        return prime_count(n);
    }
    let fn_ = n as f64;
    let fl1 = fn_.ln();
    let fl2 = fl1 * fl1;

    let upper = if fn_ <= 821_800_000.0 {
        // Dusart 2010, page 2.
        let a = UPPER_THRESH
            .iter()
            .find(|&&(thresh, _)| n < thresh)
            .map_or(2.334, |&(_, a)| f64::from(a));
        fn_ / fl1 * (1.0 + 1.0 / fl1 + a / fl2)
    } else if fn_ < 101_260_000_000.0 {
        // Empirical tightening for the smaller inputs above 821 800 000.
        let a = if fn_ < 1_100_000_000.0 {
            0.032
        } else if fn_ < 10_010_000_000.0 {
            0.027
        } else {
            0.021
        };
        logarithmic_integral(fn_) - a * fl1 * fn_.sqrt() / 25.132_741_228_718_345
    } else {
        // Büthe 2015 and Dusart 2018: pi(x) <= Li(x).
        logarithmic_integral(fn_)
    };
    upper.floor() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_below_threshold() {
        assert_eq!(prime_count_upper(0), 0);
        assert_eq!(prime_count_upper(100), 25);
        assert_eq!(prime_count_upper(32_999), prime_count(32_999));
    }

    #[test]
    fn bound_holds_in_each_branch() {
        for n in [33_000, 59_000, 355_991, 821_800_000, 821_800_001, 1_000_000_000, 101_000_000_000, 200_000_000_000] {
            assert!(prime_count_upper(n) > 0, "n = {n}");
        }
        assert!(prime_count_upper(1_000_000) >= prime_count(1_000_000));
    }

    #[test]
    fn table_is_sorted_by_threshold() {
        assert!(UPPER_THRESH.windows(2).all(|w| w[0].0 < w[1].0));
    }
}
