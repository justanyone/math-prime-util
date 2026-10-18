use crate::int::rootint;

/// Returns the largest perfect power less than `n`.
/// Returns `None` at zero because its predecessor is negative.
///
/// Port of MPU 0.75 `perfect_powers.c`, `prev_perfect_power` and
/// `_prev_perfect_power`.
///
/// ```
/// use math_prime_util::prev_perfect_power;
/// assert_eq!(prev_perfect_power(9), Some(8));
/// assert_eq!(prev_perfect_power(1), Some(0));
/// assert_eq!(prev_perfect_power(0), None);
/// ```
pub fn prev_perfect_power(n: u64) -> Option<u64> {
    if n <= 4 {
        return n.checked_sub(1).map(|m| u64::from(m != 0));
    }
    if n <= 8 {
        return Some(4);
    }
    let mut best = 8;
    for k in 2..=n.ilog2() {
        let root = rootint(n, k);
        let mut candidate = root.pow(k);
        if candidate >= n {
            candidate = (root - 1).pow(k);
        }
        if candidate > best && candidate < n {
            best = candidate;
        }
    }
    Some(best)
}
