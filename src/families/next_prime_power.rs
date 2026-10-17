use super::is_prime_power;

/// Return the first prime power strictly greater than `n`.
///
/// Return `None` if no larger prime power fits in `u64`.
/// Port of MPU 0.75 `prime_powers.c`, `next_prime_power`.
///
/// ```
/// use math_prime_util::next_prime_power;
/// assert_eq!(next_prime_power(7), Some(8));
/// assert_eq!(next_prime_power(u64::MAX), None);
/// ```
pub fn next_prime_power(n: u64) -> Option<u64> {
    if n < 2 {
        return Some(2);
    }
    if n >= 18_446_744_073_709_551_557 {
        return None;
    }
    let bit = 1 << n.ilog2();
    let mut i = n + 1 + (n & 1);
    while i & bit != 0 {
        if is_prime_power(i) != 0 {
            return Some(i);
        }
        i += 2;
    }
    Some(i - 1)
}
