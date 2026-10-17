//! Count of integers with no small prime factor, ported from MPU `rough_count`.

/// Counts the integers in `1..=n` with no prime factor below `k`.
///
/// Ported from `rough_count` in Math::Prime::Util (`lib/Math/Prime/Util/PP.pm`,
/// `sub rough_count`, MPU 0.75). MPU has no C version of `rough_count`. The
/// cases `k <= 2` and `k == 3` use the same shortcuts as MPU. For `k >= 4`
/// MPU calls `legendre_phi(n, prime_count(k-1))`, and this port uses the
/// recurrence of `legendre_phi` in `reference/mpu/legendre_phi.c`
/// (`legendre_phi`) over the primes below `k`, with its early exits. It does
/// not use MPU's table and cache code, so keep `k` small.
///
/// # Examples
///
/// ```
/// use math_prime_util::rough_count;
///
/// // Integers up to 30 with no prime factor below 5: 1, 5, 7, 11, 13, 17, 19, 23, 25, 29.
/// assert_eq!(rough_count(30, 5), 10);
/// assert_eq!(rough_count(30, 2), 30);
/// ```
#[must_use]
pub fn rough_count(n: u64, k: u64) -> u64 {
    if k <= 2 {
        return n;
    }
    if k == 3 {
        // Msubint(n, Mrshiftint(n)): odd integers up to n.
        return n - (n >> 1);
    }
    let primes: Vec<u64> = (2..k).filter(|&p| is_prime_small(p)).collect();
    legendre_phi(n, &primes)
}

/// Trial-division primality test for the small bound `k`.
fn is_prime_small(p: u64) -> bool {
    p >= 2 && (2..).take_while(|d| d * d <= p).all(|d| p % d != 0)
}

/// `phi(x, a)`: integers in `1..=x` not divisible by any prime in `primes`.
///
/// Recurrence and early exits of `legendre_phi` in `reference/mpu/legendre_phi.c`.
/// `primes` must hold every prime below its last element. When `x` is at most
/// the last prime, every `m` in `2..=x` has a prime factor in `primes`, so only
/// `1` survives.
fn legendre_phi(x: u64, primes: &[u64]) -> u64 {
    match primes.split_last() {
        None => x,
        Some((&p, _)) if x <= p => u64::from(x > 0),
        Some((&p, rest)) => legendre_phi(x, rest) - legendre_phi(x / p, rest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_k_shortcuts() {
        assert_eq!(rough_count(0, 0), 0);
        assert_eq!(rough_count(17, 1), 17);
        assert_eq!(rough_count(17, 2), 17);
        assert_eq!(rough_count(17, 3), 9);
        assert_eq!(rough_count(0, 3), 0);
    }

    #[test]
    fn legendre_branch() {
        // Primes below 5 are 2 and 3. Integers in 1..=30 with no factor 2 or 3: 1, 5, 7, 11, 13, 17, 19, 23, 25, 29.
        assert_eq!(rough_count(30, 5), 10);
        assert_eq!(rough_count(1, 5), 1);
        assert_eq!(rough_count(0, 5), 0);
        assert_eq!(legendre_phi(0, &[2]), 0);
        assert!(!is_prime_small(0));
        assert!(!is_prime_small(1));
        assert!(is_prime_small(7));
    }

    #[test]
    fn large_k_with_zero_bound_is_fast() {
        // Without the early exit this expands about 2^168 calls for the 168 primes below 1000.
        assert_eq!(rough_count(0, 1000), 0);
        assert_eq!(rough_count(1, 1000), 1);
    }

    #[test]
    fn early_exit_below_last_prime() {
        // Primes below 8; x = 7 is at most the last prime, so only 1 survives.
        assert_eq!(legendre_phi(7, &[2, 3, 5, 7]), 1);
        assert_eq!(legendre_phi(0, &[2, 3, 5, 7]), 0);
    }
}
