//! Independent laws for floor sums, prime output, and factor removal.
use math_prime_util::{floor_sum, is_prime, print_primes, remove_factors, remove_factors_exp};
use proptest::prelude::*;

proptest! {
    #[test]
    fn floor_sum_matches_direct_sum(n in 0u64..100, m in 1u64..1000,
                                    a in 0u64..1000, b in 0u64..1000) {
        let expected: u64 = (0..n).map(|i| (a * i + b) / m).sum();
        prop_assert_eq!(floor_sum(n, m, a, b), Some(expected));
    }

    #[test]
    fn floor_sum_single_term(m in 1u64.., a in any::<u64>(), b in any::<u64>()) {
        prop_assert_eq!(floor_sum(1, m, a, b), Some(b / m));
    }

    #[test]
    fn printed_primes_match_predicate(lo in 0u64..5000, width in 0u64..200) {
        let hi = lo + width;
        let mut out = Vec::new();
        print_primes(lo, hi, &mut out).unwrap();
        let expected: String = (lo..=hi).filter(|&n| is_prime(n) != 0)
            .map(|p| format!("{p}\n")).collect();
        prop_assert_eq!(out, expected.as_bytes());
    }

    #[test]
    fn factor_removal_reconstructs(n in 1u64.., k in 2u64..1000) {
        let (r, e) = remove_factors_exp(n, k).unwrap();
        prop_assert_ne!(r % k, 0);
        prop_assert_eq!(r.checked_mul(k.checked_pow(e).unwrap()), Some(n));
        prop_assert_eq!(remove_factors(n, k), Some(r));
    }
}
