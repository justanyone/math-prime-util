use super::long_double::Ld;
use super::nth_prime_approx::li_nv;

use super::prime_count;

/// Lower bound for pi(n), the number of primes up to `n`.
///
/// Port of `prime_counts.c` (MPU 0.75), `prime_count_lower`. Below 33000 the
/// exact count is returned. Up to 4 000 000 000 the bound uses Axler's
/// corollaries. Above that it uses Dusart 2018, Lemma 2.2.
///
/// # Examples
///
/// ```
/// let n = 1_000_000;
/// assert!(math_prime_util::prime_count_lower(n) <= math_prime_util::prime_count(n));
/// ```
#[must_use]
pub fn prime_count_lower(n: u64) -> u64 {
    if n < 33_000 {
        return prime_count(n);
    }
    let fn_ = Ld::from_u64(n);
    let fl1 = fn_.ln();
    let fl2 = fl1 * fl1;

    // Axler 2014, Cor 3.6; Axler 2022, Cor 5.1.
    let lower = if n <= 300_070 {
        let a = if n < 69_720 {
            905.0
        } else if n < 70_120 {
            961.0
        } else if n < 88_800 {
            918.2
        } else if n < 176_000 {
            887.7
        } else if n < 299_270 {
            839.46
        } else {
            846.66
        };
        fn_ / (fl1
            - Ld::ONE
            - Ld::ONE / fl1
            - Ld::from_f64(2.975_666) / fl2
            - Ld::from_f64(13.024_334) / (fl1 * fl2)
            + Ld::from_f64(a) / (fl2 * fl2))
    } else if n < 4_000_000_000 {
        // `prime_counts.c` also lists `a = 4` below 88 783 and `a = -3` below
        // 300 000. Both cuts are below 300 070, so this branch never takes them.
        let a = if n < 303_000 {
            5.0
        } else if n < 1_100_000 {
            -7.0
        } else if n < 4_500_000 {
            -37.0
        } else if n < 10_200_000 {
            -70.0
        } else if n < 36_900_000 {
            -53.0
        } else if n < 38_100_000 {
            -29.0
        } else {
            -84.0
        };
        li_nv(fn_)
            - (fn_.sqrt() / fl1)
                * (Ld::new(0, 0xf851_eb85_1eb8_51ec)
                    + Ld::from_f64(2.50) / fl1
                    + Ld::from_f64(a) / fl2)
    } else {
        // Dusart 2018, Lemma 2.2.
        li_nv(fn_) - Ld::from_u64(2) * fn_.sqrt() / fl1
    };
    lower.ceil_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_below_threshold() {
        assert_eq!(prime_count_lower(0), 0);
        assert_eq!(prime_count_lower(100), 25);
        assert_eq!(prime_count_lower(32_999), prime_count(32_999));
    }

    #[test]
    fn bound_holds_in_each_branch() {
        for n in [
            33_000,
            69_720,
            70_000,
            88_783,
            100_000,
            200_000,
            300_070,
            300_071,
            302_999,
            1_000_000,
            2_000_000,
            8_000_000,
            20_000_000,
            37_500_000,
            100_000_000,
            4_000_000_000,
            5_000_000_000,
        ] {
            assert!(prime_count_lower(n) <= n / 2, "n = {n}");
            assert!(prime_count_lower(n) > 0, "n = {n}");
        }
        assert!(prime_count_lower(1_000_000) <= prime_count(1_000_000));
    }
}
