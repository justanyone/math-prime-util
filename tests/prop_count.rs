//! Prime-count inversion, bounds, and approximation properties.

use math_prime_util::*;
use proptest::prelude::*;

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
