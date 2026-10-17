//! Almost primes from MPU 0.75.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use crate::{prime_count, primes, rootint};
const FIRST_3: [u64; 50] = [1,2,3,5,8,14,23,39,64,103,169,269,427,676,1065,1669,2628,4104,6414,10023,15608,24281,37733,58503,90616,140187,216625,334527,516126,795632,1225641,1886570,2901796,4460359,6851532,10518476,16138642,24748319,37932129,58110457,88981343,136192537,208364721,318653143,487128905,744398307,1137129971,1736461477,2650785552,4045250962];
const UPPER_20: [f64; 13] = [0.0,0.0,1.006,0.7385,0.6830,0.5940,0.3596,0.2227,0.1439,0.09785,0.07016,0.05303,0.04202];
const UPPER_32: [f64; 21] = [0.0,0.0,1.013,0.8094,0.7485,0.6467,0.3984,0.2464,0.1572,0.1049,0.07364,0.05452,0.04266,0.03542,0.03082,0.02798,0.02642,0.02585,0.02615,0.02808,0.03054];
const UPPER_64: [f64; 41] = [0.0,0.0,1.028,1.028,1.3043,0.72208,0.46609,0.29340,0.18571,0.12063,0.0815,0.0575,0.0427,0.03490,0.03007,0.02710,0.02554,0.02504,0.02554,0.02699,0.02954,0.03294,0.03779,0.04453,0.05393,0.06703,0.08543,0.1117,0.1494,0.205,0.287,0.410,0.60,0.90,1.36,2.12,3.35,5.38,8.83,14.75,25.07];
const LOWER_20: [f64; 13] = [0.0,0.0,0.8197,0.8418,0.5242,0.5154,0.3053,0.1901,0.1253,0.0892,0.06551,0.05082,0.04101];
const LOWER_32: [f64; 21] = [0.0,0.0,1.004,0.7383,0.6828,0.5939,0.3594,0.2222,0.1438,0.09754,0.06981,0.05245,0.04151,0.03461,0.03006,0.02709,0.02553,0.02502,0.02552,0.02697,0.02945];
const LOWER_64: [f64; 41] = [0.0,0.0,1.011,0.8093,0.7484,0.6465,0.3982,0.2463,0.1571,0.1048,0.07363,0.0545,0.0422,0.0331,0.0270,0.0232,0.0208,0.0194,0.0190,0.0193,0.0203,0.0222,0.0252,0.0295,0.0356,0.0444,0.0570,0.0753,0.102,0.14,0.20,0.297,0.44,0.68,1.07,1.71,2.8,4.7,8.0,13.89,23.98];
const OFFSETS: [u64; 64] = [u64::MAX-1,58,14,2,4,
      3,17,0,1,195,51,127,63,
      767,1535,511,255,15,8191,1023,83967,16383,111615,
      557055,2097151,524287,65535,1048575,6553599,33554431,4194303,671088639,16777215,
      536870911,2684354559,2147483647,
    25769803775,4294967295,268435455,
    206158430207,137438953471,17179869183,
    68719476735,2199023255551,5428838662143,
    21990232555519,4398046511103,1099511627775,
    100055558127615,10995116277759,17592186044415,
    545357767376895,70368744177663,1125899906842623,
    10133099161583615,9007199254740991,
    24769797950537727,4503599627370495,
    144115188075855871,72057594037927935,
    432345564227567615,288230376151711743,
    1152921504606846975,4611686018427387903
  ];
const MAX_COUNTS: [u64; 64] = [1,
    425656284035217743, 
    1701748900850019777, 
    3167597434038354478, 
    3787884015050788482, 
      3378907169603895030,2466706950238087748,1571012171387856192,913164427599983727,499840874923678341,263157990621533964,135128109904869290,68283616225825256,
     34151861008771016,16967424859951587,8393048221327186,4139595949113890,2037655246635364,1001591348315641,491808604962296,241293656953012,118304122014405,57968649799947,
     28388662714236,13895161400556,6797526392535,3323560145881,1624109166018,793189260998,387148515886,188844769357,92054377509,44841620426,
     21827124353,10616326552,5159281045,2505087309,1215204383,588891145,285076316,137840686,66567488,32103728,15460810,7433670,3567978,1709640,817053,389954,185387,87993,41604,19611,9184,4283,2001,914,421,187,84,37,15,7,2
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
    primes(start, rootint(n, k)).into_iter()
        .map(|p| count_rec(k - 1, n / p, p)).sum()
}

