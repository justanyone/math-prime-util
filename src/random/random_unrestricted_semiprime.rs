use super::{random_factored_integer::factors, urandomb, urandomm};

/// Returns a uniform semiprime with exactly `bits` bits.
///
/// Port of `reference/mpu/random_prime.c`: `random_unrestricted_semiprime`.
/// Returns `None` for bits below 3 or above 64. Larger sizes await bigint support.
///
/// # Panics
/// Panics if an unseeded thread cannot read OS entropy.
///
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::random_unrestricted_semiprime(3), Some(4));
/// assert_eq!(math_prime_util::random_unrestricted_semiprime(65), None);
/// ```
pub fn random_unrestricted_semiprime(bits: u32) -> Option<u64> {
    const SMALL: [u64; 42] = [
        4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35, 38, 39, 46, 49, 51, 55, 57, 58, 62, 65,
        69, 74, 77, 82, 85, 86, 87, 91, 93, 94, 95, 106, 111, 115, 118, 119, 121, 122, 123,
    ];
    let (start, count) = match bits {
        3 => (0, 2),
        4 => (2, 4),
        5 => (6, 4),
        6 => (10, 12),
        7 => (22, 20),
        8..=64 => {
            let min = 1u64 << (bits - 1);
            loop {
                let n = min + urandomb(bits - 1);
                if factors(n).len() == 2 {
                    return Some(n);
                }
            }
        }
        _ => return None,
    };
    Some(SMALL[start + usize::try_from(urandomm(count)).unwrap()])
}
