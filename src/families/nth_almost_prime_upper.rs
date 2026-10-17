#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use crate::{prime_count_lower, semiprime_count};

/// Return an upper bound for the nth number with `k` prime factors.
///
/// Count factors with multiplicity. Return `None` if the result cannot fit.
/// Port of MPU 0.75 `almost_primes.c`, `nth_almost_prime_upper`.
///
/// ```
/// use math_prime_util::nth_almost_prime_upper;
/// assert_eq!(nth_almost_prime_upper(3, 456), Some(1830));
/// assert_eq!(nth_almost_prime_upper(0, 2), None);
/// ```
pub fn nth_almost_prime_upper(k: u32, n: u64) -> Option<u64> {
    if n == 0 || k >= 64 {
        return None;
    }
    if k == 0 {
        return (n == 1).then_some(1);
    }
    if k == 1 {
        return prime_upper(n);
    }
    let maxn = u64::MAX - OFFSETS[k as usize];
    let maxc = MAX_COUNTS[k as usize];
    if n >= maxc {
        return (n == maxc).then_some(maxn);
    }
    if n < 8 {
        let semi = [0, 4, 6, 9, 10, 14, 15, 21];
        let mult = [0, 8, 12, 18, 20, 27, 28, 30];
        return Some(if k == 2 {
            semi[n as usize]
        } else {
            mult[n as usize] << (k - 3)
        });
    }
    let mut r = if k > 49 && n < FIRST_THREE[49] {
        k - 48
    } else {
        0
    };
    if k <= 49 || r > 0 {
        while n < FIRST_THREE[(k - r) as usize] {
            r += 1;
        }
    }
    if r > 0 {
        let up = nth_almost_prime_upper(k - r, n)?;
        return Some(if up > maxn || up > (u64::MAX >> r) {
            maxn
        } else {
            up << r
        });
    }
    interpolate(5 << k, 0, n, 0, |x| count_lower(k, x)).map(|up| up.min(maxn))
}

