#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::unreadable_literal,
    clippy::excessive_precision
)]
use crate::{exponential_integral, is_prime};

const MAX_TWIN: u64 = 18_446_744_073_709_550_771;

/// Estimate the smaller prime of the `n`th twin prime pair.
///
/// Port of `twin_primes.c`, `nth_twin_prime_approx`, in MPU 0.75.
/// Use the reference bounds and invert the twin prime count estimate.
/// Return `None` when `n` is zero.
///
/// ```
/// use math_prime_util::nth_twin_prime_approx;
/// assert_eq!(nth_twin_prime_approx(10), Some(107));
/// assert_eq!(nth_twin_prime_approx(0), None);
/// ```
pub fn nth_twin_prime_approx(n: u64) -> Option<u64> {
    if n < 6 {
        return [None, Some(3), Some(5), Some(11), Some(17), Some(29)][n as usize];
    }
    if n >= 12_975_810_317_986_308 {
        return Some(MAX_TWIN);
    }
    let x = n as f64;
    let scale = x * x.ln() * x.ln();
    let lower = 0.9 * scale;
    let upper = if n >= 10_000_000_000_000_000 {
        1.04 * scale
    } else if n >= 10_000_000_000_000 {
        1.10 * scale
    } else if n >= 10_000_000 {
        1.31 * scale
    } else if n >= 1200 {
        1.70 * scale
    } else {
        2.30 * scale + 5.0
    };
    let lo = (lower as u64).min(MAX_TWIN);
    let hi = (upper as u64).min(MAX_TWIN);
    if lo >= hi || lo >= MAX_TWIN || count_approx(hi) < n {
        return Some(MAX_TWIN);
    }
    Some(inverse(lo, hi, n))
}

// twin_primes.c: twin_prime_count_approx (private to this port slice).
fn count_approx(n: u64) -> u64 {
    if n < 2000 {
        return (3..=n)
            .filter(|&p| is_prime(p) != 0 && is_prime(p + 2) != 0)
            .count() as u64;
    }
    let x = n as f64;
    let logn = x.ln();
    let mut li2 = exponential_integral(logn) + 2.8853900817779268147198494 - x / logn;
    if n < 32_000_000 {
        let fm = match n {
            0..4000 => 0.2952,
            4000..8000 => 0.3102,
            8000..16000 => 0.3090,
            16000..32000 => 0.3096,
            32000..64000 => 0.3097,
            64000..128000 => 0.3094,
            128000..256000 => 0.3099,
            256000..600000 => 0.3098 + (x - 256000.0) * (0.3056 - 0.3098) / (600000.0 - 256000.0),
            600000..1000000 => 0.3062 + (x - 600000.0) * (0.3042 - 0.3062) / (1000000.0 - 600000.0),
            1000000..4000000 => {
                0.3067 + (x - 1000000.0) * (0.3041 - 0.3067) / (4000000.0 - 1000000.0)
            }
            4000000..16000000 => {
                0.3041 + (x - 4000000.0) * (0.2983 - 0.3041) / (16000000.0 - 4000000.0)
            }
            _ => 0.2983 + (x - 16000000.0) * (0.2961 - 0.2983) / (32000000.0 - 16000000.0),
        };
        li2 *= fm * (12.0 + logn).ln();
    }
    (1.32032363169373914785562422 * li2 + 0.5) as u64
}

// inverse_interpolate.c: _linear_interpolate.
fn linear(lo: u64, hi: u64, rlo: u64, rhi: u64, n: u64) -> u64 {
    if n == rlo {
        return lo + 1;
    }
    if n == rhi {
        return hi - 1;
    }
    let step = ((n - rlo) as f64 * (hi - lo) as f64 / (rhi - rlo) as f64 + 0.5) as u64;
    lo + step.clamp(1, hi - lo - 1)
}

// inverse_interpolate.c: _inverse_interpolate, with known bounds and threshold 0.
fn inverse(mut lo: u64, mut hi: u64, n: u64) -> u64 {
    let mut mid = lo + (hi - lo) / 2;
    let rmid = count_approx(mid);
    let (mut rlo, mut rhi) = if rmid >= n {
        hi = mid;
        (count_approx(lo), rmid)
    } else {
        lo = mid;
        (rmid, count_approx(hi))
    };
    if rlo == n {
        return lo;
    }
    if hi - lo <= 1 {
        return if rhi > n { lo } else { hi };
    }
    mid = linear(lo, hi, rlo, rhi, n);
    let mut iteration = 1;
    while hi - lo > 1 && rhi > n {
        let value = count_approx(mid);
        if value >= n {
            hi = mid;
            rhi = value;
        } else {
            lo = mid;
            rlo = value;
        }
        if rhi == n || hi - lo <= 1 {
            break;
        }
        mid = linear(lo, hi, rlo, rhi, n);
        if iteration % 6 == 0 {
            let close = (0.003 * (hi - lo) as f64 + 1.0) as u64;
            mid = mid.clamp(lo + close, hi - close);
        }
        mid = mid.clamp(lo + 1, hi - 1);
        iteration += 1;
    }
    if rlo == n {
        return lo;
    }
    if hi - lo <= 1 {
        return if rhi > n { lo } else { hi };
    }
    while hi - lo > 8 {
        let mut x1 = lo + (hi - lo) / 2;
        let mut rx1 = count_approx(x1);
        let d0 = (n - rlo) as f64;
        let d1 = rx1.abs_diff(n) as f64;
        let d2 = (rhi - n + 1) as f64;
        let pos = (x1 - lo) as f64 * d1 / (d1 * d1 + d0 * d2).sqrt();
        let step = (pos + 0.5) as u64;
        let mut x3 = if rx1 >= n { x1 - step } else { x1 + step };
        if x3 >= hi || x3 <= lo || x3 == x1 {
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else {
                lo = x1;
                rlo = rx1;
            }
        } else {
            let mut rx3 = count_approx(x3);
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
        mid = lo + (hi - lo) / 2;
        if count_approx(mid) < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolation_and_limits() {
        assert_eq!(nth_twin_prime_approx(4), Some(17));
        assert_eq!(nth_twin_prime_approx(5), Some(29));
        assert_eq!(nth_twin_prime_approx(u64::MAX), Some(MAX_TWIN));
        assert_eq!(linear(10, 20, 1, 3, 1), 11);
        assert_eq!(linear(10, 20, 1, 3, 3), 19);
        for n in [
            6,
            50,
            200,
            800,
            2000,
            4000,
            8000,
            20000,
            80000,
            500000,
            2000000,
            10000000,
            1000000000,
            10000000000000,
            10000000000000000,
        ] {
            let estimate = nth_twin_prime_approx(n).unwrap();
            assert!(estimate <= MAX_TWIN);
            assert!(count_approx(estimate) <= n || count_approx(estimate - 1) < n);
        }
    }
}
