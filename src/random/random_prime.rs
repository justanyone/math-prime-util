use super::urandomm;
use crate::{is_prob_prime, next_prime, prev_prime};

/// Returns a uniform random prime in `low..=high`, or `None` for an empty range.
///
/// Port of `reference/mpu/random_prime.c`, `random_prime`.
/// Uses the current thread's random stream.
///
/// # Panics
/// Panics if the initial OS entropy request fails.
///
/// ```
/// assert_eq!(math_prime_util::random_prime(10, 12), Some(11));
/// assert_eq!(math_prime_util::random_prime(14, 16), None);
/// ```
pub fn random_prime(low: u64, high: u64) -> Option<u64> {
    const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
    if low > high || low > MAX_PRIME || high < 2 {
        return None;
    }
    let low = if low <= 2 { 2 } else { next_prime(low - 1)? };
    let high = if high >= MAX_PRIME {
        MAX_PRIME
    } else {
        prev_prime(high + 1)?
    };
    if low > high {
        return None;
    }
    let low = if low == 2 { 1 } else { low };
    let range = ((high - low) >> 1) + 1;
    loop {
        let n = low + 2 * urandomm(range);
        if n == 1 || is_prob_prime(n) != 0 {
            return Some(if n == 1 { 2 } else { n });
        }
    }
}