fn count_lower(mut k: u32, mut n: u64) -> u64 {
    if k == 2 && n < 256 {
        return semiprime_count(n);
    }
    if k >= 64 || n >> k == 0 {
        return 0;
    }
    let mut r = k.saturating_sub(40);
    while k >= r && u128::from(n >> r) < 3_u128.pow(k - r) {
        r += 1;
    }
    k -= r;
    n >>= r;
    if k == 0 {
        return u64::from(n >= 1);
    }
    if k == 1 {
        return prime_count_lower(n);
    }

    if n < 3 * (1 << (k - 1)) {
        return 1;
    }
    if n < 9 * (1 << (k - 2)) {
        return 2;
    }
    if n < 10 * (1 << (k - 2)) {
        return 3;
    }
    let max = MAX_COUNTS[k as usize];
    if n >= u64::MAX - OFFSETS[k as usize] {
        return max;
    }
    let x = n as f64;
    let l = x.ln();
    let ll = l.ln();
    let mut mult = if n <= 1_048_575 {
        LOWER_20[k as usize]
    } else if n <= 4_294_967_295 {
        LOWER_32[k as usize]
    } else {
        LOWER_64[k as usize]
    };
    let bound = match k {
        2 => {
            if x >= 1e12 {
                mult = 1.0;
                x * (ll + 0.1769) / l * (1.0 + 0.4232 / l)
            } else {
                x * (ll + 0.261536) / l
            }
        }
        3 => {
            mult = if n < 638 {
                0.8418
            } else if n <= 1926 {
                let w = (x - 638.0) / 1288.0;
                (1.0 - w) * 0.8939 + w * 0.9233
            } else if n <= 500_194 {
                let w = (x - 1927.0) / 498_267.0;
                (1.0 - w) * 0.9233 + w
            } else if n <= 3_184_393_786 {
                let w = (x - 500_194.0) / 3_183_893_592.0;
                (1.0 - w) + w * 1.039
            } else {
                1.0004
            };
            x * ll * ll / (2.0 * l)
        }
        4 => x * (ll + 0.26153).powi(3) / (6.0 * l),
        _ => {
            let fk = f64::from(k);
            let plus = ll + (fk.ln() * fk.ln().ln() - 0.504377);
            let mut b = x / l;
            for i in 1..k {
                b *= plus / f64::from(i);
            }
            b
        }
    };
    ((bound * mult) as u64).min(max)
}
const FIRST_THREE: [u64; 50] = [
    1,
    2,
    3,
    5,
    8,
    14,
    23,
    39,
    64,
    103,
    169,
    269,
    427,
    676,
    1065,
    1669,
    2628,
    4104,
    6414,
    10_023,
    15_608,
    24_281,
    37_733,
    58_503,
    90_616,
    140_187,
    216_625,
    334_527,
    516_126,
    795_632,
    1_225_641,
    1_886_570,
    2_901_796,
    4_460_359,
    6_851_532,
    10_518_476,
    16_138_642,
    24_748_319,
    37_932_129,
    58_110_457,
    88_981_343,
    136_192_537,
    208_364_721,
    318_653_143,
    487_128_905,
    744_398_307,
    1_137_129_971,
    1_736_461_477,
    2_650_785_552,
    4_045_250_962,
];
const LOWER_20: [f64; 13] = [
    0.0, 0.0, 0.8197, 0.8418, 0.5242, 0.5154, 0.3053, 0.1901, 0.1253, 0.0892, 0.06551, 0.05082,
    0.04101,
];
const LOWER_32: [f64; 21] = [
    0.0, 0.0, 1.004, 0.7383, 0.6828, 0.5939, 0.3594, 0.2222, 0.1438, 0.09754, 0.06981, 0.05245,
    0.04151, 0.03461, 0.03006, 0.02709, 0.02553, 0.02502, 0.02552, 0.02697, 0.02945,
];
const LOWER_64: [f64; 41] = [
    0.0, 0.0, 1.011, 0.8093, 0.7484, 0.6465, 0.3982, 0.2463, 0.1571, 0.1048, 0.07363, 0.0545,
    0.0422, 0.0331, 0.0270, 0.0232, 0.0208, 0.0194, 0.0190, 0.0193, 0.0203, 0.0222, 0.0252, 0.0295,
    0.0356, 0.0444, 0.0570, 0.0753, 0.102, 0.14, 0.20, 0.297, 0.44, 0.68, 1.07, 1.71, 2.8, 4.7,
    8.0, 13.89, 23.98,
];
const OFFSETS: [u64; 64] = [
    u64::MAX - 1,
    58,
    14,
    2,
    4,
    3,
    17,
    0,
    1,
    195,
    51,
    127,
    63,
    767,
    1535,
    511,
    255,
    15,
    8191,
    1023,
    83_967,
    16_383,
    111_615,
    557_055,
    2_097_151,
    524_287,
    65_535,
    1_048_575,
    6_553_599,
    33_554_431,
    4_194_303,
    671_088_639,
    16_777_215,
    536_870_911,
    2_684_354_559,
    2_147_483_647,
    25_769_803_775,
    4_294_967_295,
    268_435_455,
    206_158_430_207,
    137_438_953_471,
    17_179_869_183,
    68_719_476_735,
    2_199_023_255_551,
    5_428_838_662_143,
    21_990_232_555_519,
    4_398_046_511_103,
    1_099_511_627_775,
    100_055_558_127_615,
    10_995_116_277_759,
    17_592_186_044_415,
    545_357_767_376_895,
    70_368_744_177_663,
    1_125_899_906_842_623,
    10_133_099_161_583_615,
    9_007_199_254_740_991,
    24_769_797_950_537_727,
    4_503_599_627_370_495,
    144_115_188_075_855_871,
    72_057_594_037_927_935,
    432_345_564_227_567_615,
    288_230_376_151_711_743,
    1_152_921_504_606_846_975,
    4_611_686_018_427_387_903,
];
const MAX_COUNTS: [u64; 64] = [
    1,
    425_656_284_035_217_743,
    1_701_748_900_850_019_777,
    3_167_597_434_038_354_478,
    3_787_884_015_050_788_482,
    3_378_907_169_603_895_030,
    2_466_706_950_238_087_748,
    1_571_012_171_387_856_192,
    913_164_427_599_983_727,
    499_840_874_923_678_341,
    263_157_990_621_533_964,
    135_128_109_904_869_290,
    68_283_616_225_825_256,
    34_151_861_008_771_016,
    16_967_424_859_951_587,
    8_393_048_221_327_186,
    4_139_595_949_113_890,
    2_037_655_246_635_364,
    1_001_591_348_315_641,
    491_808_604_962_296,
    241_293_656_953_012,
    118_304_122_014_405,
    57_968_649_799_947,
    28_388_662_714_236,
    13_895_161_400_556,
    6_797_526_392_535,
    3_323_560_145_881,
    1_624_109_166_018,
    793_189_260_998,
    387_148_515_886,
    188_844_769_357,
    92_054_377_509,
    44_841_620_426,
    21_827_124_353,
    10_616_326_552,
    5_159_281_045,
    2_505_087_309,
    1_215_204_383,
    588_891_145,
    285_076_316,
    137_840_686,
    66_567_488,
    32_103_728,
    15_460_810,
    7_433_670,
    3_567_978,
    1_709_640,
    817_053,
    389_954,
    185_387,
    87_993,
    41_604,
    19_611,
    9184,
    4283,
    2001,
    914,
    421,
    187,
    84,
    37,
    15,
    7,
    2,
];
const NTH_UPPER: &[(u64, f64, f64)] = &[
    (46_254_381, 2.00, 10.667),
    (8_009_824, 2.00, 10.273),
    (688_382, 2.30, 0.5730),
    (5_450_000, 2.00, 10.1335),
    (3_906_280, 1.67, 20.2675),
    (2_110_840, 2.51, -5.5714),
    (876_700, 2.49, -4.5129),
    (688_382, 3.31, -26.3858),
    (575_750, -0.79, 83.5215),
    (467_650, 0.93, 37.1597),
    (382_440, 2.92, -15.4768),
    (301_130, 5.92, -91.3415),
    (138_630, 2.01, 7.2842),
    (85_820, 2.07, 5.2103),
    (39_016, 2.76, -11.5918),
    (31_490, 1.49, 15.1821),
    (25_070, 11.89, -197.8951),
    (15_359, 4.80, -51.5928),
];
const SMALL_LEN: u64 = 306;
const MAX_INDEX: u64 = 425_656_284_035_217_743;
const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
fn prime_upper(n: u64) -> Option<u64> {
    let value = prime_bound(n, true);
    (value != 0).then_some(value)
}

