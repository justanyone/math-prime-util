#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use crate::{logarithmic_integral, nth_prime, prime_count, riemann_r, rootint};

pub(super) const MAX_INDEX: u64 = 425_656_284_035_217_743;
pub(super) const MAX_PRIME: u64 = 18_446_744_073_709_551_557;

// MPU prime_counts.c: prime_count_lower and prime_count_upper.
fn count_bound(n: u64, upper: bool) -> u64 {
    if n < 33_000 {
        return prime_count(n);
    }
    let x = n as f64;
    let l = x.ln();
    let l2 = l * l;
    if upper {
        if n <= 821_800_000 {
            let a = UPPER
                .iter()
                .find(|&&(t, _)| n < t)
                .map_or(2.334, |&(_, a)| f64::from(a));
            (x / l * (1.0 + 1.0 / l + a / l2)).floor() as u64
        } else {
            let a = if n < 1_100_000_000 {
                0.032
            } else if n < 10_010_000_000 {
                0.027
            } else if n < 101_260_000_000 {
                0.021
            } else {
                0.0
            };
            (logarithmic_integral(x) - a * l * x.sqrt() / 25.132_741_228_718_345).floor() as u64
        }
    } else {
        let v = if n <= 300_070 {
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
            x / (l - 1.0 - 1.0 / l - 2.975_666 / l2 - 13.024_334 / (l * l2) + a / (l2 * l2))
        } else if n < 4_000_000_000 {
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
            logarithmic_integral(x) - x.sqrt() / l * (1.94 + 2.50 / l + a / l2)
        } else {
            logarithmic_integral(x) - 2.0 * x.sqrt() / l
        };
        v.ceil() as u64
    }
}

// MPU prime_powers.c: prime_power_count and its three estimates.
pub(super) fn count(n: u64, kind: u8) -> u64 {
    if n <= 5 {
        return n.saturating_sub(1);
    }
    let pc = |x| match kind {
        0 => prime_count(x),
        1 => count_bound(x, true),
        2 => count_bound(x, false),
        _ => {
            if x < 3_000_000 {
                prime_count(x)
            } else {
                (riemann_r(x as f64) + 0.5) as u64
            }
        }
    };
    pc(n) + (2..=n.ilog2()).map(|k| pc(rootint(n, k))).sum::<u64>()
}

// MPU prime_counts.c: simple_nth_limits and nth_prime_lower/upper.
fn prime_bound(n: u64, upper: bool) -> u64 {
    if n < SMALL_LEN {
        return nth_prime(n).unwrap_or(0);
    }
    if n >= MAX_INDEX {
        return if n == MAX_INDEX { MAX_PRIME } else { 0 };
    }
    let x = n as f64;
    let l = x.ln();
    let ll = l.ln();
    if (upper && n < 15_360) || (!upper && n < 2_000_000) {
        let a = if n < 228 {
            0.6483
        } else if n < 948 {
            0.8032
        } else if n < 2195 {
            0.8800
        } else if n < 39017 {
            0.9019
        } else {
            0.9484
        };
        let mut lo = (x * (l + ll - 1.0 + (ll - 2.10) / l)) as u64;
        let mut hi = (x * (l + ll - a)) as u64;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if count_bound(mid, !upper) < n {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }
    if upper {
        let (_, c, d) = NTH_UPPER
            .iter()
            .find(|&&(t, _, _)| n >= t)
            .unwrap_or(&(0, 3.92, -33.3994));
        (x * (l + ll - 1.0 + (ll - c) / l - (ll * ll - 6.0 * ll + d) / (2.0 * l * l)))
            .floor()
            .min(MAX_PRIME as f64) as u64
    } else {
        let b = if n < 56_000_000 {
            11.200
        } else {
            11.508_000_000_02
        };
        (x * (l + ll - 1.0 + (ll - 2.0) / l - (ll * ll - 6.0 * ll + b) / (2.0 * l * l)))
            .ceil()
            .min(MAX_PRIME as f64) as u64
    }
}

pub(super) fn estimate(n: u64, kind: u8) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 7 {
        return Some(n + 1 + n / 5);
    }
    if n > MAX_INDEX {
        return None;
    }
    let lo = if n <= 100 {
        n + 1
    } else {
        (0.98 * prime_bound(n, false) as f64) as u64 - 400
    };
    Some(inverse(lo, prime_bound(n, true), n, |x| count(x, kind)))
}

