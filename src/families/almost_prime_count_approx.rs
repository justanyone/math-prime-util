#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::unreadable_literal
)]

use super::almost_prime_count::{bounds, reduce, small};
/// Estimate the count of `k`-almost primes at most `n`.
///
/// Port of MPU 0.75 `almost_primes.c`: `almost_prime_count_approx`.
/// Use the reference fit and weighted bounds.
/// ```
/// use math_prime_util::almost_prime_count_approx;
/// assert_eq!(almost_prime_count_approx(3, 30), 7);
/// ```
pub fn almost_prime_count_approx(mut k: u64, mut n: u64) -> u64 {
    if k == 0 {
        return u64::from(n >= 1);
    }
    if k >= 64 || n >> k == 0 {
        return 0;
    }
    reduce(&mut k, &mut n);
    if k == 0 {
        return u64::from(n >= 1);
    }
    if k == 1 {
        return crate::prime_count_approx(n);
    }
    if k == 2 {
        return crate::semiprime_count_approx(n);
    }
    if let Some(v) = small(k, n) {
        return v;
    }
    if k == 3 && n < 102 {
        const SM3: [u64; 19] = [
            27, 28, 30, 42, 44, 45, 50, 52, 63, 66, 68, 70, 75, 76, 78, 92, 98, 99, 102,
        ];
        return 4 + SM3.partition_point(|&v| v <= n) as u64;
    }
    let (lo, hi) = bounds(k, n);
    if k == 3 {
        let x = n as f64;
        let (s, a) = if x <= 638.0 {
            (1.554688, 0.865814)
        } else if x <= 1544.0 {
            (1.050000, 0.822256)
        } else if x <= 1927.0 {
            (0.625000, 0.791747)
        } else if x <= 486586.0 {
            (2.865611, 1.004090)
        } else if x <= 1913680.0 {
            (2.790963, 0.999618)
        } else if x <= 22347532.0 {
            (2.719238, 0.995635)
        } else if x <= 2.95e8 {
            (2.584473, 0.988802)
        } else if x <= 4.20e9 {
            (2.457108, 0.983098)
        } else if x <= 7.07e10 {
            (2.352818, 0.978931)
        } else if x <= 1.36e12 {
            (2.269745, 0.975953)
        } else if x <= 4.1e13 {
            (2.203002, 0.973796)
        } else if x <= 9.2e14 {
            (2.148463, 0.972213)
        } else {
            (2.119279, 0.971438)
        };
        let logx = x.ln();
        let t = logx.ln() + 0.26153;
        return ((0.5 * a * x * (t * t) / (logx + s) + 0.5) as u64).clamp(lo, hi);
    }
    (lo as f64 + (hi - lo) as f64 * if k > 11 { 0.20 } else { 0.76 }) as u64
}
