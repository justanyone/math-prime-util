//! Property tests: bounds that must hold for every input.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
//! Property tests: algebraic laws that must hold for every input.

use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

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

    /// The bounds are ordered: lower <= approx-free midpoint <= upper.
    #[test]
    fn bounds_are_ordered(n in 0u64..1_000_000_000_000) {
        prop_assert!(prime_count_lower(n) <= prime_count_upper(n));
    }

    /// pi is non-decreasing, so the bounds are too.
    #[test]
    fn bounds_are_monotone(a in 0u64..1_000_000_000, d in 0u64..1_000_000) {
        let b = a + d;
        prop_assert!(prime_count_lower(a) <= prime_count_lower(b));
        prop_assert!(prime_count_upper(a) <= prime_count_upper(b));
    }

    /// The approximation is non-decreasing.
    #[test]
    fn approx_is_monotone(a in 0u64..1_000_000_000_000, d in 0u64..1_000_000) {
        prop_assert!(prime_count_approx(a) <= prime_count_approx(a + d));
    }
}
