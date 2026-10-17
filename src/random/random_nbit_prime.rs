use super::{urandomb, urandomm};
use crate::{is_prob_prime, next_prime};

// Small nth-prime selections from reference/mpu/random_prime.c.
pub(super) fn small_nth_prime(index: u64) -> u64 {
    (0..index).fold(1, |p, _| next_prime(p).expect("small prime exists"))
}

/// Returns a uniform random prime with exactly `bits` bits.
///
/// Port of `reference/mpu/random_prime.c`, `random_nbit_prime`.
/// Bit sizes above 64 return `None` until the bigint phase.
///
/// # Panics
/// Panics if `bits < 2` or the initial OS entropy request fails.
///
/// ```
/// let p = math_prime_util::random_nbit_prime(8).unwrap();
/// assert!((128..256).contains(&p));
/// assert_eq!(math_prime_util::is_prime(p), 2);
/// ```
pub fn random_nbit_prime(bits: u32) -> Option<u64> {
    assert!(bits >= 2, "bits must be >= 2");
    let selection = match bits {
        2 => return Some(if urandomb(1) != 0 { 2 } else { 3 }),
        3 => return Some(if urandomb(1) != 0 { 5 } else { 7 }),
        4 => return Some(if urandomb(1) != 0 { 11 } else { 13 }),
        5 => Some((7, 5)),
        6 => Some((12, 7)),
        7 => Some((19, 13)),
        8 => Some((32, 23)),
        9 => Some((55, 43)),
        _ => None,
    };
    if let Some((start, range)) = selection {
        return Some(small_nth_prime(start + urandomm(range)));
    }
    if bits > 64 {
        return None;
    }
    let base = (1u64 << (bits - 1)) + 1;
    loop {
        let n = base + (urandomb(bits - 2) << 1);
        if is_prob_prime(n) != 0 {
            return Some(n);
        }
    }
}
