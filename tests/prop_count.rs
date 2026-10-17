//! Algebraic laws for exact prime counts and the partial sieve.
use math_prime_util::{legendre_phi, prime_count, primes};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn count_matches_sieve(n in 0_u64..100_000) {
        prop_assert_eq!(prime_count(n), primes(2, n).len() as u64);
    }
    #[test]
    fn phi_recurrence(x in 0_u64..100_000, a in 1_usize..20) {
        let p = primes(2, 100)[a - 1];
        prop_assert_eq!(legendre_phi(x, a as u64),
            legendre_phi(x, a as u64 - 1) - legendre_phi(x / p, a as u64 - 1));
    }
}
