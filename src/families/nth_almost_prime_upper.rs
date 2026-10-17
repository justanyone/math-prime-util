#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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
    if n == 0 || k >= 64 { return None; }
    if k == 0 { return (n == 1).then_some(1); }
    if k == 1 { return prime_upper(n); }
    let maxn = u64::MAX - OFFSETS[k as usize];
    let maxc = MAX_COUNTS[k as usize];
    if n >= maxc { return (n == maxc).then_some(maxn); }
    if n < 8 {
        let semi = [0,4,6,9,10,14,15,21];
        let mult = [0,8,12,18,20,27,28,30];
        return Some(if k == 2 { semi[n as usize] } else { mult[n as usize] << (k-3) });
    }
    let mut r = if k > 49 && n < FIRST_THREE[49] { k-48 } else { 0 };
    if k <= 49 || r > 0 {
        while n < FIRST_THREE[(k-r) as usize] { r += 1; }
    }
    if r > 0 {
        let up = nth_almost_prime_upper(k-r, n)?;
        return Some(if up > maxn || up > (u64::MAX >> r) { maxn } else { up << r });
    }
    interpolate(5 << k, 0, n, 0, |x| count_lower(k, x)).map(|up| up.min(maxn))
}

fn count_lower(mut k: u32, mut n: u64) -> u64 {
    if k >= 64 || n >> k == 0 { return 0; }
    let mut r = k.saturating_sub(40);
    while k >= r && u128::from(n >> r) < 3_u128.pow(k-r) { r += 1; }
    k -= r; n >>= r;
    if k == 0 { return u64::from(n >= 1); }
    if k == 1 { return prime_count_lower(n); }
    if k == 2 && n < 256 { return semiprime_count(n); }
    if n < 3 * (1 << (k-1)) { return 1; }
    if n < 9 * (1 << (k-2)) { return 2; }
    if n < 10 * (1 << (k-2)) { return 3; }
    let max = MAX_COUNTS[k as usize];
    if n >= u64::MAX - OFFSETS[k as usize] { return max; }
    let x = n as f64; let l = x.ln(); let ll = l.ln();
    let mut mult = if n <= 1_048_575 { LOWER_20[k as usize] } else if n <= 4_294_967_295 { LOWER_32[k as usize] } else { LOWER_64[k as usize] };
    let bound = match k {
        2 => if x >= 1e12 { mult = 1.0; x*(ll+0.1769)/l*(1.0+0.4232/l) } else { x*(ll+0.261536)/l },
        3 => {
            mult = if n < 638 { 0.8418 } else if n <= 1926 { let w = (x-638.0)/1288.0; (1.0-w)*0.8939+w*0.9233 }
                else if n <= 500_194 { let w = (x-1927.0)/498_267.0; (1.0-w)*0.9233+w }
                else if n <= 3_184_393_786 { let w = (x-500_194.0)/3_183_893_592.0; (1.0-w)+w*1.039 } else { 1.0004 };
            x*ll*ll/(2.0*l)
        },
        4 => x*(ll+0.26153).powi(3)/(6.0*l),
        _ => {
            let fk = f64::from(k); let plus = ll + (fk.ln()*fk.ln().ln()-0.504377);
            let mut b = x/l;
            for i in 1..k { b *= plus/f64::from(i); }
            b
        }
    };
    ((bound*mult) as u64).min(max)
}
const FIRST_THREE: [u64; 50] = [1, 2, 3, 5, 8, 14, 23, 39, 64, 103, 169, 269, 427, 676, 1065, 1669, 2628, 4104, 6414, 10023, 15608, 24281, 37733, 58503, 90616, 140187, 216625, 334527, 516126, 795632, 1225641, 1886570, 2901796, 4460359, 6851532, 10518476, 16138642, 24748319, 37932129, 58110457, 88981343, 136192537, 208364721, 318653143, 487128905, 744398307, 1137129971, 1736461477, 2650785552U, 4045250962U];
const LOWER_20: [f64; 13] = [0.0,0.0, 0.8197, 0.8418, 0.5242,
   0.5154,0.3053,0.1901,0.1253,0.0892,0.06551,0.05082,0.04101];
const LOWER_32: [f64; 21] = [0.0,0.0, 1.004,  0.7383, 0.6828,
   0.5939,0.3594,0.2222,0.1438,0.09754,0.06981,0.05245,0.04151,
   0.03461,0.03006,0.02709,0.02553,0.02502,0.02552,0.02697,0.02945 ];
const LOWER_64: [f64; 41] = [0.0,0.0, 1.011,  0.8093, 0.7484,
  
  0.6465,0.3982,0.2463,0.1571,0.1048,0.07363,0.0545,0.0422,
  
  0.0331,0.0270,0.0232,0.0208,0.0194,0.0190,0.0193,0.0203,
  
  0.0222,0.0252,0.0295,0.0356,0.0444,0.0570,0.0753,0.102,
  
  0.14,0.20,0.297,
  
  0.44,0.68,1.07,1.71,2.8,4.7,8.0,13.89,23.98,
];
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
