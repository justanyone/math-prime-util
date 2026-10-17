//! Prime factorization used by modular roots and the public factor API.
use super::is_carmichael::prime_factors;

/// Return the prime factors of `n` in ascending order, with multiplicity.
///
/// Port of MPU 0.75 `factor.c`, `factor`.
///
/// `0` factors as `[0]`. `1` has no prime factors.
///
/// ```
/// assert_eq!(math_prime_util::factor(12), vec![2, 2, 3]);
/// assert_eq!(math_prime_util::factor(0), vec![0]);
/// assert!(math_prime_util::factor(1).is_empty());
/// ```
#[must_use]
pub fn factor(n: u64) -> Vec<u64> {
    if n == 0 { vec![0] } else { prime_factors(n) }
}

/// Return the prime-power factorization of `n` as `(prime, exponent)` pairs.
///
/// Port of MPU 0.75 `factor.c`, `factorintp` (Perl `factor_exp`).
/// [`factor`] lists the same primes with multiplicity.
///
/// `0` is `[(0, 1)]`. `1` has an empty factorization.
///
/// ```
/// assert_eq!(math_prime_util::factor_exp(12), vec![(2, 2), (3, 1)]);
/// assert_eq!(math_prime_util::factor_exp(0), vec![(0, 1)]);
/// assert!(math_prime_util::factor_exp(1).is_empty());
/// ```
#[must_use]
pub fn factor_exp(n: u64) -> Vec<(u64, u32)> {
    let mut powers = Vec::new();
    for prime in factor(n) {
        match powers.last_mut() {
            Some((p, exponent)) if *p == prime => *exponent += 1,
            _ => powers.push((prime, 1)),
        }
    }
    powers
}

#[cfg(test)]
mod tests {
    use super::{factor, factor_exp};

    #[test]
    fn edges_and_repeated_primes() {
        assert_eq!(factor(0), vec![0]);
        assert!(factor(1).is_empty());
        assert_eq!(factor(32), vec![2, 2, 2, 2, 2]);
        assert_eq!(factor(1_000_003 * 1_000_033), vec![1_000_003, 1_000_033]);
        assert_eq!(factor_exp(0), vec![(0, 1)]);
        assert!(factor_exp(1).is_empty());
        assert_eq!(factor_exp(12), vec![(2, 2), (3, 1)]);
        assert_eq!(factor_exp(47), vec![(47, 1)]);
        assert_eq!(
            factor_exp(1_000_003 * 1_000_033),
            vec![(1_000_003, 1), (1_000_033, 1)]
        );
    }
}
