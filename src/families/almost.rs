//! Almost primes from MPU 0.75.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::unreadable_literal,
    clippy::excessive_precision
)]
use crate::{prime_count, primes, rootint};
const FIRST_3: [u64; 50] = [
    1, 2, 3, 5, 8, 14, 23, 39, 64, 103, 169, 269, 427, 676, 1065, 1669, 2628, 4104, 6414, 10023,
    15608, 24281, 37733, 58503, 90616, 140187, 216625, 334527, 516126, 795632, 1225641, 1886570,
    2901796, 4460359, 6851532, 10518476, 16138642, 24748319, 37932129, 58110457, 88981343,
    136192537, 208364721, 318653143, 487128905, 744398307, 1137129971, 1736461477, 2650785552,
    4045250962,
];
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
const MAX_COUNTS: [u64; 64] = [
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

fn reduce(mut k: u32, mut n: u64) -> (u32, u64) {
    while k > 0 && u128::from(n) < 3u128.pow(k.min(64)) {
        k -= 1;
        n >>= 1;
    }
    (k, n)
}

// almost_primes.c: _cs and _final_sum. Primes are in nondecreasing order.
fn count_rec(k: u32, n: u64, start: u64) -> u64 {
    if k == 1 {
        return prime_count(n).saturating_sub(prime_count(start - 1));
    }
    primes(start, rootint(n, k))
        .into_iter()
        .map(|p| count_rec(k - 1, n / p, p))
        .sum()
}

/// Count integers at most `n` with `k` prime factors, with repetition.
/// Port of MPU `almost_primes.c`, `almost_prime_count` and `_cs`.
/// ```
/// assert_eq!(math_prime_util::almost_prime_count(3, 30), 7);
/// ```
pub fn almost_prime_count(k: u32, n: u64) -> u64 {
    if k == 0 {
        return u64::from(n >= 1);
    }
    if k >= 64 || n >> k == 0 {
        return 0;
    }
    if n >= u64::MAX - OFFSETS[k as usize] {
        return MAX_COUNTS[k as usize];
    }
    let (k, n) = reduce(k, n);
    if k == 0 {
        return n;
    }
    count_rec(k, n, 2)
}

// almost_primes.c: _genkap, with unique ordered prime factors.
fn generate(k: u32, lo: u64, hi: u64, m: u64, start: u64, out: &mut Vec<u64>) {
    if k == 1 {
        out.extend(
            primes(start.max(lo.div_ceil(m)), hi / m)
                .into_iter()
                .map(|p| m * p),
        );
    } else {
        for p in primes(start, rootint(hi / m, k)) {
            generate(k - 1, lo, hi, m * p, p, out);
        }
    }
}

/// List integers in the closed range with `k` prime factors, with repetition.
/// Port of MPU `almost_primes.c`, `_genkap` and `range_almost_prime_sieve`.
/// ```
/// assert_eq!(math_prime_util::almost_primes(2, 4, 10), vec![4, 6, 9, 10]);
/// ```
pub fn almost_primes(k: u32, lo: u64, hi: u64) -> Vec<u64> {
    if lo > hi {
        return Vec::new();
    }
    if k == 0 {
        return if lo <= 1 && hi >= 1 {
            vec![1]
        } else {
            Vec::new()
        };
    }
    if k >= 64 || hi >> k == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    generate(k, lo, hi, 1, 2, &mut out);
    out.sort_unstable();
    out
}

fn small_nth(k: u32, n: u64) -> u64 {
    if k == 2 {
        [0, 4, 6, 9, 10, 14, 15, 21][n as usize]
    } else {
        [0u64, 8, 12, 18, 20, 27, 28, 30][n as usize]
            .checked_shl(k - 3)
            .unwrap_or(0)
    }
}

fn nth_reduce(k: u32, n: u64) -> u32 {
    let mut r = k.saturating_sub(48);
    while k - r > 0 && n < FIRST_3[(k - r) as usize] {
        r += 1;
    }
    r
}

// inverse_interpolate.c: inverse_interpolate_k, zero threshold.
fn inverse(mut lo: u64, n: u64, f: impl Fn(u64) -> u64) -> u64 {
    let mut hi = lo.max(2);
    while f(hi) < n {
        if hi == u64::MAX {
            return 0;
        }
        hi = hi.saturating_mul(2);
    }
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if f(mid) < n {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if f(lo) > n { lo.saturating_sub(1) } else { lo }
}

fn nth(k: u32, n: u64, kind: u8) -> Option<u64> {
    if n == 0 || k >= 64 {
        return None;
    }
    if k == 0 {
        return (n == 1).then_some(1);
    }
    if n >= MAX_COUNTS[k as usize] {
        return (n == MAX_COUNTS[k as usize]).then_some(u64::MAX - OFFSETS[k as usize]);
    }
    if k == 1 {
        return crate::nth_prime(n);
    }
    if k == 2 && kind == 3 && n < 206 {
        return nth(k, n, 0);
    }
    if n < 8 {
        return Some(small_nth(k, n));
    }
    let r = nth_reduce(k, n);
    if r > 0 && kind != 3 {
        return nth(k - r, n, kind).and_then(|v| v.checked_mul(1u64 << r));
    }
    let lo = 1u64 << k;
    let value = inverse(lo, n, |x| match kind {
        1 => almost_prime_count_upper(k, x),
        2 => almost_prime_count_lower(k, x),
        3 => almost_prime_count_approx(k, x),
        _ => almost_prime_count(k, x),
    });
    (value != 0).then_some(value)
}

/// Return the exact value for the `n`th integer with `k` prime factors.
/// Return `None` if the index is zero or the result exceeds `u64`.
/// Port of MPU `almost_primes.c`, `nth_almost_prime`.
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime(3, 4), Some(20));
/// ```
pub fn nth_almost_prime(k: u32, n: u64) -> Option<u64> {
    nth(k, n, 0)
}

/// Return the lower bound for the `n`th integer with `k` prime factors.
/// Return `None` if the index is zero or the result exceeds `u64`.
/// Port of MPU `almost_primes.c`, `nth_almost_prime_lower`.
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime_lower(3, 4), Some(20));
/// ```
pub fn nth_almost_prime_lower(k: u32, n: u64) -> Option<u64> {
    nth(k, n, 1)
}