/// Estimate the `n`th prime power. Index one is 2.
/// Returns `None` for zero or an index beyond the native prime limit.
/// Port of MPU 0.75 `prime_powers.c`, `nth_prime_power_approx`.
/// ```
/// assert_eq!(math_prime_util::nth_prime_power_approx(10), Some(16));
/// ```
pub fn nth_prime_power_approx(n: u64) -> Option<u64> {
    estimate(n, 3)
}

const UPPER: &[(u64, f32)] = &[
    (59000, 2.48),
    (355991, 2.54),
    (3550000, 2.51),
    (3560000, 2.49),
    (5000000, 2.48),
    (8000000, 2.47),
    (13000000, 2.46),
    (18000000, 2.45),
    (31000000, 2.44),
    (41000000, 2.43),
    (48000000, 2.42),
    (119000000, 2.41),
    (182000000, 2.40),
    (192000000, 2.395),
    (213000000, 2.390),
    (271000000, 2.385),
    (322000000, 2.380),
    (400000000, 2.375),
    (510000000, 2.370),
    (682000000, 2.367),
    (2953652287, 2.362),
];
const NTH_UPPER: &[(u64, f64, f64)] = &[
    (46254381, 2.00, 10.667),
    (8009824, 2.00, 10.273),
    (688382, 2.30, 0.5730),
    (5450000, 2.00, 10.1335),
    (3906280, 1.67, 20.2675),
    (2110840, 2.51, -5.5714),
    (876700, 2.49, -4.5129),
    (688382, 3.31, -26.3858),
    (575750, -0.79, 83.5215),
    (467650, 0.93, 37.1597),
    (382440, 2.92, -15.4768),
    (301130, 5.92, -91.3415),
    (138630, 2.01, 7.2842),
    (85820, 2.07, 5.2103),
    (39016, 2.76, -11.5918),
    (31490, 1.49, 15.1821),
    (25070, 11.89, -197.8951),
    (15359, 4.80, -51.5928),
];
// MPU inverse_interpolate.c: _linear_interpolate and _inverse_interpolate.
fn linear(lo: u64, hi: u64, rlo: u64, rhi: u64, n: u64) -> u64 {
    if n == rlo {
        return lo + 1;
    }
    if n == rhi {
        return hi - 1;
    }
    let span = hi - lo;
    let movement = ((n - rlo) as f64 * span as f64 / (rhi - rlo) as f64 + 0.5) as u64;
    lo + movement.clamp(1, span - 1)
}
fn inverse(mut lo: u64, mut hi: u64, n: u64, f: impl Fn(u64) -> u64) -> u64 {
    let mut mid = lo + (hi - lo) / 2;
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
    mid = linear(lo, hi, rlo, rhi, n);
    let mut iteration = 1;
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
        if iteration % 6 == 0 {
            let close = (0.003 * (hi - lo) as f64 + 1.0) as u64;
            if lo + close > mid {
                mid = lo + close;
            } else if hi - close < mid {
                mid = hi - close;
            }
        }
        mid = mid.clamp(lo + 1, hi - 1);
        iteration += 1;
    }
    if rlo == n {
        return lo;
    }
    if hi - lo <= 1 {
        return if rlo == n || rhi > n { lo } else { hi };
    }
    while hi - lo > 8 {
        let mut x1 = lo + (hi - lo) / 2;
        let mut rx1 = f(x1);
        let d0 = (n - rlo) as f64;
        let d1 = rx1.abs_diff(n) as f64;
        let d2 = (rhi - n + 1) as f64;
        let sign = if rx1 >= n { 1.0 } else { -1.0 };
        let pos = sign * (x1 - lo) as f64 * d1 / (d1 * d1 + d0 * d2).sqrt();
        let movement = (pos + if pos >= 0.0 { 0.5 } else { -0.5 }) as i128;
        let candidate = i128::from(x1) - movement;
        if candidate <= i128::from(lo) || candidate >= i128::from(hi) || candidate == i128::from(x1)
        {
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else {
                lo = x1;
                rlo = rx1;
            }
        } else {
            let mut x3 = candidate as u64;
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
        mid = lo + (hi - lo) / 2;
        if f(mid) < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}

const SMALL_LEN: u64 = 306;
