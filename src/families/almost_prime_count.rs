#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::unreadable_literal
)]
const UPPER_20: [f64; 13] = [
    0.0, 0.0, 1.006, 0.7385, 0.6830, 0.5940, 0.3596, 0.2227, 0.1439, 0.09785, 0.07016, 0.05303,
    0.04202,
];
const UPPER_32: [f64; 21] = [
    0.0, 0.0, 1.013, 0.8094, 0.7485, 0.6467, 0.3984, 0.2464, 0.1572, 0.1049, 0.07364, 0.05452,
    0.04266, 0.03542, 0.03082, 0.02798, 0.02642, 0.02585, 0.02615, 0.02808, 0.03054,
];
const UPPER_64: [f64; 41] = [
    0.0, 0.0, 1.028, 1.028, 1.3043, 0.72208, 0.46609, 0.29340, 0.18571, 0.12063, 0.0815, 0.0575,
    0.0427, 0.03490, 0.03007, 0.02710, 0.02554, 0.02504, 0.02554, 0.02699, 0.02954, 0.03294,
    0.03779, 0.04453, 0.05393, 0.06703, 0.08543, 0.1117, 0.1494, 0.205, 0.287, 0.410, 0.60, 0.90,
    1.36, 2.12, 3.35, 5.38, 8.83, 14.75, 25.07,
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
const OFFSET: [u64; 64] = [
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
    83967,
    16383,
    111615,
    557055,
    2097151,
    524287,
    65535,
    1048575,
    6553599,
    33554431,
    4194303,
    671088639,
    16777215,
    536870911,
    2684354559,
    2147483647,
    25769803775,
    4294967295,
    268435455,
    206158430207,
    137438953471,
    17179869183,
    68719476735,
    2199023255551,
    5428838662143,
    21990232555519,
    4398046511103,
    1099511627775,
    100055558127615,
    10995116277759,
    17592186044415,
    545357767376895,
    70368744177663,
    1125899906842623,
    10133099161583615,
    9007199254740991,
    24769797950537727,
    4503599627370495,
    144115188075855871,
    72057594037927935,
    432345564227567615,
    288230376151711743,
    1152921504606846975,
    4611686018427387903,
];
const MAX: [u64; 64] = [
    1,
    425656284035217743,
    1701748900850019777,
    3167597434038354478,
    3787884015050788482,
    3378907169603895030,
    2466706950238087748,
    1571012171387856192,
    913164427599983727,
    499840874923678341,
    263157990621533964,
    135128109904869290,
    68283616225825256,
    34151861008771016,
    16967424859951587,
    8393048221327186,
    4139595949113890,
    2037655246635364,
    1001591348315641,
    491808604962296,
    241293656953012,
    118304122014405,
    57968649799947,
    28388662714236,
    13895161400556,
    6797526392535,
    3323560145881,
    1624109166018,
    793189260998,
    387148515886,
    188844769357,
    92054377509,
    44841620426,
    21827124353,
    10616326552,
    5159281045,
    2505087309,
    1215204383,
    588891145,
    285076316,
    137840686,
    66567488,
    32103728,
    15460810,
    7433670,
    3567978,
    1709640,
    817053,
    389954,
    185387,
    87993,
    41604,
    19611,
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

use crate::{prime_count, primes, rootint, semiprime_count};
use std::collections::HashMap;

pub(super) fn max_nth(k: u64) -> u64 {
    u64::MAX - OFFSET[k as usize]
}
pub(super) fn max_count(k: u64) -> u64 {
    MAX[k as usize]
}

// almost_primes.c: reduce_prime_count_factor.
pub(super) fn reduce(k: &mut u64, n: &mut u64) {
    let mut r = k.saturating_sub(40);
    while (*n >> r) < 3_u64.pow((*k - r) as u32) {
        r += 1;
    }
    *k -= r;
    *n >>= r;
}

// almost_primes.c: _cs and _final_sum. Store repeated prime counts locally.
fn count_sum(k: u64, n: u64, lo: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    let mut sum = 0;
    for p in primes(lo, rootint(n, k as u32)) {
        if k == 2 {
            let count = *cache.entry(n / p).or_insert_with(|| prime_count(n / p));
            let index = *cache.entry(p - 1).or_insert_with(|| prime_count(p - 1));
            sum += count - index;
        } else {
            sum += count_sum(k - 1, n / p, p, cache);
        }
    }
    sum
}

/// Count integers at most `n` with `k` prime factors, with repetition.
///
/// Port of MPU 0.75 `almost_primes.c`: `almost_prime_count`, `_cs`, and `_final_sum`.
/// ```
/// use math_prime_util::almost_prime_count;
/// assert_eq!(almost_prime_count(3, 30), 7);
/// ```
pub fn almost_prime_count(mut k: u64, mut n: u64) -> u64 {
    if k == 0 {
        return u64::from(n >= 1);
    }
    if k >= 64 || n >> k == 0 {
        return 0;
    }
    reduce(&mut k, &mut n);
    if n >= max_nth(k) {
        return max_count(k);
    }
    match k {
        0 => n,
        1 => prime_count(n),
        2 => semiprime_count(n),
        _ => count_sum(k, n, 2, &mut HashMap::new()),
    }
}

pub(super) fn bounds(mut k: u64, mut n: u64) -> (u64, u64) {
    if k >= 64 || n >> k == 0 {
        return (0, 0);
    }
    reduce(&mut k, &mut n);
    if k == 0 {
        let v = u64::from(n >= 1);
        return (v, v);
    }
    if k == 1 {
        return (crate::prime_count_lower(n), crate::prime_count_upper(n));
    }
    if let Some(v) = small(k, n) {
        return (v, v);
    }
    let max = max_count(k);
    if n >= max_nth(k) {
        return (max, max);
    }
    let x = n as f64;
    let logx = x.ln();
    let loglogx = logx.ln();
    let mut logplus = loglogx + 0.26153;
    let i = k as usize;
    let (mut ml, mut mu) = if n <= 1048575 {
        (LOWER_20[i], UPPER_20[i])
    } else if n <= 4294967295 {
        (LOWER_32[i], UPPER_32[i])
    } else {
        (LOWER_64[i], UPPER_64[i])
    };
    let (mut lower, mut upper);
    if k == 2 {
        lower = x * (loglogx + 0.261536) / logx;
        upper = lower;
        if x >= 1e12 {
            lower = x * (loglogx + 0.1769) / logx * (1.0 + 0.4232 / logx);
            ml = 1.0;
        }
    } else if k == 3 {
        upper = x * (logplus * logplus + 1.055852) / (2.0 * logx);
        lower = x * loglogx * loglogx / (2.0 * logx);
        ml = if n < 638 {
            0.8418
        } else if n <= 1926 {
            let w = (x - 638.0) / (1926.0 - 638.0);
            (1.0 - w) * 0.8939 + w * 0.9233
        } else if n <= 500194 {
            let w = (x - 1927.0) / (500194.0 - 1927.0);
            (1.0 - w) * 0.9233 + w
        } else if n <= 3184393786 {
            let w = (x - 500194.0) / (3184393786.0 - 500194.0);
            (1.0 - w) + w * 1.039
        } else {
            1.0004
        };
        if n > 4294967295 {
            mu = 0.8711;
        }
    } else if k == 4 {
        lower = x * logplus * logplus * logplus / (6.0 * logx);
        upper = lower;
        if x > 1e12 {
            upper += 0.511977 * x * ((x / 4.0).ln().ln() + 0.261536) / logx;
            mu = 1.028;
        }
        if n > 4294967295 {
            mu = 0.780;
        }
        if x > 1e12 {
            mu = 0.6921;
        }
    } else {
        lower = x / logx;
        logplus = loglogx + ((k as f64).ln() * (k as f64).ln().ln() - 0.504377);
        for j in 1..k {
            lower *= logplus / (j as f64);
        }
        upper = lower;
    }
    lower *= ml;
    upper *= mu;
    let l = if lower >= u64::MAX as f64 || lower > max as f64 {
        max
    } else {
        lower as u64
    };
    let u = if upper >= u64::MAX as f64 || upper > max as f64 {
        max
    } else {
        (upper + 1.0) as u64
    };
    (l, u)
}

pub(super) fn small(k: u64, n: u64) -> Option<u64> {
    let n = u128::from(n);
    if n < 3 * (1_u128 << (k - 1)) {
        Some(1)
    } else if n < 9 * (1_u128 << (k - 2)) {
        Some(2)
    } else if n < 10 * (1_u128 << (k - 2)) {
        Some(3)
    } else {
        None
    }
}