/// Return the upper bound for the `n`th integer with `k` prime factors.
/// Return `None` if the index is zero or the result exceeds `u64`.
/// Port of MPU `almost_primes.c`, `nth_almost_prime_upper`.
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime_upper(3, 4), Some(20));
/// ```
pub fn nth_almost_prime_upper(k: u32, n: u64) -> Option<u64> {
    nth(k, n, 2)
}

/// Return the estimate for the `n`th integer with `k` prime factors.
/// Return `None` if the index is zero or the result exceeds `u64`.
/// Port of MPU `almost_primes.c`, `nth_almost_prime_approx`.
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime_approx(3, 4), Some(20));
/// ```
pub fn nth_almost_prime_approx(k: u32, n: u64) -> Option<u64> {
    nth(k, n, 3)
}

fn prime_approx(n: u64) -> u64 {
    if n >= (1u64 << 53) {
        return super::precision::prime_approx(n);
    }
    if n < 3_000_000 {
        prime_count(n)
    } else {
        (crate::riemann_r(n as f64) + 0.5) as u64
    }
}

// almost_primes.c: _almost_prime_count_bounds, including the empirical tables.
#[allow(clippy::too_many_lines)]
fn bounds(k: u32, n: u64) -> (u64, u64) {
    if k >= 64 || n >> k == 0 {
        return (0, 0);
    }
    let (k, n) = reduce(k, n);
    if k == 0 {
        return (u64::from(n >= 1), u64::from(n >= 1));
    }
    if k == 1 {
        if n >= (1u64 << 53) {
            return (
                super::precision::prime_bound(n, false),
                super::precision::prime_bound(n, true),
            );
        }
        return (
            super::nth_prime_power_approx::count_bound(n, false),
            super::nth_prime_power_approx::count_bound(n, true),
        );
    }
    let x = n as f64;
    if x < 3.0 * 2f64.powf(f64::from(k) - 1.0) {
        return (1, 1);
    }
    if x < 9.0 * 2f64.powf(f64::from(k) - 2.0) {
        return (2, 2);
    }
    if x < 10.0 * 2f64.powf(f64::from(k) - 2.0) {
        return (3, 3);
    }
    let max = MAX_COUNTS[k as usize];
    if n >= u64::MAX - OFFSETS[k as usize] {
        return (max, max);
    }
    let l = x.ln();
    let ll = l.ln();
    let lp = ll + 0.26153;
    let (mut ml, mut mu) = if n <= 1_048_575 {
        (LOWER_20[k as usize], UPPER_20[k as usize])
    } else if n <= 4_294_967_295 {
        (LOWER_32[k as usize], UPPER_32[k as usize])
    } else {
        (LOWER_64[k as usize], UPPER_64[k as usize])
    };
    let (bl, bu) = match k {
        2 => {
            let b = x * (ll + 0.261536) / l;
            if x >= 1e12 {
                ml = 1.0;
                (x * (ll + 0.1769) / l * (1.0 + 0.4232 / l), b)
            } else {
                (b, b)
            }
        }
        3 => {
            ml = if n < 638 {
                0.8418
            } else if n <= 1926 {
                let w = (x - 638.0) / (1926.0 - 638.0);
                (1.0 - w) * 0.8939 + w * 0.9233
            } else if n <= 500_194 {
                let w = (x - 1927.0) / (500194.0 - 1927.0);
                (1.0 - w) * 0.9233 + w
            } else if n <= 3_184_393_786 {
                let w = (x - 500194.0) / (3184393786.0 - 500194.0);
                (1.0 - w) + w * 1.039
            } else {
                1.0004
            };
            if n > 4_294_967_295 {
                mu = 0.8711;
            }
            (
                x * ll * ll / (2.0 * l),
                x * (lp * lp + 1.055852) / (2.0 * l),
            )
        }
        4 => {
            let b = x * lp * lp * lp / (6.0 * l);
            let mut u = b;
            if x > 1e12 {
                u += 0.511977 * x * ((x / 4.0).ln().ln() + 0.261536) / l;
                mu = 1.028;
            }
            if n > 4_294_967_295 {
                mu = 0.780;
            }
            if x > 1e12 {
                mu = 0.6921;
            }
            (b, u)
        }
        _ => {
            let lp = ll + ((f64::from(k)).ln() * (f64::from(k)).ln().ln() - 0.504377);
            let mut b = x / l;
            for i in 1..k {
                b *= lp / f64::from(i);
            }
            (b, b)
        }
    };
    (
        ((bl * ml) as u64).min(max),
        ((bu * mu + 1.0) as u64).min(max),
    )
}

