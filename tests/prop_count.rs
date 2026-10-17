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
