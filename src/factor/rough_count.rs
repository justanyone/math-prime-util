//! Count of integers with no small prime factor, ported from MPU `rough_count`.

/// Counts the integers in `1..=n` with no prime factor below `k`.
///
/// Ported from `rough_count` in `Math::Prime::Util` (`lib/Math/Prime/Util/PP.pm`,
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
    if n == 0 {
        return 0;
    }
    // A prime above n divides no value in 1..=n, so only primes up to min(k-1, n) matter.
    let top = (k - 1).min(n);
    let primes: Vec<u64> = (2..=top).filter(|&p| is_prime_small(p)).collect();
    legendre_phi(n, &primes)
}

/// Trial-division primality test for the small bound `k`.
fn is_prime_small(p: u64) -> bool {
    p >= 2 && (2..=p).take_while(|d| d * d <= p).all(|d| p % d != 0)
}

/// `phi(x, a)`: integers in `1..=x` not divisible by any prime in `primes`.
///
/// Recurrence and early exits of `legendre_phi` in `reference/mpu/legendre_phi.c`
/// (MPU 0.75), written as `phi(x, a) = x - sum_i phi(x / p_i, i - 1)`, so the
/// recursion depth is the number of nested divisions by primes, not `a`.
/// `primes` must hold every prime below its last element. When `x` is at most
/// the last prime, every `m` in `2..=x` has a prime factor in `primes`, so only
/// `1` survives.
fn legendre_phi(x: u64, primes: &[u64]) -> u64 {
    match primes.split_last() {
        None => x,
        Some((&p, _)) if x <= p => u64::from(x > 0),
        Some(_) => {
            let mut count = x;
            for (i, &p) in primes.iter().enumerate() {
                let y = x / p;
                if y == 0 {
                    break;
                }
                count -= legendre_phi(y, &primes[..i]);
            }
            count
        }
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

    /// Brute-force count of `1..=n` with no divisor in `2..k`.
    fn brute_rough(n: u64, k: u64) -> u64 {
        (1..=n).filter(|&m| (2..k).all(|d| m % d != 0)).count() as u64
    }

    #[test]
    fn huge_k_with_small_bound_is_fast() {
        // Before the cap at n, this enumerated every prime below u64::MAX.
        assert_eq!(rough_count(0, u64::MAX), 0);
        assert_eq!(rough_count(1, u64::MAX), 1);
        assert_eq!(rough_count(1000, u64::MAX), 1);
    }

    #[test]
    fn matches_brute_force_small_domain() {
        for n in 0..=600 {
            for k in 1..=40 {
                assert_eq!(rough_count(n, k), brute_rough(n, k), "n={n} k={k}");
            }
        }
    }

    #[test]
    fn matches_brute_force_larger_bounds() {
        assert_eq!(rough_count(200_000, 100), brute_rough(200_000, 100));
        assert_eq!(rough_count(50_000, 1000), brute_rough(50_000, 1000));
    }
}
