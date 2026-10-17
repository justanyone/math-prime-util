//! Count of integers with no small prime factor, ported from MPU `rough_count`.

/// Counts the integers in `1..=n` with no prime factor below `k`.
///
/// Ported from `rough_count` in Math::Prime::Util (`lib/Math/Prime/Util/PP.pm`,
/// MPU 0.75). The cases `k <= 2` and `k == 3` use the same shortcuts as MPU.
/// For `k >= 4` MPU calls `legendre_phi(n, prime_count(k-1))`. This port uses
/// the plain Legendre recurrence over the primes below `k`, not MPU's faster
/// `legendre_phi`. Its cost grows as `2^pi(k-1)`, so keep `k` small.
///
/// # Examples
///
/// ```
/// use math_prime_util::rough_count;
///
/// // Integers up to 30 with no prime factor below 5: 1, 7, 11, 13, 17, 19, 23, 29.
/// assert_eq!(rough_count(30, 5), 8);
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
fn legendre_phi(x: u64, primes: &[u64]) -> u64 {
    match primes.split_last() {
        None => x,
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
        // Primes below 5 are 2 and 3.
        assert_eq!(rough_count(30, 5), 8);
        assert_eq!(rough_count(1, 5), 1);
        assert_eq!(rough_count(0, 5), 0);
        assert_eq!(legendre_phi(0, &[2]), 0);
        assert!(!is_prime_small(0));
        assert!(!is_prime_small(1));
        assert!(is_prime_small(7));
    }
}
