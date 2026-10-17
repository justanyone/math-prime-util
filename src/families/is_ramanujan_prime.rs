//! Ramanujan-prime membership.
#![allow(clippy::cast_possible_truncation)]

use crate::{is_prime, next_prime, prime_count};

/// Return true when `n` is a Ramanujan prime.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `is_ramanujan_prime`.
/// A composite is rejected. A prime must have no prime in
/// `(n/2, next_prime(n)/2]`. The rank `pi(n) - pi(n/2)` then picks the
/// Ramanujan prime to compare with.
///
/// # Examples
///
/// ```
/// use math_prime_util::is_ramanujan_prime;
/// assert!(is_ramanujan_prime(2));
/// assert!(is_ramanujan_prime(11));
/// assert!(!is_ramanujan_prime(13));
/// assert!(!is_ramanujan_prime(1));
/// ```
#[must_use]
pub fn is_ramanujan_prime(n: u64) -> bool {
    if is_prime(n) == 0 {
        return false;
    }
    if n < 17 {
        return n == 2 || n == 11;
    }
    if is_prime(n / 2 + 1) != 0 {
        return false;
    }
    let d = half_gap(n);
    for i in 2..=d {
        // `n / 2 + i` cannot overflow: `n / 2 < 2^63` and `i <= 7`.
        if is_prime(n / 2 + i) != 0 {
            return false;
        }
    }
    // R_v is the only prime n with pi(n) - pi(n/2) = v that is the v-th Ramanujan prime.
    let rank = prime_count(n) - prime_count(n >> 1);
    crate::nth_ramanujan_prime(rank) == Some(n)
}

/// Gap from `n` to the next prime, divided by two.
/// Above `MPU_MAX_PRIME` the `C` code uses the known gap past `UV_MAX`.
fn half_gap(n: u64) -> u64 {
    match next_prime(n) {
        Some(next) => (next - n) / 2,
        None => u64::midpoint(u64::MAX - n, 14),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `constants.h` `MPU_MAX_PRIME` on a 64-bit build. Test-only: `half_gap` is checked against it.
    const MPU_MAX_PRIME: u64 = u64::MAX - 58;

    const SMALL_RAM: [u64; 23] = [
        2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227, 229,
        233, 239, 241,
    ];

    #[test]
    fn table_members_and_nearby_composites() {
        for &prime in &SMALL_RAM {
            assert!(is_ramanujan_prime(prime), "{prime}");
            assert_eq!(is_ramanujan_prime(prime), is_ramanujan_prime(prime));
        }
        for n in [0, 1, 4, 6, 8, 9, 15, 21, 25, 27, 241 + 2] {
            assert!(!is_ramanujan_prime(n), "{n}");
        }
        assert!(!is_ramanujan_prime(u64::MAX));
        assert!(!is_ramanujan_prime(13));
        assert!(!is_ramanujan_prime(19));
        assert!(!is_ramanujan_prime(23));
    }

    #[test]
    fn pre_test_survivor_that_is_not_ramanujan_fails_the_rank_check() {
        // 137 has prime gap 2 and `(137 + 1) / 2 = 69` is composite.
        assert_eq!(is_prime(137), 2);
        assert!(!is_ramanujan_prime(137));
    }

    #[test]
    fn gap_above_the_last_prime_uses_the_word_constant() {
        assert_eq!(half_gap(MPU_MAX_PRIME), 36);
        assert_eq!(half_gap(u64::MAX), 7);
        assert_eq!(half_gap(11), 1);
    }
}
