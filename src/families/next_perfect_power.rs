use crate::int::rootint;

/// Returns the smallest perfect power greater than `n`.
/// Returns `None` if the result exceeds `u64::MAX`.
///
/// Port of MPU 0.75 `perfect_powers.c`, `next_perfect_power` and
/// `_next_perfect_power`, with the bound from `constants.h`.
///
/// ```
/// use math_prime_util::next_perfect_power;
/// assert_eq!(next_perfect_power(9), Some(16));
/// assert_eq!(next_perfect_power(u64::MAX), None);
/// ```
pub fn next_perfect_power(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(1);
    }
    if n == 1 {
        return Some(4);
    }
    if n >= 18_446_744_065_119_617_025 {
        return None;
    }
    let mut best = (rootint(n, 2) + 1).checked_pow(2).unwrap_or(u64::MAX);
    for k in 3..=1 + n.ilog2() {
        if let Some(candidate) = (rootint(n, k) + 1).checked_pow(k) {
            if candidate < best && candidate > n {
                best = candidate;
            }
        }
    }
    (best != u64::MAX).then_some(best)
}