/// Return the lower bound on the count of integers with `k` prime factors.
/// Port of MPU `almost_primes.c`, `almost_prime_count_lower`.
/// ```
/// assert_eq!(math_prime_util::almost_prime_count_lower(2, 100), 34);
/// ```
pub fn almost_prime_count_lower(k: u32, n: u64) -> u64 {
    if k == 2 && n < 256 {
        almost_prime_count(k, n)
    } else {
        bounds(k, n).0
    }
}

/// Return the upper bound on the count of integers with `k` prime factors.
/// Port of MPU `almost_primes.c`, `almost_prime_count_upper`.
/// ```
/// assert_eq!(math_prime_util::almost_prime_count_upper(2, 100), 34);
/// ```
pub fn almost_prime_count_upper(k: u32, n: u64) -> u64 {
    if k == 2 && n < 256 {
        almost_prime_count(k, n)
    } else {
        bounds(k, n).1
    }
}

/// Estimate the count of integers with `k` prime factors.
/// Port of MPU `almost_primes.c`, `almost_prime_count_approx`.
/// ```
/// assert_eq!(math_prime_util::almost_prime_count_approx(3, 30), 7);
/// ```
pub fn almost_prime_count_approx(k: u32, n: u64) -> u64 {
    if k == 0 {
        return u64::from(n >= 1);
    }
    if k >= 64 || n >> k == 0 {
        return 0;
    }
    let (k, n) = reduce(k, n);
    if k == 1 {
        return prime_approx(n);
    }
    if k == 2 {
        return semi_approx(n);
    }
    if k == 0 {
        return n;
    }
    let x = n as f64;
    if x < 3.0 * 2f64.powf(f64::from(k) - 1.0) {
        return 1;
    }
    if x < 9.0 * 2f64.powf(f64::from(k) - 2.0) {
        return 2;
    }
    if x < 10.0 * 2f64.powf(f64::from(k) - 2.0) {
        return 3;
    }
    if k == 3 && n < 102 {
        return 4 + [
            27, 28, 30, 42, 44, 45, 50, 52, 63, 66, 68, 70, 75, 76, 78, 92, 98, 99, 102,
        ]
        .partition_point(|&v| v <= n) as u64;
    }
    let (lo, hi) = bounds(k, n);
    if k == 3 {
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
        let ll = x.ln().ln() + 0.26153;
        return ((0.5 * a * x * ll * ll / (x.ln() + s) + 0.5) as u64).clamp(lo, hi);
    }
    (lo as f64 + (hi - lo) as f64 * if k > 11 { 0.20 } else { 0.76 }) as u64
}

