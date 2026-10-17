#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use super::nth_ramanujan_prime_lower::SMALL;

// MPU inverse_interpolate.c, _linear_interpolate.
fn linear(lo: u64, hi: u64, rlo: u64, rhi: u64, n: u64) -> u64 {
    if n == rlo {
        return lo + 1;
    }
    if n == rhi {
        return hi - 1;
    }
    let shift = ((n - rlo) as f64 * (hi - lo) as f64 / (rhi - rlo) as f64 + 0.5) as u64;
    lo + shift.clamp(1, hi - lo - 1)
}

// MPU inverse_interpolate.c, _inverse_interpolate, known upper bound and zero threshold.
#[allow(clippy::too_many_lines)] // Keep the three C interpolation stages together.
pub(super) fn inverse(mut lo: u64, mut hi: u64, n: u64, f: impl Fn(u64) -> u64) -> u64 {
    let mid = lo + (hi - lo) / 2;
    let rmid = f(mid);
    let (mut rlo, mut rhi);
    if rmid >= n {
        hi = mid;
        rhi = rmid;
        rlo = f(lo);
        if rlo == n {
            return lo;
        }
    } else {
        lo = mid;
        rlo = rmid;
        rhi = f(hi);
    }
    if hi - lo <= 1 {
        return if rlo == n || rhi > n { lo } else { hi };
    }
    let mut mid = linear(lo, hi, rlo, rhi, n);
    let mut loops = 1;
    while hi - lo > 1 && rhi > n {
        let rmid = f(mid);
        if rmid >= n {
            hi = mid;
            rhi = rmid;
        } else {
            lo = mid;
            rlo = rmid;
        }
        if rhi == n || hi - lo <= 1 {
            break;
        }
        mid = linear(lo, hi, rlo, rhi, n);
        if loops % 6 == 0 {
            let close = (0.003 * (hi - lo) as f64 + 1.0) as u64;
            if lo + close > mid {
                mid = lo + close;
            } else if hi - close < mid {
                mid = hi - close;
            }
        }
        mid = mid.clamp(lo + 1, hi - 1);
        loops += 1;
    }
    if rlo == n {
        return lo;
    }
    if hi - lo <= 1 {
        return if rhi > n { lo } else { hi };
    }
    while hi - lo > 8 {
        let mut x1 = lo + (hi - lo) / 2;
        let mut rx1 = f(x1);
        let d0 = (n - rlo) as f64;
        let d1 = rx1.abs_diff(n) as f64;
        let d2 = (rhi - n + 1) as f64;
        let sign = if rx1 >= n { 1.0 } else { -1.0 };
        let pos = sign * ((x1 - lo) as f64 * d1) / (d1 * d1 + d0 * d2).sqrt();
        let step = pos.round().abs() as u64;
        let mut x3 = if pos >= 0.0 {
            x1.saturating_sub(step)
        } else {
            x1.saturating_add(step)
        };
        if x3 >= hi || x3 <= lo || x3 == x1 {
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else {
                lo = x1;
                rlo = rx1;
            }
        } else {
            let mut rx3 = f(x3);
            if rx1 > rx3 {
                std::mem::swap(&mut x1, &mut x3);
                std::mem::swap(&mut rx1, &mut rx3);
            }
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else if rx3 >= n {
                lo = x1;
                rlo = rx1;
                hi = x3;
                rhi = rx3;
            } else {
                lo = x3;
                rlo = rx3;
            }
        }
    }
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if f(mid) < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}

// MPU ramanujan_primes.c, ramanujan_prime_count_approx.
fn count_approx(n: u64) -> u64 {
    if n <= 241 {
        return SMALL.partition_point(|&p| p <= n) as u64;
    }
    let pc = |x| {
        if x < 3_000_000 {
            crate::prime_count(x)
        } else {
            (crate::riemann_r(x as f64) + 0.5) as u64
        }
    };
    pc(n) - pc(n / 2)
}

/// Estimate the nth Ramanujan prime. Return `None` for index zero.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `nth_ramanujan_prime_approx`.
/// Interpolate the Ramanujan prime count estimate between the C bounds.
///
/// ```
/// assert_eq!(math_prime_util::nth_ramanujan_prime_approx(10), Some(97));
/// assert_eq!(math_prime_util::nth_ramanujan_prime_approx(0), None);
/// ```
pub fn nth_ramanujan_prime_approx(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 23 {
        return Some(SMALL[(n - 1) as usize]);
    }
    let lo = super::nth_ramanujan_prime_lower(n)?.checked_sub(1)?;
    let hi = super::nth_ramanujan_prime::upper(n);
    if hi == 0 {
        return None;
    }
    Some(inverse(lo, hi, n, count_approx))
}
