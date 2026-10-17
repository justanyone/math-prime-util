use super::{random_nbit_prime::small_nth_prime, urandomm};
use crate::is_prob_prime;

/// Returns a uniform random prime with exactly `digits` decimal digits.
///
/// Port of `reference/mpu/random_prime.c`, `random_ndigit_prime`.
/// Sizes of 20 or more digits require MPU's bigint fallback and return `None`.
///
/// # Panics
/// Panics if `digits == 0` or the initial OS entropy request fails.
///
/// ```
/// let p = math_prime_util::random_ndigit_prime(3).unwrap();
/// assert!((100..1000).contains(&p));
/// assert_eq!(math_prime_util::is_prime(p), 2);
/// ```
pub fn random_ndigit_prime(digits: u32) -> Option<u64> {
    assert!(digits >= 1, "digits must be >= 1");
    let selection = match digits {
        1 => Some((1, 4)),
        2 => Some((5, 21)),
        3 => Some((26, 143)),
        _ => None,
    };
    if let Some((start, range)) = selection {
        return Some(small_nth_prime(start + urandomm(range)));
    }
    if digits >= 20 {
        return None;
    }
    let low = 10u64.pow(digits - 1) + 1;
    let high = 10 * low - 11;
    let range = ((high - low) >> 1) + 1;
    loop {
        let n = low + 2 * urandomm(range);
        if is_prob_prime(n) != 0 {
            return Some(n);
        }
    }
}
