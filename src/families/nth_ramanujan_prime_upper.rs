//! Upper bound for the nth Ramanujan prime.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

/// Ramanujan primes `R_1` through `R_23`. `ramanujan_primes.c` `small_ram_primes`.
const SMALL_RAM: [u64; 23] = [
    2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227, 229, 233,
    239, 241,
];

/// `constants.h` `MPU_MAX_PRIME` on a 64-bit build.
const MPU_MAX_PRIME: u64 = u64::MAX - 58;
/// `constants.h` `MPU_MAX_PRIME_IDX` on a 64-bit build.
const MPU_MAX_PRIME_IDX: u64 = 425_656_284_035_217_743;

/// Return an upper bound for the `n`th Ramanujan prime.
///
/// Index zero has no value. Port of MPU 0.75 `ramanujan_primes.c`,
/// `nth_ramanujan_prime_upper`. The bound is Axler 2017, then
/// `nth_prime_upper` from `prime_counts.c`.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_ramanujan_prime_upper;
/// assert_eq!(nth_ramanujan_prime_upper(0), None);
/// assert_eq!(nth_ramanujan_prime_upper(1), Some(2));
/// assert_eq!(nth_ramanujan_prime_upper(5), Some(41));
/// ```
#[must_use]
pub fn nth_ramanujan_prime_upper(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= SMALL_RAM.len() as u64 {
        return Some(SMALL_RAM[n as usize - 1]);
    }
    if n < 12_581 {
        let r = if n < 168 {
            ramanujan_axler(n, -4.7691, -6.2682)
        } else if n < 2_290 {
            ramanujan_axler(n, -0.9315, -0.5635)
        } else if n < 5_225 {
            ramanujan_axler(n, -0.5318, -0.0710)
        } else {
            ramanujan_axler(n, 0.1212, 0.7973)
        };
        return checked_bound(trunc_u64(r));
    }
    checked_bound(trunc_u64(ramanujan_axler(n, 0.0, axler_d(n))))
}

/// `nth_prime_upper` for an Axler index. `None` when the index is past
/// `MPU_MAX_PRIME_IDX`: `nth_prime_upper` returns 0 there, and 0 bounds no prime.
fn checked_bound(idx: u64) -> Option<u64> {
    (idx <= MPU_MAX_PRIME_IDX).then(|| nth_prime_upper(idx))
}

/// `prime_counts.c` `ramanujan_axler`. The `C` code narrows the result to `double`.
fn ramanujan_axler(n: u64, c: f64, d: f64) -> f64 {
    let n = n as f64;
    let log2 = 2.0_f64.ln();
    let logn = n.ln();
    let loglogn = logn.ln();
    let c1 = 2.0 * log2 * log2 + log2 + c;
    let c2 = log2 * log2 * log2 + 2.0 * log2 * log2 + d;
    let logn2 = logn * logn;
    let logn3 = logn2 * logn;
    let logn4 = logn2 * logn2;
    let numerator = log2 * logn * loglogn * loglogn - c1 * logn * loglogn + c2 * logn
        - log2 * log2 * loglogn
        + log2 * log2 * log2
        + log2 * log2;
    let denominator = logn4 + logn3 * loglogn - logn3 * log2 - logn2 * log2;
    let u = numerator / denominator;
    2.0 * n * (1.0 + log2 / logn - (log2 * loglogn - log2 * log2 - log2) / logn2 + u)
}

fn axler_d(n: u64) -> f64 {
    if n < 18_175 {
        0.3548
    } else if n < 82_883 {
        -0.2450
    } else if n < 316_314 {
        -0.6384
    } else if n < 1_000_001 {
        -0.9353
    } else if n < 4_000_001 {
        -1.1271
    } else if n < 16_000_001 {
        -1.4152
    } else if n < 64_000_001 {
        -1.6671
    } else if n < 128_000_001 {
        -1.8855
    } else if n < 256_000_001 {
        -1.9325
    } else if n < 384_000_001 {
        -2.0190
    } else if n < 512_000_001 {
        -2.0310
    } else {
        axler_d_tail(n)
    }
}

fn axler_d_tail(n: u64) -> f64 {
    if n > 117_462_814_829_787 {
        -2.8
    } else if n > 28_330_126_673_435 {
        -2.735
    } else if n > 3_594_243_587_299 {
        -2.635
    } else if n > 514_808_375_201 {
        -2.535
    } else if n > 84_086_679_236 {
        -2.435
    } else if n > 3_999_654_659 {
        -2.235
    } else {
        -2.0884
    }
}

