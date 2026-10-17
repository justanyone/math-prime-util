#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use crate::{logarithmic_integral, nth_prime, prime_count, primes};

pub(super) const SMALL: [u64; 23] = [
    2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227, 229, 233,
    239, 241,
];

/// Return the Ramanujan primes in the inclusive interval.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `ramanujan_primes` and
/// `n_range_ramanujan_primes`. This uses Noe's sieve.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_primes(10, 30), vec![11, 17, 29]);
/// ```
#[allow(clippy::missing_panics_doc)]
pub fn ramanujan_primes(low: u64, high: u64) -> Vec<u64> {
    if high < 2 || low > high {
        return Vec::new();
    }
    if high <= 241 {
        return SMALL
            .into_iter()
            .filter(|&p| p >= low && p <= high)
            .collect();
    }
    let first = super::ramanujan_prime_count_lower(low).max(1);
    // R_n lies between p_(2n) and p_(3n).
    let mut lo = count_bound(high, false) / 3;
    let mut hi = count_bound(high, true) / 2;
    let last = if nth_bound(lo, false) > high {
        lo
    } else if nth_bound(hi, false) < high {
        hi
    } else {
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            if nth_bound(mid, false) <= high {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    };
    range(first, last)
        .into_iter()
        .filter(|&p| p >= low && p <= high)
        .collect()
}

// MPU 0.75 ramanujan_primes.c::n_range_ramanujan_primes.
pub(super) fn range(first: u64, last: u64) -> Vec<u64> {
    if first > last {
        return Vec::new();
    }
    if last <= 23 {
        return SMALL[(first - 1) as usize..last as usize].to_vec();
    }
    let mut result = vec![0; usize::try_from(last - first + 1).expect("Ramanujan sieve size")];
    for (i, &p) in SMALL[..2].iter().enumerate() {
        let n = i as u64 + 1;
        if n >= first && n <= last {
            result[(n - first) as usize] = p;
        }
    }
    let mut start = if first == 1 {
        7
    } else {
        nth_bound(first, false).saturating_sub(1).max(15)
    };
    if start % 2 == 0 {
        start -= 1;
    }
    let end = nth_bound(last, true).saturating_add(1);
    let ps = primes(start, end);
    let halves = primes(start.div_ceil(2), end.div_ceil(2));
    let mut pi = 0;
    let mut qi = 0;
    let mut s = if first == 1 {
        2
    } else {
        1 + prime_count(start - 2) - prime_count((start - 1) / 2)
    };
    for k in (start..=end).step_by(2) {
        if ps.get(pi) == Some(&k) {
            s += 1;
            pi += 1;
        }
        if s >= first && s <= last {
            result[(s - first) as usize] = k + 1;
        }
        if k & 3 == 1 && halves.get(qi) == Some(&k.div_ceil(2)) {
            s -= 1;
            qi += 1;
        }
        if s >= first && s <= last {
            result[(s - first) as usize] = k + 2;
        }
    }
    result
}

fn axler(n: u64, c: f64, d: f64) -> u64 {
    let x = n as f64;
    let a = 2_f64.ln();
    let l = x.ln();
    let ll = l.ln();
    let c1 = 2.0 * a * a + a + c;
    let c2 = a * a * a + 2.0 * a * a + d;
    let u = (a * l * ll * ll - c1 * l * ll + c2 * l - a * a * ll + a * a * a + a * a)
        / (l * l * l * l + l * l * l * ll - l * l * l * a - l * l * a);
    (2.0 * x * (1.0 + a / l - (a * ll - a * a - a) / (l * l) + u)) as u64
}

pub(super) fn nth_bound(n: u64, upper: bool) -> u64 {
    if n <= 23 {
        return if n == 0 { 0 } else { SMALL[(n - 1) as usize] };
    }
    let (c, d) = if upper {
        if n < 12_581 {
            let (c, d) = select(
                n,
                &[
                    (168, -4.7691, -6.2682),
                    (2290, -0.9315, -0.5635),
                    (5225, -0.5318, -0.0710),
                    (12_581, 0.1212, 0.7973),
                ],
            );
            return prime_nth_bound(axler(n, c, d), true);
        }
        let (_, mut d) = select(
            n,
            &[
                (18_175, 0.0, 0.3548),
                (82_883, 0.0, -0.2450),
                (316_314, 0.0, -0.6384),
                (1_000_001, 0.0, -0.9353),
                (4_000_001, 0.0, -1.1271),
                (16_000_001, 0.0, -1.4152),
                (64_000_001, 0.0, -1.6671),
                (128_000_001, 0.0, -1.8855),
                (256_000_001, 0.0, -1.9325),
                (384_000_001, 0.0, -2.0190),
                (512_000_001, 0.0, -2.0310),
                (u64::MAX, 0.0, -2.0884),
            ],
        );
        for (t, v) in [
            (3_999_654_659, -2.235),
            (84_086_679_236, -2.435),
            (514_808_375_201, -2.535),
            (3_594_243_587_299, -2.635),
            (28_330_126_673_435, -2.735),
            (117_462_814_829_787, -2.8),
        ] {
            if n > t {
                d = v;
            }
        }
        (0.0, d)
    } else {
        if n < 34_816 {
            let (c, d) = select(
                n,
                &[
                    (189, 4.2720, 0.340),
                    (1245, -0.2179, -6.179),
                    (2984, 0.1446, -4.8693),
                    (14_303, -0.3570, -5.1154),
                    (34_816, -1.5770, -7.5332),
                ],
            );
            return prime_nth_bound(axler(n, c, d), false);
        }
        let (_, mut d) = select(
            n,
            &[
                (76_400, 0.0, 0.0126),
                (280_816, 0.0, 0.5132),
                (915_887, 0.0, 0.9967),
                (4_000_001, 0.0, 1.5004),
                (16_000_001, 0.0, 1.7184),
                (64_000_001, 0.0, 1.9860),
                (128_000_001, 0.0, 2.1352),
                (256_000_001, 0.0, 2.1658),
                (384_000_001, 0.0, 2.1999),
                (512_000_001, 0.0, 2.2047),
                (640_000_001, 0.0, 2.2324),
                (u64::MAX, 0.0, 2.2385),
            ],
        );
        for (t, v) in [
            (14_888_378_285, 2.29),
            (467_037_926_604, 2.31),
            (2_778_491_401_197, 2.315),
            (10_656_144_781_918, 2.317),
            (63_698_770_351_741, 2.319),
        ] {
            if n > t {
                d = v;
            }
        }
        (1.472, d)
    };
    prime_nth_bound(axler(n, c, d), upper)
}

fn select(n: u64, table: &[(u64, f64, f64)]) -> (f64, f64) {
    let &(_, c, d) = table
        .iter()
        .find(|&&(t, _, _)| n < t)
        .unwrap_or(&table[table.len() - 1]);
    (c, d)
}

// MPU 0.75 prime_counts.c::nth_prime_upper / nth_prime_lower.
#[allow(clippy::items_after_statements)]
fn prime_nth_bound(n: u64, upper: bool) -> u64 {
    if n < 306 {
        return nth_prime(n).unwrap_or(0);
    }
    const MAX_INDEX: u64 = 425_656_284_035_217_743;
    const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
    if n >= MAX_INDEX {
        return if n == MAX_INDEX { MAX_PRIME } else { 0 };
    }
    let x = n as f64;
    let l = x.ln();
    let ll = l.ln();
    if n < if upper { 15_360 } else { 2_000_000 } {
        let a = if n < 948 {
            0.8032
        } else if n < 2195 {
            0.8800
        } else if n < 39_017 {
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
    let (c, d) = if upper {
        select(
            n,
            &[
                (25_070, 4.80, -51.5928),
                (31_490, 11.89, -197.8951),
                (39_016, 1.49, 15.1821),
                (85_820, 2.76, -11.5918),
                (138_630, 2.07, 5.2103),
                (301_130, 2.01, 7.2842),
                (382_440, 5.92, -91.3415),
                (467_650, 2.92, -15.4768),
                (575_750, 0.93, 37.1597),
                (688_382, -0.79, 83.5215),
                (876_700, 3.31, -26.3858),
                (2_110_840, 2.49, -4.5129),
                (3_906_280, 2.51, -5.5714),
                (5_450_000, 1.67, 20.2675),
                (8_009_824, 2.0, 10.1335),
                (46_254_381, 2.0, 10.273),
                (u64::MAX, 2.0, 10.667),
            ],
        )
    } else {
        (
            2.0,
            if n < 56_000_000 {
                11.200
            } else {
                11.508_000_000_02
            },
        )
    };
    let v = x * (l + ll - 1.0 + (ll - c) / l - (ll * ll - 6.0 * ll + d) / (2.0 * l * l));
    if upper {
        (v.floor() as u64).min(MAX_PRIME)
    } else {
        (v.ceil() as u64).min(MAX_PRIME)
    }
}

// MPU 0.75 prime_counts.c::prime_count_lower / prime_count_upper.
pub(super) fn count_bound(n: u64, upper: bool) -> u64 {
    if n < 33_000 {
        return prime_count(n);
    }
    let x = n as f64;
    let l = x.ln();
    let l2 = l * l;
    let v = if upper {
        if n <= 821_800_000 {
            let (a, _) = select(
                n,
                &[
                    (59_000, f64::from(2.48_f32), 0.0),
                    (355_991, f64::from(2.54_f32), 0.0),
                    (3_550_000, f64::from(2.51_f32), 0.0),
                    (3_560_000, f64::from(2.49_f32), 0.0),
                    (5_000_000, f64::from(2.48_f32), 0.0),
                    (8_000_000, f64::from(2.47_f32), 0.0),
                    (13_000_000, f64::from(2.46_f32), 0.0),
                    (18_000_000, f64::from(2.45_f32), 0.0),
                    (31_000_000, f64::from(2.44_f32), 0.0),
                    (41_000_000, f64::from(2.43_f32), 0.0),
                    (48_000_000, f64::from(2.42_f32), 0.0),
                    (119_000_000, f64::from(2.41_f32), 0.0),
                    (182_000_000, f64::from(2.40_f32), 0.0),
                    (192_000_000, f64::from(2.395_f32), 0.0),
                    (213_000_000, f64::from(2.390_f32), 0.0),
                    (271_000_000, f64::from(2.385_f32), 0.0),
                    (322_000_000, f64::from(2.380_f32), 0.0),
                    (400_000_000, f64::from(2.375_f32), 0.0),
                    (510_000_000, f64::from(2.370_f32), 0.0),
                    (682_000_000, f64::from(2.367_f32), 0.0),
                    (2_953_652_287, f64::from(2.362_f32), 0.0),
                ],
            );
            x / l * (1.0 + 1.0 / l + a / l2)
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
            logarithmic_integral(x) - a * l * x.sqrt() / (8.0 * std::f64::consts::PI)
        }
    } else if n <= 300_070 {
        let (a, _) = select(
            n,
            &[
                (69_720, 905.0, 0.0),
                (70_120, 961.0, 0.0),
                (88_800, 918.2, 0.0),
                (176_000, 887.7, 0.0),
                (299_270, 839.46, 0.0),
                (300_071, 846.66, 0.0),
            ],
        );
        x / (l - 1.0 - 1.0 / l - 2.975_666 / l2 - 13.024_334 / (l * l2) + a / (l2 * l2))
    } else {
        let (a, _) = select(
            n,
            &[
                (303_000, 5.0, 0.0),
                (1_100_000, -7.0, 0.0),
                (4_500_000, -37.0, 0.0),
                (10_200_000, -70.0, 0.0),
                (36_900_000, -53.0, 0.0),
                (38_100_000, -29.0, 0.0),
                (4_000_000_000, -84.0, 0.0),
            ],
        );
        logarithmic_integral(x)
            - x.sqrt() / l
                * if n < 4_000_000_000 {
                    1.94 + 2.50 / l + a / l2
                } else {
                    2.0
                }
    };
    if upper {
        v.floor() as u64
    } else {
        v.ceil() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_bound_edges() {
        assert!(range(3, 2).is_empty());
        assert_eq!(nth_bound(0, true), 0);
        assert_eq!(
            prime_nth_bound(425_656_284_035_217_743, true),
            18_446_744_073_709_551_557
        );
        assert_eq!(prime_nth_bound(u64::MAX, false), 0);
        for n in [
            24,
            168,
            189,
            1245,
            2290,
            2984,
            5225,
            12581,
            14303,
            18175,
            34816,
            76400,
            82883,
            280816,
            316314,
            915887,
            1_000_001,
            4_000_001,
            16_000_001,
            64_000_001,
            128_000_001,
            256_000_001,
            384_000_001,
            512_000_001,
            640_000_001,
            10_000_000_000,
            1_000_000_000_000,
            100_000_000_000_000,
        ] {
            assert!(nth_bound(n, false) <= nth_bound(n, true));
        }
        for n in [
            33_000,
            69_720,
            70_120,
            88_800,
            176_000,
            299_270,
            300_070,
            300_071,
            1_000_000,
            10_000_000,
            1_000_000_000,
            10_000_000_000,
            1_000_000_000_000,
        ] {
            assert!(count_bound(n, false) <= count_bound(n, true));
        }
    }
}
