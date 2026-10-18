use super::{divisor_sum, factor_powers};
use crate::is_square;

const TAU: &[i64] = &[
    0,
    1,
    -24,
    252,
    -1472,
    4830,
    -6048,
    -16_744,
    84_480,
    -113_643,
    -115_920,
    534_612,
    -370_944,
    -577_738,
    401_856,
    1_217_160,
    987_136,
    -6_905_934,
    2_727_432,
    10_661_420,
    -7_109_760,
    -4_219_488,
    -12_830_688,
    18_643_272,
    21_288_960,
    -25_499_225,
    13_865_712,
    -73_279_080,
    24_647_168,
    128_406_630,
    -29_211_840,
    -52_843_168,
    -196_706_304,
    134_722_224,
    165_742_416,
    -80_873_520,
    167_282_496,
    -182_213_314,
    -255_874_080,
    -145_589_976,
    408_038_400,
    308_120_442,
    101_267_712,
    -17_125_708,
    -786_948_864,
    -548_895_690,
    -447_438_528,
];

/// Return the native Ramanujan tau value.
/// Return `None` if the native calculation overflows.
///
/// Port of MPU 0.75 `util.c::ramanujan_tau` and `PP.pm::_tauprime`, `_taupower`.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_tau(2), Some(-24));
/// ```
pub fn ramanujan_tau(n: u64) -> Option<i64> {
    if let Some(&value) = TAU.get(usize::try_from(n).ok()?) {
        return Some(value);
    }
    let mut result = 1_i128;
    for (p, e) in factor_powers(n) {
        let tp = tau_prime(p)?;
        // PP.pm::_taupower returns immediately for exponent one.
        let p11 = if e > 1 {
            i128::from(p).checked_pow(11)?
        } else {
            0
        };
        let (mut previous, mut current) = (1_i128, tp);
        for _ in 1..e {
            let next = tp
                .checked_mul(current)?
                .checked_sub(p11.checked_mul(previous)?)?;
            (previous, current) = (current, next);
        }
        result = result.checked_mul(current)?;
    }
    i64::try_from(result).ok()
}

// MPU PP.pm::_tauprime and _taupower supply values beyond util.c's table.
fn tau_prime(p: u64) -> Option<i128> {
    if let Some(&value) = TAU.get(usize::try_from(p).ok()?) {
        return Some(i128::from(value));
    }
    let x = i128::from(p);
    let leading = 28_i128
        .checked_mul(x.checked_pow(6)?)?
        .checked_sub(28_i128.checked_mul(x.checked_pow(5)?)?)?
        .checked_sub(90_i128.checked_mul(x.checked_pow(4)?)?)?
        .checked_sub(35_i128.checked_mul(x.checked_pow(3)?)?)?
        .checked_sub(1)?;
    let mut sum = 0_i128;
    for t in 1..=crate::sqrtint(p) {
        let t2 = t * t;
        let v = p - t2;
        let h = match v % 4 {
            0 => 2 * class_number(v) + class_number(4 * v),
            3 => (if v % 8 == 3 { 6 } else { 4 }) * class_number(v),
            _ => class_number(4 * v),
        };
        let t2 = i128::from(t2);
        let term = t2
            .checked_pow(3)?
            .checked_mul(4 * t2 * t2 - 9 * x * t2 + 7 * x * x)?
            .checked_mul(h)?;
        sum = sum.checked_add(term)?;
    }
    leading.checked_sub(32_i128.checked_mul(sum / 3)?)
}
// util.c::hclassno and _count_class_div, in the bounded tau-prime path.
fn class_number(n: u64) -> i128 {
    if n == 0 {
        return -1;
    }
    if matches!(n % 4, 1 | 2) {
        return 0;
    }
    if n == 3 {
        return 4;
    }
    let mut b = n & 1;
    let mut b2 = (n + 1) >> 2;
    let square = is_square(b2);
    let mut h = divisor_sum(b2, 0).unwrap_or(0) >> 1;
    if b == 1 {
        h = 1 + u64::from(square) + 2 * (h - 1);
    }
    b += 2;
    loop {
        b2 = (n + b * b) >> 2;
        if 3 * b2 >= n {
            break;
        }
        let limit = crate::sqrtint(b2) - u64::from(is_square(b2));
        let divisors = (b + 1..=limit).filter(|d| b2 % d == 0).count();
        h += u64::from(b2 % b == 0)
            + u64::from(is_square(b2))
            + 2 * u64::try_from(divisors).unwrap_or(0);
        b += 2;
    }
    let correction = if b2 * 3 == n {
        4
    } else if square && n % 2 == 0 {
        6
    } else {
        0
    };
    i128::from(12 * h + correction)
}

#[cfg(test)]
mod tests {
    use super::class_number;

    #[test]
    fn class_number_small_discriminants() {
        for (n, expected) in [(0, -1), (1, 0), (2, 0), (3, 4), (4, 6), (7, 12)] {
            assert_eq!(class_number(n), expected);
        }
    }
}
