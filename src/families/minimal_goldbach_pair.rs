use crate::{is_prime, next_prime};

/// Return the smaller prime in the first Goldbach pair for `n`.
/// Return `None` if no pair exists.
///
/// Port of MPU 0.75 `goldbach.c`, `minimal_goldbach_pair`.
///
/// ```
/// use math_prime_util::minimal_goldbach_pair;
/// assert_eq!(minimal_goldbach_pair(12), Some(5));
/// assert_eq!(minimal_goldbach_pair(11), None);
/// ```
pub fn minimal_goldbach_pair(n: u64) -> Option<u64> {
    if n < 4 {
        return None;
    }
    if n & 1 != 0 || n == 4 {
        return (is_prime(n - 2) != 0).then_some(2);
    }
    let mut p = 3;
    while p <= n / 2 {
        if is_prime(n - p) != 0 {
            return Some(p);
        }
        p = next_prime(p)?;
    }
    None
}
