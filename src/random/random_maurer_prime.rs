use super::{urandomb, urandomm};
use crate::{is_prime, next_prime};

/// Returns a proven prime with exactly `bits` bits.
///
/// Port of MPU `lib/Math/Prime/Util/RandomPrimes.pm`, `random_maurer_prime`.
/// The native path uses `random_prime.c`, `random_nbit_prime`. Native
/// primality tests are deterministic. Sizes above 64 return `None` until
/// the bigint phase.
///
/// # Panics
/// Panics if `bits < 2`, or if an unseeded thread cannot read OS entropy.
///
/// ```
/// math_prime_util::srand(15);
/// let p = math_prime_util::random_maurer_prime(10).unwrap();
/// assert_eq!(p, 619);
/// assert_eq!(math_prime_util::is_prime(p), 2);
/// ```
pub fn random_maurer_prime(bits: u32) -> Option<u64> {
    assert!(bits >= 2, "random_maurer_prime, bits must be >= 2");
    match bits {
        2 => Some(if urandomb(1) == 0 { 3 } else { 2 }),
        3 => Some(if urandomb(1) == 0 { 7 } else { 5 }),
        4 => Some(if urandomb(1) == 0 { 13 } else { 11 }),
        5..=9 => {
            let (start, range) = match bits {
                5 => (7, 5),
                6 => (12, 7),
                7 => (19, 13),
                8 => (32, 23),
                _ => (55, 43),
            };
            let index = start + urandomm(range);
            // MPU samples a prime index, rather than an odd candidate here.
            let mut prime = 0;
            for _ in 0..index {
                prime = next_prime(prime)?;
            }
            Some(prime)
        }
        10..=64 => {
            let low = (1u64 << (bits - 1)) + 1;
            loop {
                let candidate = low + (urandomb(bits - 2) << 1);
                if is_prime(candidate) == 2 {
                    return Some(candidate);
                }
            }
        }
        _ => None,
    }
}