/// `prime_counts.c` `nth_prime_upper` for the index that Axler returns.
fn nth_prime_upper(n: u64) -> u64 {
    if n < 306 {
        return if n == 0 {
            0
        } else {
            crate::nth_prime(n).unwrap_or(0)
        };
    }
    if n > MPU_MAX_PRIME_IDX {
        return 0;
    }
    if n == MPU_MAX_PRIME_IDX {
        return MPU_MAX_PRIME;
    }
    let fn_ = n as f64;
    let flogn = fn_.ln();
    let flog2n = flogn.ln();
    if n < 15_360 {
        let (mut lo, mut hi) = simple_nth_limits(fn_, flogn, flog2n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if prime_count_lower(mid) < n {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }
    let (c, d) = nth_prime_cd(n);
    let upper = fn_
        * (flogn + flog2n - 1.0 + ((flog2n - c) / flogn)
            - (flog2n * flog2n - 6.0 * flog2n + d) / (2.0 * flogn * flogn));
    if !upper.is_finite() || upper >= u64::MAX as f64 {
        return MPU_MAX_PRIME;
    }
    trunc_u64(upper)
}

/// `prime_counts.c` `simple_nth_limits`. Reachable for `306 <= n < 15_360`.
fn simple_nth_limits(n: f64, logn: f64, loglogn: f64) -> (u64, u64) {
    let a = if n < 948.0 {
        0.8032
    } else if n < 2_195.0 {
        0.8800
    } else {
        0.9019
    };
    let lo = trunc_u64(n * (logn + loglogn - 1.0 + ((loglogn - 2.10) / logn)));
    let mut hi = trunc_u64(n * (logn + loglogn - a));
    if hi < lo {
        hi = MPU_MAX_PRIME;
    }
    (lo, hi)
}

fn nth_prime_cd(n: u64) -> (f64, f64) {
    if n >= 46_254_381 {
        (2.00, 10.667)
    } else if n >= 8_009_824 {
        (2.00, 10.273)
    } else if n >= 5_450_000 {
        (2.00, 10.1335)
    } else if n >= 3_906_280 {
        (1.67, 20.2675)
    } else if n >= 2_110_840 {
        (2.51, -5.5714)
    } else if n >= 876_700 {
        (2.49, -4.5129)
    } else if n >= 688_382 {
        (3.31, -26.3858)
    } else if n >= 575_750 {
        (-0.79, 83.5215)
    } else if n >= 467_650 {
        (0.93, 37.1597)
    } else if n >= 382_440 {
        (2.92, -15.4768)
    } else if n >= 301_130 {
        (5.92, -91.3415)
    } else if n >= 138_630 {
        (2.01, 7.2842)
    } else if n >= 85_820 {
        (2.07, 5.2103)
    } else if n >= 39_016 {
        (2.76, -11.5918)
    } else if n >= 31_490 {
        (1.49, 15.1821)
    } else if n >= 25_070 {
        (11.89, -197.8951)
    } else if n >= 15_359 {
        (4.80, -51.5928)
    } else {
        (3.92, -33.3994)
    }
}

/// `prime_counts.c` `prime_count_lower` on the range the binary search uses.
fn prime_count_lower(n: u64) -> u64 {
    if n < 33_000 {
        return crate::prime_count(n);
    }
    let fn_ = n as f64;
    let fl1 = fn_.ln();
    let fl2 = fl1 * fl1;
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
    let lower = fn_
        / (fl1 - 1.0 - 1.0 / fl1 - 2.975_666 / fl2 - 13.024_334 / (fl1 * fl2) + a / (fl2 * fl2));
    ceil_u64(lower)
}

fn trunc_u64(x: f64) -> u64 {
    if !x.is_finite() || x <= 0.0 {
        0
    } else if x >= u64::MAX as f64 {
        u64::MAX
    } else {
        x as u64
    }
}

fn ceil_u64(x: f64) -> u64 {
    if !x.is_finite() || x <= 0.0 {
        0
    } else if x >= u64::MAX as f64 {
        u64::MAX
    } else {
        x.ceil() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_index_matches_the_table_and_zero_is_empty() {
        assert_eq!(nth_ramanujan_prime_upper(0), None);
        for (i, &prime) in SMALL_RAM.iter().enumerate() {
            assert_eq!(nth_ramanujan_prime_upper(i as u64 + 1), Some(prime));
        }
        assert_eq!(nth_ramanujan_prime_upper(1), nth_ramanujan_prime_upper(1));
    }

    #[test]
    fn axler_bands_and_tail_return_a_positive_bound() {
        let samples = [
            24,
            168,
            2_290,
            5_225,
            12_581,
            18_175,
            82_883,
            316_314,
            1_000_001,
            4_000_001,
            16_000_001,
            64_000_001,
            128_000_001,
            256_000_001,
            384_000_001,
            512_000_001,
            3_999_654_659,
            3_999_654_660,
            84_086_679_236,
            84_086_679_237,
            514_808_375_201,
            514_808_375_202,
            3_594_243_587_299,
            3_594_243_587_300,
            28_330_126_673_435,
            28_330_126_673_436,
            117_462_814_829_787,
            117_462_814_829_788,
        ];
        let mut previous = 0_u64;
        for n in samples {
            let bound = nth_ramanujan_prime_upper(n).expect("n > 0");
            assert!(bound > previous / 2, "n={n} bound={bound}");
            previous = bound;
        }
    }

    #[test]
    fn prime_upper_table_search_formula_and_ends() {
        assert_eq!(nth_prime_upper(0), 0);
        assert_eq!(nth_prime_upper(1), 2);
        assert_eq!(nth_prime_upper(305), crate::nth_prime(305).unwrap_or(0));
        assert!(nth_prime_upper(306) >= 2000);
        assert!(nth_prime_upper(947) <= nth_prime_upper(948));
        assert!(nth_prime_upper(2_194) <= nth_prime_upper(2_195));
        assert!(nth_prime_upper(15_359) <= nth_prime_upper(15_360));
        let knees = [
            25_070, 31_490, 39_016, 85_820, 138_630, 301_130, 382_440, 467_650, 575_750, 688_382,
            876_700, 2_110_840, 3_906_280, 5_450_000, 8_009_824, 46_254_381,
        ];
        for n in knees {
            let bound = nth_prime_upper(n);
            assert!(bound > n, "n={n} bound={bound}");
        }
        assert_eq!(nth_prime_upper(MPU_MAX_PRIME_IDX), MPU_MAX_PRIME);
        assert_eq!(nth_prime_upper(MPU_MAX_PRIME_IDX + 1), 0);
        let wide = nth_prime_upper(400_000_000_000_000_000);
        assert!(wide == MPU_MAX_PRIME || wide > 0);
    }

    #[test]
    fn past_max_index_has_no_bound() {
        // Public boundary: a 0 sentinel from `nth_prime_upper` must become `None`, not `Some(0)`.
        assert_eq!(nth_ramanujan_prime_upper(u64::MAX), None);
        assert_eq!(nth_ramanujan_prime_upper(u64::MAX / 2), None);
        assert_eq!(checked_bound(MPU_MAX_PRIME_IDX), Some(MPU_MAX_PRIME));
        assert_eq!(checked_bound(MPU_MAX_PRIME_IDX + 1), None);
    }

    #[test]
    fn count_lower_hits_each_coefficient() {
        assert_eq!(prime_count_lower(10), crate::prime_count(10));
        assert_eq!(prime_count_lower(1), 0);
        let knees = [33_000, 69_720, 70_120, 88_800, 176_000, 299_270, 300_071];
        for n in knees {
            let lower = prime_count_lower(n);
            assert!(lower > 0, "n={n}");
            assert!(lower <= n, "n={n} lower={lower}");
        }
    }

    #[test]
    fn limits_swap_uses_the_max_prime() {
        // A large `loglogn` makes the low estimate pass the high estimate.
        let (lo, hi) = simple_nth_limits(10.0, 0.5, 20.0);
        assert!(lo > 196, "lo={lo}");
        assert_eq!(hi, MPU_MAX_PRIME);
    }

    #[test]
    fn non_finite_casts_are_zero() {
        assert_eq!(trunc_u64(f64::NAN), 0);
        assert_eq!(trunc_u64(-1.0), 0);
        assert_eq!(trunc_u64(u64::MAX as f64 * 2.0), u64::MAX);
        assert_eq!(ceil_u64(f64::NAN), 0);
        assert_eq!(ceil_u64(-3.2), 0);
        assert_eq!(ceil_u64(u64::MAX as f64 * 2.0), u64::MAX);
        assert_eq!(ceil_u64(1.1), 2);
    }
}
