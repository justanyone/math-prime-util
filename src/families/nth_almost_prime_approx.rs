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
            (1.554688, 0.865814)
        } else if n <= 1544 {
            (1.05, 0.822256)
        } else if n <= 1927 {
            (0.625, 0.791747)
        } else if n <= 486586 {
            (2.865611, 1.004090)
        } else if n <= 1913680 {
            (2.790963, 0.999618)
        } else if n <= 22347532 {
            (2.719238, 0.995635)
        } else if n <= 295000000 {
            (2.584473, 0.988802)
        } else if n <= 4200000000 {
            (2.457108, 0.983098)
        } else if n <= 70700000000 {
            (2.352818, 0.978931)
        } else if n <= 1360000000000 {
            (2.269745, 0.975953)
        } else if n <= 41000000000000 {
            (2.203002, 0.973796)
        } else if n <= 920000000000000 {
            (2.148463, 0.972213)
        } else {
            (2.119279, 0.971438)
        };
        let x = n as f64;
        let l = x.ln();
        let ll = l.ln();
        return ((0.5 * a * x * (ll + 0.26153) * (ll + 0.26153) / (l + s) + 0.5) as u64)
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
        return Some(inverse(2, 0, n, |x| {
            (crate::riemann_r(x as f64) + 0.5) as u64
        }));
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
