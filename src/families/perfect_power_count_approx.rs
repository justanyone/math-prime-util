use crate::rootint;

// MPU perfect_powers.c, _moebius and perfect_power_count.
const MOEBIUS: [i8; 65] = [
    0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1, -1, 0, -1, 1, 1, 0, -1, 0, -1, 0, 1, 1, -1,
    0, 0, 1, 0, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, 1, 1, 0, -1, -1, -1, 0, 0, 1, -1,
    0, 0, 0, 1, 0, -1, 0, 1, 0, 1, 1, -1, 0, -1, 1, 0, 0,
];

pub(super) fn count(n: u64) -> u64 {
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

/// Count the positive perfect powers at most `n`, including 1.
///
/// Port of `perfect_power_count_approx` in MPU 0.75 `perfect_powers.c`.
/// MPU uses the exact count for this approximation.
///
/// ```
/// use math_prime_util::perfect_power_count_approx;
/// assert_eq!(perfect_power_count_approx(100), 13);
/// ```
#[must_use]
pub fn perfect_power_count_approx(n: u64) -> u64 {
    count(n)
}
