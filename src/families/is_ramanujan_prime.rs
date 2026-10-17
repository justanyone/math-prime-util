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
    // `R_rank <= max`. A larger prime is not that Ramanujan prime.
    if n > max || rank == 0 {
        return false;
    }
    let mink = window_start(n);
    if mink > max {
        return false;
    }
    noe_level(mink, max, rank - 1) == n
}

/// First odd `k` of Noe's scan. The write that stores `n` uses `k = n - 2`.
fn window_start(n: u64) -> u64 {
    let mut mink = n.saturating_sub(4);
    if mink < 15 {
        mink = 15;
    }
    if mink % 2 == 0 {
        mink -= 1;
    }
    mink
}

/// Gap from `n` to the next prime, divided by two.
/// Above `MPU_MAX_PRIME` the `C` code uses the known gap past `UV_MAX`.
fn half_gap(n: u64) -> u64 {
    match next_prime(n) {
        Some(next) => (next - n) / 2,
        None => (u64::MAX - n + 14) / 2,
    }
}

/// Last value Noe's sieve writes at index `target` on `mink..=max`.
/// `ramanujan_primes.c` `n_range_ramanujan_primes` gives the start count.
fn noe_level(mink: u64, max: u64, target: u64) -> u64 {
    if max < mink {
        return 0;
    }
    let mut level = 1 + prime_count(mink - 2) - prime_count((mink - 1) >> 1);
    let ps = primes(mink, max);
    let half_lo = (mink + 1) >> 1;
    let half_hi = (max + 1) >> 1;
    let hs = primes(half_lo, half_hi);
    let mut prime_at = 0_usize;
    let mut half_at = 0_usize;
    let mut last = 0_u64;
    let mut k = mink;
    loop {
        if prime_at < ps.len() && ps[prime_at] < k {
            prime_at += 1;
            continue;
        }
        if prime_at < ps.len() && ps[prime_at] == k {
            level += 1;
            prime_at += 1;
        }
        if level == target {
            last = k.saturating_add(1);
        }
        if k & 3 == 1 {
            let half = (k + 1) >> 1;
            while half_at < hs.len() && hs[half_at] < half {
                half_at += 1;
            }
            if half_at < hs.len() && hs[half_at] == half {
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
        // 137 has prime gap 2 and `(137 + 1) / 2 = 69` is composite.
        // It is not a Ramanujan prime, so the sieve must reject it.
        assert_eq!(is_prime(137), 2);
        assert!(!is_ramanujan_prime(137));
        assert!(crate::nth_ramanujan_prime_upper(1) == Some(2));
        assert_eq!(window_start(17), 15);
        assert_eq!(window_start(29), 25);
    }

    #[test]
    fn gap_above_the_last_prime_uses_the_word_constant() {
        assert_eq!(half_gap(MPU_MAX_PRIME), (58 + 14) / 2);
        assert_eq!(half_gap(u64::MAX), 7);
        assert_eq!(half_gap(11), 1);
        assert_eq!(noe_level(15, 241, 0), 2);
        assert_eq!(noe_level(15, 241, 22), 241);
        assert_eq!(noe_level(30, 10, 0), 0);
        assert_ne!(noe_level(15, 40, 5), 13);
    }
}
