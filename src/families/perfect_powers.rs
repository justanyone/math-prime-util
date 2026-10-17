//! Bounds for the n-th perfect power from MPU 0.75 (`perfect_powers.c`).
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

/// Largest perfect power that fits in 64 bits: `(2^32 - 1)^2`.
/// MPU: `MPU_MAX_PERFECT_POW` in `constants.h` (64-bit build).
const MAX_PERFECT_POW: u64 = 18_446_744_065_119_617_025;
/// First index whose n-th perfect power is `MAX_PERFECT_POW` or more.
/// MPU: `MPU_MAX_PERFECT_POW_IDX` in `constants.h` (64-bit build).
const MAX_PERFECT_POW_IDX: u64 = 4_297_615_581;

/// Returns the lower bound of the n-th perfect power.
///
/// Port of `nth_perfect_power_lower` in `reference/mpu/perfect_powers.c`.
/// The same polynomial in `n` is used, with the same exponents and constant.
///
/// Returns `None` when `n` is 0, where MPU returns undef.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_perfect_power_lower;
///
/// assert_eq!(nth_perfect_power_lower(10), Some(14));
/// ```
#[must_use]
pub fn nth_perfect_power_lower(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 1 {
        return Some(n);
    }
    if n >= MAX_PERFECT_POW_IDX {
        return Some(MAX_PERFECT_POW);
    }
    let x = n as f64;
    let mut pp =
        x.powf(2.0) + (13.0 / 3.0) * x.powf(4.0 / 3.0) + (32.0 / 15.0) * x.powf(16.0 / 15.0);
    pp += -2.0 * x.powf(5.0 / 3.0) - 2.0 * x.powf(7.0 / 5.0) - 2.0 * x.powf(9.0 / 7.0)
        + 2.0 * x.powf(12.0 / 10.0);
    pp += -2.0 * x.powf(13.0 / 11.0) - 2.0 * x.powf(15.0 / 13.0);
    pp += 5.5;
    Some(clamp(pp))
}

/// Returns the upper bound of the n-th perfect power.
///
/// Port of `nth_perfect_power_upper` in `reference/mpu/perfect_powers.c`.
/// The same polynomial in `n` is used, with the same exponents and constant.
///
/// Returns `None` when `n` is 0, where MPU returns undef.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_perfect_power_upper;
///
/// assert_eq!(nth_perfect_power_upper(10), Some(92));
/// ```
#[must_use]
pub fn nth_perfect_power_upper(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 1 {
        return Some(n);
    }
    if n >= MAX_PERFECT_POW_IDX {
        return Some(MAX_PERFECT_POW);
    }
    let x = n as f64;
    let mut pp =
        x.powf(2.0) + (13.0 / 3.0) * x.powf(4.0 / 3.0) + (32.0 / 15.0) * x.powf(16.0 / 15.0);
    pp += -2.0 * x.powf(5.0 / 3.0) - 2.0 * x.powf(7.0 / 5.0) - 2.0 * x.powf(9.0 / 7.0)
        + 2.0 * x.powf(12.0 / 10.0);
    // MPU skips the 11 and 13 terms here.
    pp += 2.0 * x.powf(16.0 / 14.0);
    pp -= 3.5;
    Some(clamp(pp))
}

/// Returns the approximate value of the n-th perfect power.
///
/// Port of `nth_perfect_power_approx` in `reference/mpu/perfect_powers.c`.
/// The Möbius-sum block in the C source is under `#if 0` and is not ported.
///
/// Returns `None` when `n` is 0, where MPU returns undef.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_perfect_power_approx;
///
/// assert_eq!(nth_perfect_power_approx(10), Some(56));
/// ```
#[must_use]
pub fn nth_perfect_power_approx(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 1 {
        return Some(n);
    }
    if n >= MAX_PERFECT_POW_IDX {
        return Some(MAX_PERFECT_POW);
    }
    let x = n as f64;
    let mut pp =
        x.powf(2.0) + (13.0 / 3.0) * x.powf(4.0 / 3.0) + (32.0 / 15.0) * x.powf(16.0 / 15.0);
    pp += -2.0 * x.powf(5.0 / 3.0) - 2.0 * x.powf(7.0 / 5.0) - 2.0 * x.powf(9.0 / 7.0)
        + 2.0 * x.powf(12.0 / 10.0);
    pp += -2.0 * x.powf(13.0 / 11.0) - 2.0 * x.powf(15.0 / 13.0)
        + 2.0 * x.powf(16.0 / 14.0)
        + 2.0 * x.powf(17.0 / 15.0);
    pp -= 0.48 * x.powf(19.0 / 17.0);
    pp -= 1.5;
    Some(clamp(pp))
}

/// Converts a bound to `u64`, capping at `u64::MAX` as MPU does.
fn clamp(pp: f64) -> u64 {
    if pp >= u64::MAX as f64 {
        u64::MAX
    } else {
        pp as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_and_one() {
        assert_eq!(nth_perfect_power_lower(0), None);
        assert_eq!(nth_perfect_power_upper(0), None);
        assert_eq!(nth_perfect_power_approx(0), None);
        assert_eq!(nth_perfect_power_lower(1), Some(1));
        assert_eq!(nth_perfect_power_upper(1), Some(1));
        assert_eq!(nth_perfect_power_approx(1), Some(1));
    }

    #[test]
    fn small_values_are_ordered() {
        for n in 2..2000_u64 {
            let lo = nth_perfect_power_lower(n).unwrap();
            let hi = nth_perfect_power_upper(n).unwrap();
            let ap = nth_perfect_power_approx(n).unwrap();
            assert!(lo <= ap && ap <= hi, "{n}: {lo} {ap} {hi}");
        }
    }

    #[test]
    fn index_cap() {
        for n in [MAX_PERFECT_POW_IDX, u64::MAX] {
            assert_eq!(nth_perfect_power_lower(n), Some(MAX_PERFECT_POW));
            assert_eq!(nth_perfect_power_upper(n), Some(MAX_PERFECT_POW));
            assert_eq!(nth_perfect_power_approx(n), Some(MAX_PERFECT_POW));
        }
    }

    #[test]
    fn clamp_saturates() {
        assert_eq!(clamp(1e30), u64::MAX);
        assert_eq!(clamp(12.7), 12);
    }
}
