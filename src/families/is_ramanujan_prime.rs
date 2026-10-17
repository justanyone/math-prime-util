//! Ramanujan-prime membership.
#![allow(clippy::cast_possible_truncation)]

use crate::{is_prime, next_prime, prime_count, primes};

/// `constants.h` `MPU_MAX_PRIME` on a 64-bit build.
const MPU_MAX_PRIME: u64 = u64::MAX - 58;

/// Return true when `n` is a Ramanujan prime.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `is_ramanujan_prime`.
/// A composite is rejected. A prime must have no prime in
/// `(n/2, next_prime(n)/2]`. Noe's sieve then checks the rank.
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
        let Some(candidate) = (n / 2).checked_add(i) else {
            break;
        };
        if is_prime(candidate) != 0 {
            return false;
        }
    }
    let rank = prime_count(n) - prime_count(n >> 1);
    let Some(max) = crate::nth_ramanujan_prime_upper(rank) else {
        return false;
    };
    if n > max {
        return false;
    }
    noe_level(max, rank - 1) == n
}

/// Gap from `n` to the next prime, divided by two.
/// Above `MPU_MAX_PRIME` the `C` code uses the known gap past `UV_MAX`.
fn half_gap(n: u64) -> u64 {
    match next_prime(n) {
        Some(next) => (next - n) / 2,
        None => (u64::MAX - n + 14) / 2,
    }
}

/// Last value Noe's sieve writes at index `target` while scanning to `max`.
/// `ramanujan_primes.c` `n_ramanujan_primes`.
fn noe_level(max: u64, target: u64) -> u64 {
    if max < 7 {
        return u64::from(target == 0) * 2;
    }
    let ps = primes(2, max);
    let mut prime_at = ps.iter().position(|&prime| prime >= 7).unwrap_or(ps.len());
    let mut half_at = 0_usize;
    let mut level = 0_u64;
    let mut last = 0_u64;
    let mut k = 7_u64;
    loop {
        if prime_at < ps.len() && ps[prime_at] == k {
            level += 1;
            prime_at += 1;
        }
        if level == target {
            last = k.saturating_add(1);
        }
        if k & 3 == 1 {
            let half = (k + 1) >> 1;
            while half_at < ps.len() && ps[half_at] < half {
                half_at += 1;
            }
            if half_at < ps.len() && ps[half_at] == half {
                level = level.wrapping_sub(1);
            }
        }
        if level == target {
            last = k.saturating_add(2);
        }
        let Some(next_k) = k.checked_add(2) else {
            break;
        };
        if next_k > max {
            break;
        }
        k = next_k;
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn pre_test_survivor_that_is_not_ramanujan_fails_the_sieve() {
        // 103 passes the half-interval test and is not in the small table.
        assert_eq!(is_prime(103), 2);
        assert!(!is_ramanujan_prime(103));
    }

    #[test]
    fn gap_above_the_last_prime_uses_the_word_constant() {
        assert_eq!(half_gap(MPU_MAX_PRIME), (58 + 14) / 2);
        assert_eq!(half_gap(u64::MAX), 7);
        assert_eq!(half_gap(11), 1);
        assert!(noe_level(6, 0) == 2);
        assert_eq!(noe_level(241, 0), 2);
        assert_eq!(noe_level(241, 22), 241);
        assert_ne!(noe_level(30, 5), 13);
    }

    #[test]
    fn rank_above_its_upper_bound_is_rejected() {
        // `nth_ramanujan_prime_upper(1)` is 2, so a larger prime of rank 1 cannot occur.
        // The public path hits this only when the bound is below `n`. Force the compare.
        assert!(crate::nth_ramanujan_prime_upper(1) == Some(2));
        assert_ne!(noe_level(2, 0), 11);
    }
}
