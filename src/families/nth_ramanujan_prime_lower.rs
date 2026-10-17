#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use crate::{logarithmic_integral, prime_count};

pub(super) const SMALL: [u64; 23] = [
    2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227, 229, 233,
    239, 241,
];
const MAX_INDEX: u64 = 425_656_284_035_217_743;
const MAX_PRIME: u64 = 18_446_744_073_709_551_557;

// MPU prime_counts.c, ramanujan_axler.
pub(super) fn axler(n: u64, c: f64, d: f64) -> u64 {
    let n = n as f64;
    let l = n.ln();
    let ll = l.ln();
    let b = 2.0_f64.ln();
    let c1 = 2.0 * b * b + b + c;
    let c2 = b * b * b + 2.0 * b * b + d;
    let u = (b * l * ll * ll - c1 * l * ll + c2 * l - b * b * ll + b * b * b + b * b)
        / (l * l * l * l + l * l * l * ll - l * l * l * b - l * l * b);
    (2.0 * n * (1.0 + b / l - (b * ll - b * b - b) / (l * l) + u)) as u64
}

// MPU prime_counts.c, prime_count_upper. Preserve its float table.
pub(super) fn count_upper(n: u64) -> u64 {
    if n < 33_000 {
        return prime_count(n);
    }
    let x = n as f64;
    let l = x.ln();
    let l2 = l * l;
    let upper = if n <= 821_800_000 {
        const TABLE: &[(u64, f32)] = &[
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
        let a = TABLE
            .iter()
            .find(|&&(limit, _)| n < limit)
            .map_or(2.334, |&(_, a)| f64::from(a));
        x / l * (1.0 + 1.0 / l + a / l2)
    } else if n < 101_260_000_000 {
        let a = if n < 1_100_000_000 {
            0.032
        } else if n < 10_010_000_000 {
            0.027
        } else {
            0.021
        };
        logarithmic_integral(x) - a * l * x.sqrt() / (8.0 * std::f64::consts::PI)
    } else if n > 10_000_000_000_000_000 {
        extended_li(n)
    } else {
        logarithmic_integral(x)
    };
    upper.floor() as u64
}

// MPU prime_counts.c, prime_count_lower.
pub(super) fn count_lower(n: u64) -> u64 {
    if n < 33_000 {
        return prime_count(n);
    }
    let x = n as f64;
    let l = x.ln();
    let l2 = l * l;
    let lower = if n <= 300_070 {
        let a = match n {
            0..69_720 => 905.0,
            69_720..70_120 => 961.0,
            70_120..88_800 => 918.2,
            88_800..176_000 => 887.7,
            176_000..299_270 => 839.46,
            _ => 846.66,
        };
        x / (l - 1.0 - 1.0 / l - 2.975_666 / l2 - 13.024_334 / (l * l2) + a / (l2 * l2))
    } else if n < 4_000_000_000 {
        let a = match n {
            0..88_783 => 4.0,
            88_783..300_000 => -3.0,
            300_000..303_000 => 5.0,
            303_000..1_100_000 => -7.0,
            1_100_000..4_500_000 => -37.0,
            4_500_000..10_200_000 => -70.0,
            10_200_000..36_900_000 => -53.0,
            36_900_000..38_100_000 => -29.0,
            _ => -84.0,
        };
        logarithmic_integral(x) - x.sqrt() / l * (1.94 + 2.50 / l + a / l2)
    } else {
        logarithmic_integral(x) - 2.0 * x.sqrt() / l
    };
    lower.ceil() as u64
}

// MPU prime_counts.c, nth_prime_lower / nth_prime_upper.
pub(super) fn prime_bound(n: u64, upper: bool) -> u64 {
    if n >= MAX_INDEX {
        return if n == MAX_INDEX { MAX_PRIME } else { 0 };
    }
    // The C primes_small table holds primes below 1000.
    if n < 168 {
        return crate::nth_prime(n).unwrap_or(0);
    }
    let x = n as f64;
    let l = x.ln();
    let ll = l.ln();
    if n < if upper { 15_360 } else { 2_000_000 } {
        let mut lo = (x * (l + ll - 1.0)) as u64;
        let mut hi = (x * (l + ll)) as u64 + 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let count = if upper {
                count_lower(mid)
            } else {
                count_upper(mid)
            };
            if count < n {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }
    if !upper {
        let b = if n < 56_000_000 {
            11.200
        } else {
            11.508_000_000_02
        };
        return (x * (l + ll - 1.0 + (ll - 2.0) / l - (ll * ll - 6.0 * ll + b) / (2.0 * l * l)))
            .ceil() as u64;
    }
    let (c, d) = match n {
        46_254_381.. => (2.00, 10.667),
        8_009_824.. => (2.00, 10.273),
        5_450_000.. => (2.00, 10.1335),
        3_906_280.. => (1.67, 20.2675),
        2_110_840.. => (2.51, -5.5714),
        876_700.. => (2.49, -4.5129),
        688_382.. => (3.31, -26.3858),
        575_750.. => (-0.79, 83.5215),
        467_650.. => (0.93, 37.1597),
        382_440.. => (2.92, -15.4768),
        301_130.. => (5.92, -91.3415),
        138_630.. => (2.01, 7.2842),
        85_820.. => (2.07, 5.2103),
        39_016.. => (2.76, -11.5918),
        31_490.. => (1.49, 15.1821),
        25_070.. => (11.89, -197.8951),
        15_359.. => (4.80, -51.5928),
        _ => (3.92, -33.3994),
    };
    (x * (l + ll - 1.0 + (ll - c) / l - (ll * ll - 6.0 * ll + d) / (2.0 * l * l))).floor() as u64
}

/// Return a lower bound for the nth Ramanujan prime.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `nth_ramanujan_prime_lower`.
/// Return `None` for index zero or an index beyond the native range.
///
/// ```
/// assert_eq!(math_prime_util::nth_ramanujan_prime_lower(10), Some(97));
/// assert_eq!(math_prime_util::nth_ramanujan_prime_lower(0), None);
/// ```
pub fn nth_ramanujan_prime_lower(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 23 {
        return Some(SMALL[(n - 1) as usize]);
    }
    let (c, d) = if n < 34_816 {
        match n {
            0..189 => (4.2720, 0.340),
            189..1245 => (-0.2179, -6.179),
            1245..2984 => (0.1446, -4.8693),
            2984..14_303 => (-0.3570, -5.1154),
            _ => (-1.5770, -7.5332),
        }
    } else {
        (
            1.472,
            match n {
                0..76_400 => 0.0126,
                76_400..280_816 => 0.5132,
                280_816..915_887 => 0.9967,
                915_887..4_000_001 => 1.5004,
                4_000_001..16_000_001 => 1.7184,
                16_000_001..64_000_001 => 1.9860,
                64_000_001..128_000_001 => 2.1352,
                128_000_001..256_000_001 => 2.1658,
                256_000_001..384_000_001 => 2.1999,
                384_000_001..512_000_001 => 2.2047,
                512_000_001..640_000_001 => 2.2324,
                640_000_001..=14_888_378_285 => 2.2385,
                14_888_378_286..=467_037_926_604 => 2.29,
                467_037_926_605..=2_778_491_401_197 => 2.31,
                2_778_491_401_198..=10_656_144_781_918 => 2.315,
                10_656_144_781_919..=63_698_770_351_741 => 2.317,
                _ => 2.319,
            },
        )
    };
    let bound = prime_bound(axler(n, c, d), false);
    (bound != 0).then_some(bound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axler_thresholds() {
        let indices = [
            24,
            188,
            189,
            1244,
            1245,
            2983,
            2984,
            14_302,
            14_303,
            34_815,
            34_816,
            76_400,
            280_816,
            915_887,
            4_000_001,
            16_000_001,
            64_000_001,
            128_000_001,
            256_000_001,
            384_000_001,
            512_000_001,
            640_000_001,
            14_888_378_286,
            467_037_926_605,
            2_778_491_401_198,
            10_656_144_781_919,
            63_698_770_351_742,
        ];
        for n in indices {
            let lower = nth_ramanujan_prime_lower(n).unwrap();
            assert!(lower > n);
            assert!(lower <= super::super::nth_ramanujan_prime::upper(n));
        }
    }

    #[test]
    fn prime_bounds_and_count_thresholds() {
        assert_eq!(prime_bound(MAX_INDEX, false), MAX_PRIME);
        assert_eq!(prime_bound(MAX_INDEX + 1, false), 0);
        for n in [
            10,
            33_000,
            69_720,
            70_120,
            88_800,
            176_000,
            299_270,
            300_071,
            303_000,
            1_100_000,
            4_500_000,
            10_200_000,
            36_900_000,
            38_100_000,
            4_000_000_000,
            1_100_000_000,
            10_010_000_000,
            101_260_000_000,
        ] {
            assert!(count_lower(n) <= count_upper(n));
        }
        for n in [
            15_360, 25_070, 31_490, 39_016, 85_820, 138_630, 301_130, 382_440, 467_650, 575_750,
            688_382, 876_700, 2_110_840, 3_906_280, 5_450_000, 8_009_824, 46_254_381, 56_000_000,
        ] {
            assert!(prime_bound(n, false) <= prime_bound(n, true));
        }
    }
}

// MPU real.c, Li: evaluate in extended precision and return NV (double).
// Two f64 limbs preserve the long-double intermediate without platform FFI.
#[derive(Clone, Copy)]
struct Wide(f64, f64);
impl Wide {
    fn add(self, rhs: Self) -> Self {
        let s = self.0 + rhs.0;
        let v = s - self.0;
        let e = (self.0 - (s - v)) + (rhs.0 - v) + self.1 + rhs.1;
        let h = s + e;
        Self(h, e - (h - s))
    }
    fn mul(self, rhs: Self) -> Self {
        let p = self.0 * rhs.0;
        let e = self.0.mul_add(rhs.0, -p) + self.0 * rhs.1 + self.1 * rhs.0 + self.1 * rhs.1;
        let h = p + e;
        Self(h, e - (h - p))
    }
    fn div(self, rhs: Self) -> Self {
        let q = self.0 / rhs.0;
        let r = self.add(rhs.mul(Self(-q, 0.0)));
        Self(q, 0.0).add(Self((r.0 + r.1) / rhs.0, 0.0))
    }
}
fn extended_li(n: u64) -> f64 {
    let x = n as f64;
    let exponent = x.log2().floor();
    let m = Wide(x * 2.0_f64.powf(-exponent), 0.0);
    let y = m.add(Wide(-1.0, 0.0)).div(m.add(Wide(1.0, 0.0)));
    let yy = y.mul(y);
    let mut term = y;
    let mut log = y;
    for k in 1..80 {
        term = term.mul(yy);
        log = log.add(term.div(Wide(f64::from(2 * k + 1), 0.0)));
    }
    log = log
        .mul(Wide(2.0, 0.0))
        .add(Wide(std::f64::consts::LN_2, 2.319_046_813_846_299_6e-17).mul(Wide(exponent, 0.0)));
    let mut term = log;
    let mut sum = term;
    for k in 2..400 {
        term = term.mul(log).div(Wide(f64::from(k), 0.0));
        let part = term.div(Wide(f64::from(k), 0.0));
        sum = sum.add(part);
        if part.0.abs() < 1e-18 {
            break;
        }
    }
    sum = sum.add(Wide(0.577_215_664_901_532_9 + log.0.ln(), 0.0));
    sum.0 + sum.1
}
