use super::is_semiprime;
use super::is_semiprime::{SMALL_PRIMES, factors};

/// Return whether `n` has exactly `k` prime factors, with multiplicity.
///
/// Port of MPU 0.75 `util.c::is_almost_prime` and `factor.c::prime_bigomega`.
///
/// ```
/// use math_prime_util::is_almost_prime;
/// assert!(is_almost_prime(3, 12));
/// assert!(is_almost_prime(0, 1));
/// ```
pub fn is_almost_prime(mut k: u64, mut n: u64) -> bool {
    if k == 0 {
        return n == 1;
    }
    if k == 1 {
        return crate::is_prime(n) != 0;
    }
    if k == 2 {
        return is_semiprime(n);
    }
    if k >= 64 || n >> k == 0 {
        return false;
    }
    for p in [2, 3, 5, 7] {
        while k > 0 && n % p == 0 {
            k -= 1;
            n /= p;
        }
    }
    let mut p = 11;
    if k >= 5 {
        let mut sp = 5;
        while k > 1 && n > 1 && sp < SMALL_PRIMES.len() - 1 {
            p = SMALL_PRIMES[sp];
            if below_power(n, p, k) {
                return false;
            }
            while k > 0 && n % p == 0 {
                k -= 1;
                n /= p;
            }
            sp += 1;
        }
        p = SMALL_PRIMES[sp];
    }
    match k {
        0 => n == 1,
        1 => crate::is_prime(n) != 0,
        2 => is_semiprime(n),
        _ => !below_power(n, p, k) && factors(n).len() as u64 == k,
    }
}

// util.c::ipowsafe saturates on overflow, rather than wrapping.
pub(super) fn below_power(n: u64, p: u64, k: u64) -> bool {
    let Ok(k) = u32::try_from(k) else {
        return n < u64::MAX;
    };
    n < p.checked_pow(k).unwrap_or(u64::MAX)
}
