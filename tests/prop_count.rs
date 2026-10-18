//! Property tests: algebraic laws that must hold for every input.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]

use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    /// The lower bound never exceeds the exact count.
    #[test]
    fn lower_bound_below_exact(n in 0u64..2_000_000) {
        prop_assert!(prime_count_lower(n) <= prime_count(n));
    }

    /// The upper bound never falls below the exact count.
    #[test]
    fn upper_bound_above_exact(n in 0u64..2_000_000) {
        prop_assert!(prime_count_upper(n) >= prime_count(n));
    }

    /// The lower bound never exceeds the upper bound.
    #[test]
    fn bounds_are_ordered(n in 0u64..1_000_000_000_000) {
        prop_assert!(prime_count_lower(n) <= prime_count_upper(n));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]

    /// The exact count agrees with a sieve.
    #[test]
    fn count_matches_sieve(n in 0_u64..100_000) {
        prop_assert_eq!(prime_count(n), primes(2, n).len() as u64);
    }

    /// Legendre's recurrence: `phi(x, a) = phi(x, a - 1) - phi(x / p_a, a - 1)`.
    #[test]
    fn phi_recurrence(x in 0_u64..100_000, a in 1_usize..20) {
        let p = primes(2, 100)[a - 1];
        prop_assert_eq!(legendre_phi(x, a as u64),
            legendre_phi(x, a as u64 - 1) - legendre_phi(x / p, a as u64 - 1));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    /// The nth-prime bounds bracket the `n`th prime.
    #[test]
    fn nth_prime_bounds_bracket(n in 1_u64..300_000) {
        let p = nth_prime(n).unwrap();
        prop_assert!(nth_prime_lower(n).unwrap() <= p);
        prop_assert!(p <= nth_prime_upper(n).unwrap());
    }

    /// The same for larger `n`, through the exact count: `lower(n) <= p_n`
    /// exactly when fewer than `n` primes lie below `lower(n)`, and
    /// `p_n <= upper(n)` exactly when at least `n` primes lie up to `upper(n)`.
    #[test]
    fn nth_prime_bounds_bracket_by_count(n in 300_000_u64..100_000_000) {
        prop_assert!(prime_count(nth_prime_lower(n).unwrap() - 1) < n);
        prop_assert!(prime_count(nth_prime_upper(n).unwrap()) >= n);
    }

    /// The estimate lies within 1% (plus a small constant) of the `n`th prime.
    #[test]
    fn nth_prime_approx_is_close(n in 1_u64..300_000) {
        let p = nth_prime(n).unwrap();
        prop_assert!(nth_prime_approx(n).unwrap().abs_diff(p) <= p / 100 + 20);
    }

    /// All three are exact below index 306, where MPU reads its prime table.
    #[test]
    fn nth_prime_family_exact_below_table_end(n in 1_u64..306) {
        let p = nth_prime(n);
        prop_assert_eq!(nth_prime_lower(n), p);
        prop_assert_eq!(nth_prime_upper(n), p);
        prop_assert_eq!(nth_prime_approx(n), p);
    }

    /// No 64-bit answer exists past the index of the largest 64-bit prime.
    #[test]
    fn nth_prime_family_none_past_max_index(n in 425_656_284_035_217_744_u64..) {
        prop_assert_eq!(nth_prime_lower(n), None);
        prop_assert_eq!(nth_prime_upper(n), None);
        prop_assert_eq!(nth_prime_approx(n), None);
    }
}
