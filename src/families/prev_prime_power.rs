use super::is_prime_power;

/// Return the first prime power strictly less than `n`.
///
/// Return `None` for inputs at most two.
/// Port of MPU 0.75 `prime_powers.c`, `prev_prime_power`.
///
/// ```
/// use math_prime_util::prev_prime_power;
/// assert_eq!(prev_prime_power(9), Some(8));
/// assert_eq!(prev_prime_power(2), None);
/// ```
pub fn prev_prime_power(n: u64) -> Option<u64> {
    if n <= 2 {
        return None;
    }
    let n = n - 1;
    let bit = 1 << n.ilog2();
    let mut i = n - u64::from(n & 1 == 0);
    while i & bit != 0 {
        if is_prime_power(i) != 0 {
            return Some(i);
        }
        i -= 2;
    }
    Some(i + 1)
}
