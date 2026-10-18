#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use super::almost_prime_count_upper::{bounds, max_count, max_nth, reduce_count};
use super::nth_almost_prime_lower::{inverse, small};

// MPU almost_primes.c: almost_prime_count_approx.
pub(super) fn count_approx(k: u32, n: u64) -> u64 {
    if k == 0 {
        return u64::from(n >= 1);
    }
    if k >= 64 || n >> k == 0 {
        return 0;
    }
    let (k, n) = reduce_count(k, n);
    if k == 0 {
        return u64::from(n >= 1);
    }
    if k == 1 {
        return crate::prime_count_approx(n);
    }
    if k == 2 {
        return crate::semiprime_count_approx(n);
    }
    for (limit, value) in [
        (3_u64 << (k - 1), 1),
        (9_u64 << (k - 2), 2),
        (10_u64 << (k - 2), 3),
    ] {
        if n < limit {
            return value;
        }
    }
    if k == 3 && n < 102 {
        let values = [
            27, 28, 30, 42, 44, 45, 50, 52, 63, 66, 68, 70, 75, 76, 78, 92, 98, 99, 102,
        ];
        return 4 + values.partition_point(|&v| v <= n) as u64;
    }
    let (lo, hi) = bounds(k, n);
    if k == 3 {
        let (s, a) = if n <= 638 {
            (1.554_688, 0.865_814)
        } else if n <= 1544 {
            (1.05, 0.822_256)
        } else if n <= 1927 {
            (0.625, 0.791_747)
        } else if n <= 486_586 {
            (2.865_611, 1.004_090)
        } else if n <= 1_913_680 {
            (2.790_963, 0.999_618)
        } else if n <= 22_347_532 {
            (2.719_238, 0.995_635)
        } else if n <= 295_000_000 {
            (2.584_473, 0.988_802)
        } else if n <= 4_200_000_000 {
            (2.457_108, 0.983_098)
        } else if n <= 70_700_000_000 {
            (2.352_818, 0.978_931)
        } else if n <= 1_360_000_000_000 {
            (2.269_745, 0.975_953)
        } else if n <= 41_000_000_000_000 {
            (2.203_002, 0.973_796)
        } else if n <= 920_000_000_000_000 {
            (2.148_463, 0.972_213)
        } else {
            (2.119_279, 0.971_438)
        };
        let x = n as f64;
        let l = x.ln();
        let ll = l.ln();
        return ((0.5 * a * x * (ll + 0.261_53) * (ll + 0.261_53) / (l + s) + 0.5) as u64)
            .clamp(lo, hi);
    }
    lo + ((hi - lo) as f64 * if k > 11 { 0.20 } else { 0.76 }) as u64
}
/// Estimate the `n`th integer with `k` prime factors.
///
/// Port of MPU 0.75 `almost_primes.c`, `nth_almost_prime_approx`.
/// Return `None` for index zero or a value outside the native range.
///
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime_approx(3, 4), Some(20));
/// ```
#[must_use]
pub fn nth_almost_prime_approx(k: u32, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if k == 0 {
        return (n == 1).then_some(1);
    }
    if k == 1 {
        if n < 306 {
            return crate::nth_prime(n);
        }
        return Some(inverse_r(n));
    }
    if k == 2 {
        return crate::nth_semiprime_approx(n);
    }
    let max = max_count(k);
    if n >= max {
        return (n == max && max > 0).then(|| max_nth(k));
    }
    if n < 8 {
        return Some(small(k, n));
    }
    let lo = super::nth_almost_prime_lower(k, n)?;
    Some(inverse(lo, 0, n, |x| count_approx(k, x)))
}

// MPU real.c: ld_inverse_R and inverse_R.
fn inverse_r(n: u64) -> u64 {
    let x = n as f64;
    let mut t = if x <= 3.5 {
        x + 2.24 * (x - 1.0) / 2.0
    } else {
        x * x.ln()
    };
    if x > 3.5 {
        if x < 50.0 {
            t *= 1.2;
        } else if x < 1000.0 {
            t *= 1.15;
        } else {
            let dn = crate::logarithmic_integral(t) - x;
            t -= dn * t.ln() / (1.0 + dn / (2.0 * t));
        }
    }
    let mut old: f64 = 0.0;
    for i in 0..100 {
        let dn = crate::riemann_r(t) - x;
        let term = dn * t.ln() / (1.0 + dn / (2.0 * t));
        if i > 0 && term.abs() >= old.abs() {
            t -= term / 4.0;
            break;
        }
        old = term;
        t -= term;
    }
    t.ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_formula_transitions() {
        for n in [
            8,
            12,
            18,
            20,
            27,
            100,
            102,
            638,
            639,
            1544,
            1545,
            1927,
            1928,
            486_586,
            486_587,
            1_913_681,
            22_347_533,
            295_000_001,
            4_200_000_001,
            70_700_000_001,
            1_360_000_000_001,
            41_000_000_000_001,
            920_000_000_000_001,
        ] {
            let (lo, hi) = bounds(3, n);
            let estimate = count_approx(3, n);
            assert!((lo..=hi).contains(&estimate));
        }
        assert_eq!(count_approx(0, 0), 0);
        assert_eq!(count_approx(0, 1), 1);
        assert_eq!(count_approx(64, 100), 0);
        assert_eq!(count_approx(3, 0), 0);
        for k in [1, 2, 5, 12, 20, 40, 63] {
            let value = count_approx(k, u64::MAX / 2);
            let (lo, hi) = bounds(k, u64::MAX / 2);
            assert!((lo..=hi).contains(&value));
        }
    }
    #[test]
    fn inverse_r_matches_its_target() {
        for n in [2, 4, 40, 500, 1000, 1_000_000] {
            let value = inverse_r(n);
            assert!((crate::riemann_r(value as f64) - n as f64).abs() < 1.0);
        }
        assert!(nth_almost_prime_approx(1, 1000).unwrap() > 7900);
    }
}
