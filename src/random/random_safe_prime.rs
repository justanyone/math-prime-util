use super::{urandomb, urandomm};
use crate::is_prob_prime;

/// Returns a random prime `p` with `bits` bits such that `(p - 1) / 2` is prime.
///
/// Port of `reference/mpu/random_prime.c`, `random_safe_prime`.
/// Bit sizes above 64 return `None` until the bigint phase.
///
/// # Panics
/// Panics if `bits < 3` or the initial OS entropy request fails.
///
/// ```
/// let p = math_prime_util::random_safe_prime(8).unwrap();
/// assert_eq!(math_prime_util::is_prime(p), 2);
/// assert_eq!(math_prime_util::is_prime((p - 1) / 2), 2);
/// ```
pub fn random_safe_prime(bits: u32) -> Option<u64> {
    const SMALL_SAFE: [u64; 8] = [5, 7, 11, 23, 47, 59, 83, 107];
    const P15MASK: u64 = 14079;
    assert!(bits >= 3, "bits must be >= 3");
    match bits {
        3 => return Some(SMALL_SAFE[usize::try_from(urandomm(2)).unwrap()]),
        4 => return Some(11),
        5 => return Some(23),
        6 => return Some(SMALL_SAFE[4 + usize::try_from(urandomm(2)).unwrap()]),
        7 => return Some(SMALL_SAFE[6 + usize::try_from(urandomm(2)).unwrap()]),
        65.. => return None,
        _ => (),
    }
    let base = (1u64 << (bits - 2)) + 1;
    loop {
        let p = base + (urandomb(bits - 3) << 1);
        let q = 2 * p + 1;
        if (P15MASK & (1 << (p % 15))) == 0
            && p % 7 != 0
            && p % 7 != 3
            && is_prob_prime(p) != 0
            && is_prob_prime(q) != 0
        {
            return Some(q);
        }
    }
}
