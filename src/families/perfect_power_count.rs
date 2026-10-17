use crate::int::rootint;

/// Counts positive perfect powers at most `n`. One is included once.
///
/// Port of MPU 0.75 `perfect_powers.c`, `perfect_power_count`, with its
/// `_moebius` table and integer-root inclusion and exclusion sum.
///
/// ```
/// use math_prime_util::perfect_power_count;
/// assert_eq!(perfect_power_count(0), 0);
/// assert_eq!(perfect_power_count(16), 5);
/// ```
pub fn perfect_power_count(n: u64) -> u64 {
    const MOEBIUS: [i8; 65] = [
        0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0, 1, 1, -1, 0, 0, 1,
        0, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0, -1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 1, 0, -1,
        0, 1, 0, 1, 1, -1, 0, -1, 1, 0, 0,
    ];
    if n < 8 {
        return u64::from(n >= 1) + u64::from(n >= 4);
    }
    let mut sum = 1;
    for k in 2..=n.ilog2() {
        match MOEBIUS[k as usize] {
            -1 => sum += rootint(n, k) - 1,
            1 => sum -= rootint(n, k) - 1,
            _ => {}
        }
    }
    sum
}
