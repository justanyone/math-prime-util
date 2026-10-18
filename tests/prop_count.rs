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
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn prop_count_bounds_bracket_nth_prime(n in 1u64..=20_000) {
        let p = nth_prime(n).unwrap();
        prop_assert!(nth_prime_lower(n).unwrap() <= p);
        prop_assert!(p <= nth_prime_upper(n).unwrap());
    }

    #[test]
    fn prop_count_bounds_are_monotone(n in 1u64..=1_000_000_000) {
        prop_assert!(nth_prime_lower(n).unwrap() <= nth_prime_lower(n + 1).unwrap());
        prop_assert!(nth_prime_upper(n).unwrap() <= nth_prime_upper(n + 1).unwrap());
    }

    #[test]
    fn prop_count_inverts_nth_prime(n in 1u64..=20_000) {
        let p = nth_prime(n).unwrap();
        prop_assert_eq!(prime_count(p), n);
        prop_assert_eq!(prime_count(p - 1), n - 1);
    }

    #[test]
    fn prop_count_approximation_is_close(n in 306u64..=20_000) {
        let p = nth_prime(n).unwrap();
        let estimate = nth_prime_approx(n).unwrap();
        prop_assert!(estimate.abs_diff(p) <= p / 100 + 20);
    }

    #[test]
    fn prop_count_prime_count_of_bound(n in 1u64..=20_000) {
        // pi(p_n) = n, so pi(upper(n)) >= n and pi(lower(n)) <= n.
        prop_assert!(prime_count(nth_prime_upper(n).unwrap()) >= n);
        prop_assert!(prime_count(nth_prime_lower(n).unwrap()) <= n);
    }
}