/// Count integers at most `n` with `k` prime factors, with repetition.
/// Port of MPU `almost_primes.c`, `almost_prime_count` and `_cs`.
/// ```
/// assert_eq!(math_prime_util::almost_prime_count(3, 30), 7);
/// ```
pub fn almost_prime_count(k: u32, n: u64) -> u64 {
    if k == 0 { return u64::from(n >= 1); }
    if k >= 64 || n >> k == 0 { return 0; }
    if n >= u64::MAX - OFFSETS[k as usize] { return MAX_COUNTS[k as usize]; }
    let (k, n) = reduce(k, n);
    if k == 0 { return n; }
    count_rec(k, n, 2)
}

// almost_primes.c: _genkap, with unique ordered prime factors.
fn generate(k: u32, lo: u64, hi: u64, m: u64, start: u64, out: &mut Vec<u64>) {
    if k == 1 {
        out.extend(primes(start.max(lo.div_ceil(m)), hi / m).into_iter().map(|p| m * p));
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
    if lo > hi { return Vec::new(); }
    if k == 0 { return if lo <= 1 && hi >= 1 { vec![1] } else { Vec::new() }; }
    if k >= 64 || hi >> k == 0 { return Vec::new(); }
    let mut out = Vec::new();
    generate(k, lo, hi, 1, 2, &mut out);
    out.sort_unstable();
    out
}

fn small_nth(k: u32, n: u64) -> u64 {
    if k == 2 { [0,4,6,9,10,14,15,21][n as usize] }
    else { [0,8,12,18,20,27,28,30][n as usize].checked_shl(k-3).unwrap_or(0) }
}

fn nth_reduce(k: u32, n: u64) -> u32 {
    let mut r = k.saturating_sub(48);
    while k-r > 0 && n < FIRST_3[(k-r) as usize] { r += 1; }
    r
}

// inverse_interpolate.c: inverse_interpolate_k, zero threshold.
fn inverse(mut lo: u64, n: u64, f: impl Fn(u64) -> u64) -> u64 {
    let mut hi = lo.max(2);
    while f(hi) < n {
        if hi == u64::MAX { return 0; }
        hi = hi.saturating_mul(2);
    }
    while lo < hi {
        let mid = lo + (hi-lo)/2;
        if f(mid) < n { lo = mid+1; } else { hi = mid; }
    }
    if f(lo) > n { lo.saturating_sub(1) } else { lo }
}

fn nth(k: u32, n: u64, kind: u8) -> Option<u64> {
    if n == 0 || k >= 64 { return None; }
    if k == 0 { return (n == 1).then_some(1); }
    if n >= MAX_COUNTS[k as usize] {
        return (n == MAX_COUNTS[k as usize]).then_some(u64::MAX - OFFSETS[k as usize]);
    }
    if k == 1 { return crate::nth_prime(n); }
    if n < 8 { return Some(small_nth(k, n)); }
    let r = nth_reduce(k, n);
    if r > 0 && kind != 3 {
        return nth(k-r, n, kind).and_then(|v| v.checked_mul(1u64 << r));
    }
    let lo = 1u64 << k;
    let value = inverse(lo, n, |x| match kind {
        1 => almost_prime_count_upper(k,x),
        2 => almost_prime_count_lower(k,x),
        3 => almost_prime_count_approx(k,x),
        _ => almost_prime_count(k,x),
    });
    (value != 0).then_some(value)
}

/// Return the exact value for the `n`th integer with `k` prime factors.
/// Return `None` if the index is zero or the result exceeds `u64`.
/// Port of MPU `almost_primes.c`, `nth_almost_prime`.
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime(3, 4), Some(20));
/// ```
pub fn nth_almost_prime(k: u32, n: u64) -> Option<u64> { nth(k, n, 0) }

/// Return the lower bound for the `n`th integer with `k` prime factors.
/// Return `None` if the index is zero or the result exceeds `u64`.
/// Port of MPU `almost_primes.c`, `nth_almost_prime_lower`.
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime_lower(3, 4), Some(20));
/// ```
pub fn nth_almost_prime_lower(k: u32, n: u64) -> Option<u64> { nth(k, n, 1) }

/// Return the upper bound for the `n`th integer with `k` prime factors.
/// Return `None` if the index is zero or the result exceeds `u64`.
/// Port of MPU `almost_primes.c`, `nth_almost_prime_upper`.
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime_upper(3, 4), Some(20));
/// ```
pub fn nth_almost_prime_upper(k: u32, n: u64) -> Option<u64> { nth(k, n, 2) }

/// Return the estimate for the `n`th integer with `k` prime factors.
/// Return `None` if the index is zero or the result exceeds `u64`.
/// Port of MPU `almost_primes.c`, `nth_almost_prime_approx`.
/// ```
/// assert_eq!(math_prime_util::nth_almost_prime_approx(3, 4), Some(20));
/// ```
pub fn nth_almost_prime_approx(k: u32, n: u64) -> Option<u64> { nth(k, n, 3) }
