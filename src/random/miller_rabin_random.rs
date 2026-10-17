use super::urandomm;
use crate::{is_prob_prime, is_strong_pseudoprime};

/// Returns 1 when `n` passes `bases` Miller-Rabin tests, else 0.
///
/// Port of `reference/mpu/random_prime.c`, `is_mr_random`.
/// The XS name is `miller_rabin_random`.
/// Each base is uniform in `2..=n-2`.
/// When `bases` is at least `3 * (n / 4)`, a deterministic test runs and no base is drawn.
/// A value of `n` with more than 64 bits returns `None` until the bigint phase.
///
/// # Panics
/// Panics if `bases` is 0.
/// Panics if the first OS entropy request fails.
///
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::miller_rabin_random(31, 5), Some(1));
/// assert_eq!(
///     math_prime_util::miller_rabin_random(u128::from(u64::MAX) + 1, 1),
///     None
/// );
/// ```
pub fn miller_rabin_random(n: u128, bases: u64) -> Option<u8> {
    assert!(bases > 0, "bases must be positive");
    let Ok(n) = u64::try_from(n) else {
        return None;
    };
    if n < 2 {
        return Some(0);
    }
    // random_prime.c uses 3 * (n / 4), not (3 * n) / 4.
    if bases >= 3 * (n / 4) {
        return Some(u8::from(is_prob_prime(n) != 0));
    }
    let mut left = bases;
    while left > 0 {
        let base = 2 + urandomm(n - 3);
        if !is_strong_pseudoprime(n, base) {
            return Some(0);
        }
        left -= 1;
    }
    Some(1)
}