fn prime_bound(n: u64, upper: bool) -> u64 {
    if n < SMALL_LEN {
        return crate::nth_prime(n).unwrap_or(0);
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
        } else if n < 39_017 {
            0.9019
        } else {
            0.9484
        };
        let mut lo = (x * (l + ll - 1.0 + (ll - 2.10) / l)) as u64;
        let mut hi = (x * (l + ll - a)) as u64;
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

// MPU inverse_interpolate.c: _linear_interpolate and _inverse_interpolate.
fn linear(lo: u64, hi: u64, rlo: u64, rhi: u64, n: u64) -> u64 {
    if n == rlo {
        return lo + 1;
    }
    if n == rhi {
        return hi - 1;
    }
    let movement = ((n - rlo) as f64 * (hi - lo) as f64 / (rhi - rlo) as f64 + 0.5) as u64;
    lo + movement.clamp(1, hi - lo - 1)
}

pub(super) fn interpolate(
    mut lo: u64,
    mut hi: u64,
    n: u64,
    threshold: u64,
    f: impl Fn(u64) -> u64,
) -> Option<u64> {
    let (mut rlo, mut rhi);
    if hi != 0 {
        let mid = lo + (hi - lo) / 2;
        let rmid = f(mid);
        if rmid >= n {
            hi = mid;
            rhi = rmid;
            rlo = f(lo);
            if rlo == n {
                return Some(lo);
            }
        } else {
            lo = mid;
            rlo = rmid;
            rhi = f(hi);
        }
    } else {
        rlo = f(lo);
        if rlo == n {
            return Some(lo);
        }
        loop {
            let scale = (n as f64 / rlo as f64 - 0.004).clamp(1.004, 8.0);
            let mid = if u64::MAX as f64 / lo as f64 <= scale {
                u64::MAX
            } else {
                (scale * lo as f64 + 1.0) as u64
            };
            let rmid = f(mid);
            if rmid >= n {
                hi = mid;
                rhi = rmid;
                break;
            }
            lo = mid;
            rlo = rmid;
            if lo == u64::MAX {
                return None;
            }
        }
    }
    if hi - lo <= 1 {
        return Some(if rlo == n || rhi > n { lo } else { hi });
    }
    let mut mid = linear(lo, hi, rlo, rhi, n);
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
        return Some(lo);
    }
    if hi - lo <= 1 {
        return Some(if rhi > n { lo } else { hi });
    }
    while hi - lo > 8 && (hi - lo > threshold || rhi > n) {
        let mut x1 = lo + (hi - lo) / 2;
        let mut rx1 = f(x1);
        let d0 = (n - rlo) as f64;
        let d1 = rx1.abs_diff(n) as f64;
        let d2 = (rhi - n + 1) as f64;
        let sign = if rx1 >= n { 1.0 } else { -1.0 };
        let pos = sign * ((x1 - lo) as f64 * d1) / (d1 * d1 + d0 * d2).sqrt();
        let mut x3 = if pos >= 0.0 {
            x1.saturating_sub((pos + 0.5) as u64)
        } else {
            x1.saturating_add((-pos + 0.5) as u64)
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
    while hi - lo > 1 && (hi - lo > threshold || rhi > n) {
        let mid = lo + (hi - lo) / 2;
        if f(mid) < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(hi)
}
