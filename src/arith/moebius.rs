use super::{factors, range_len};
use crate::{rootint, sqrtint};

/// Return the Moebius function. Port of `moebius.c::moebius`.
/// ```
/// assert_eq!(math_prime_util::moebius(30), -1);
/// ```
#[allow(clippy::missing_panics_doc)] // Both conversions are bounded by the small table.
pub fn moebius(n: u64) -> i8 {
    const SMALL: [u32; 16] = [
        2_703_565_065,
        23_406_865,
        620_863_913,
        1_630_114_197,
        157_354_249,
        2_844_895_525,
        2_166_423_889,
        363_177_345,
        2_835_441_929,
        2_709_852_521,
        1_095_049_497,
        92_897_577,
        1_772_687_649,
        162_113_833,
        160_497_957,
        689_538_385,
    ];
    if n < 256 {
        return i8::try_from((SMALL[usize::try_from(n / 16).unwrap()] >> (2 * (n % 16))) & 3)
            .unwrap()
            - 1;
    }
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % (p * p) == 0 {
            return 0;
        }
    }
    let f = factors(n);
    if f.iter().any(|&(_, e)| e > 1) {
        0
    } else if f.len() % 2 == 0 {
        1
    } else {
        -1
    }
}

/// Return Moebius values on an inclusive interval.
/// Port of `moebius.c::range_moebius`, `_sieve_moebius`, and `_moebius_by_hand`.
/// # Panics
/// Panics if the interval is reversed or too large to allocate.
/// ```
/// assert_eq!(math_prime_util::moebius_range(1, 4), [1,-1,-1,0]);
/// ```
pub fn moebius_range(lo: u64, hi: u64) -> Vec<i8> {
    let len = range_len(lo, hi);
    let width = hi - lo;
    if hi < 100 || width < 10 || (hi > 1 << 25 && width < rootint(hi, 3) / 4) {
        return (lo..=hi).map(moebius).collect();
    }
    let mut mu = vec![0_u8; len];
    let (mut logp, mut nextlog) = (1_u8, 3_u64);
    let root = sqrtint(hi);
    for p in crate::sieve::primes(
        2,
        root + u64::from(root * root != hi && root < u64::from(u32::MAX)),
    ) {
        if p > nextlog {
            logp += 2;
            nextlog = (nextlog - 1) * 4 + 1;
        }
        let start = ((p - lo % p) % p).max(if lo == 0 { p } else { 0 });
        for i in (usize::try_from(start).unwrap()..len).step_by(usize::try_from(p).unwrap()) {
            mu[i] = mu[i].wrapping_add(logp);
        }
        let p2 = p * p;
        let start = ((p2 - lo % p2) % p2).max(if lo == 0 { p2 } else { 0 });
        for i in (usize::try_from(start).unwrap()..len).step_by(usize::try_from(p2).unwrap()) {
            mu[i] = 128;
        }
    }
    mu.into_iter()
        .enumerate()
        .map(|(i, a)| {
            let n = lo + u64::try_from(i).unwrap();
            if n == 0 || a & 128 != 0 {
                0
            } else {
                let sign = if a & 1 == 0 { 1 } else { -1 };
                if u32::from(a) >= n.ilog2() {
                    sign
                } else {
                    -sign
                }
            }
        })
        .collect()
}