// semi_primes.c: semiprime_count_approx, Crisan and Erban series.
fn semi_approx(n: u64) -> u64 {
    const SEMI: [u64; 191] = [
        0, 4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35, 38, 39, 46, 49, 51, 55, 57, 58, 62, 65,
        69, 74, 77, 82, 85, 86, 87, 91, 93, 94, 95, 106, 111, 115, 118, 119, 121, 122, 123, 129,
        133, 134, 141, 142, 143, 145, 146, 155, 158, 159, 161, 166, 169, 177, 178, 183, 185, 187,
        194, 201, 202, 203, 205, 206, 209, 213, 214, 215, 217, 218, 219, 221, 226, 235, 237, 247,
        249, 253, 254, 259, 262, 265, 267, 274, 278, 287, 289, 291, 295, 298, 299, 301, 302, 303,
        305, 309, 314, 319, 321, 323, 326, 327, 329, 334, 335, 339, 341, 346, 355, 358, 361, 362,
        365, 371, 377, 381, 382, 386, 391, 393, 394, 395, 398, 403, 407, 411, 413, 415, 417, 422,
        427, 437, 445, 446, 447, 451, 453, 454, 458, 466, 469, 471, 473, 478, 481, 482, 485, 489,
        493, 497, 501, 502, 505, 511, 514, 515, 517, 519, 526, 527, 529, 533, 535, 537, 538, 542,
        543, 545, 551, 553, 554, 559, 562, 565, 566, 573, 579, 581, 583, 586, 589, 591, 597, 611,
        614, 622, 623,
    ];
    const COEFF: [f64; 19] = [
        0.26149721284764278375,
        -2.0710850628855780875,
        -7.6972777412176108802,
        -35.345660320564161516,
        -206.71503925406509339,
        -1511.1997871316530251,
        -13546.323682845914021,
        -146229.10675883565523,
        -1867579.6280076650637,
        -27733045.258413542557,
        -470983423.57703294361,
        -9011500983.75,
        -191744069149.4,
        -4487573459710.5,
        -114472069580579.8,
        -3158610502077135.6,
        -93682567786528911.9,
        -2970838770257639695.3,
        -100274471240063911725.1,
    ];
    const CROSS: [f64; 15] = [
        632.0,
        9385.0,
        136411.0,
        4610076.0,
        66358000.0,
        440590000.0,
        2557200000.0,
        53032001000.0,
        1151076796431.0,
        20416501389724.0,
        165815501587300.0,
        953038830319448.0,
        20019396133340433.0,
        192558867109258424.0,
        1757883874953032448.0,
    ];
    const MINCOUNT: [f64; 16] = [
        82.0,
        195.0,
        2485.0,
        31446.0,
        906319.0,
        11741185.0,
        72840337.0,
        398702652.0,
        7538564737.0,
        150382042176.0,
        2482510001499.0,
        19204997230933.0,
        106211451717048.0,
        2094735089989940.0,
        19282342825922188.0,
        168996486318315136.0,
    ];

    if n <= 623 {
        return (SEMI.partition_point(|&v| v <= n) - 1) as u64;
    }
    let l = (n as f64).ln();
    let ll = l.ln();
    let terms = 3 + CROSS.partition_point(|&v| n as f64 >= v);
    let mut series = 0.0;
    let mut den = 1.0;
    let mut fac = 1.0;
    for (i, &c) in COEFF.iter().take(terms).enumerate() {
        if i > 0 {
            fac *= i as f64;
        }
        series += fac * (ll / den);
        series += c / den;
        den *= l;
    }
    (((n as f64 / l) * series + 0.5) as u64)
        .min(MAX_COUNTS[2])
        .max(MINCOUNT[terms - 3] as u64)
}
