//! Counts and bounds for integer families beyond the native range.

use num_bigint::{BigInt, BigUint};
use num_traits::ToPrimitive;

use super::{arith::rootint, counts};

fn perfect_power_count(n: &BigUint) -> BigUint {
    if *n < BigUint::from(8_u8) {
        return BigUint::from(
            u8::from(*n >= BigUint::from(1_u8)) + u8::from(*n >= BigUint::from(4_u8)),
        );
    }
    let mut sum = BigInt::from(1_u8);
    for k in 2..n.bits() {
        let mu = crate::moebius(k);
        if mu != 0 {
            let exponent = u32::try_from(k).expect("root exponent fits u32");
            sum -= BigInt::from(mu) * BigInt::from(rootint(n, exponent) - 1_u8);
        }
    }
    sum.to_biguint()
        .expect("perfect power count is nonnegative")
}

fn prime_power_count(n: &BigUint, count: fn(&BigUint) -> BigUint) -> BigUint {
    let mut sum = count(n);
    for k in 2..n.bits() {
        let exponent = u32::try_from(k).expect("root exponent fits u32");
        sum += count(&rootint(n, exponent));
    }
    sum
}

/// Estimate the number of perfect powers at most `n`.
///
/// Port of MPU 0.75 `perfect_powers.c::perfect_power_count_approx`.
/// The count is exact and includes 1.
///
/// ```
/// use math_prime_util::bigint::family_bounds::perfect_power_count_approx;
/// use num_bigint::BigUint;
/// assert_eq!(perfect_power_count_approx(&BigUint::from(100_u8)), BigUint::from(13_u8));
/// ```
#[must_use]
pub fn perfect_power_count_approx(n: &BigUint) -> BigUint {
    perfect_power_count(n)
}

/// Return a lower bound for the number of perfect powers at most `n`.
///
/// Port of MPU 0.75 `perfect_powers.c::perfect_power_count_lower`.
/// The count is exact and includes 1.
///
/// ```
/// use math_prime_util::bigint::family_bounds::perfect_power_count_lower;
/// use num_bigint::BigUint;
/// assert_eq!(perfect_power_count_lower(&BigUint::from(100_u8)), BigUint::from(13_u8));
/// ```
#[must_use]
pub fn perfect_power_count_lower(n: &BigUint) -> BigUint {
    perfect_power_count(n)
}

/// Return an upper bound for the number of perfect powers at most `n`.
///
/// Port of MPU 0.75 `perfect_powers.c::perfect_power_count_upper`.
/// The count is exact and includes 1.
///
/// ```
/// use math_prime_util::bigint::family_bounds::perfect_power_count_upper;
/// use num_bigint::BigUint;
/// assert_eq!(perfect_power_count_upper(&BigUint::from(100_u8)), BigUint::from(13_u8));
/// ```
#[must_use]
pub fn perfect_power_count_upper(n: &BigUint) -> BigUint {
    perfect_power_count(n)
}

/// Estimate the number of prime powers at most `n`.
///
/// Port of MPU 0.75 `prime_powers.c::prime_power_count_approx`.
/// Sum prime count bounds at each integer root.
///
/// ```
/// use math_prime_util::bigint::family_bounds::prime_power_count_approx;
/// use num_bigint::BigUint;
/// assert_eq!(prime_power_count_approx(&BigUint::from(100_u8)), BigUint::from(35_u8));
/// ```
#[must_use]
pub fn prime_power_count_approx(n: &BigUint) -> BigUint {
    if let Some(n) = n.to_u64() {
        return BigUint::from(crate::prime_power_count_approx(n));
    }
    prime_power_count(n, counts::prime_count_approx)
}

/// Return a lower bound for the number of prime powers at most `n`.
///
/// Port of MPU 0.75 `prime_powers.c::prime_power_count_lower`.
/// Sum prime count bounds at each integer root.
///
/// ```
/// use math_prime_util::bigint::family_bounds::prime_power_count_lower;
/// use num_bigint::BigUint;
/// assert_eq!(prime_power_count_lower(&BigUint::from(100_u8)), BigUint::from(35_u8));
/// ```
#[must_use]
pub fn prime_power_count_lower(n: &BigUint) -> BigUint {
    if let Some(n) = n.to_u64() {
        return BigUint::from(crate::prime_power_count_lower(n));
    }
    prime_power_count(n, counts::prime_count_lower)
}

/// Return an upper bound for the number of prime powers at most `n`.
///
/// Port of MPU 0.75 `prime_powers.c::prime_power_count_upper`.
/// Sum prime count bounds at each integer root.
///
/// ```
/// use math_prime_util::bigint::family_bounds::prime_power_count_upper;
/// use num_bigint::BigUint;
/// assert_eq!(prime_power_count_upper(&BigUint::from(100_u8)), BigUint::from(35_u8));
/// ```
#[must_use]
pub fn prime_power_count_upper(n: &BigUint) -> BigUint {
    if let Some(n) = n.to_u64() {
        return BigUint::from(crate::prime_power_count_upper(n));
    }
    prime_power_count(n, counts::prime_count_upper)
}
