//! Check the `BigUint` Goldbach forms against the native forms on the `u64` range.
#![cfg(feature = "bigint")]
use math_prime_util::{self as native, bigint::families as big};
use num_bigint::BigUint;
use proptest::prelude::*;

fn values(v: Vec<u64>) -> Vec<BigUint> {
    v.into_iter().map(BigUint::from).collect()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn native_agreement(n in 0_u64..20_000) {
        let b = BigUint::from(n);
        prop_assert_eq!(big::goldbach_pairs(&b), values(native::goldbach_pairs(n)));
        prop_assert_eq!(big::goldbach_pair_count(&b), BigUint::from(native::goldbach_pair_count(n)));
    }

    /// Odd inputs answer from one primality test at any size.
    #[test]
    fn odd_native_agreement(k in 0_u64..(1 << 40)) {
        let n = u64::MAX - 2 * k;
        let b = BigUint::from(n);
        prop_assert_eq!(big::goldbach_pairs(&b), values(native::goldbach_pairs(n)));
        prop_assert_eq!(big::goldbach_pair_count(&b), BigUint::from(native::goldbach_pair_count(n)));
    }

    /// Past 2^64 an odd `n` has the one pair `2 + (n - 2)` exactly when `n - 2` is prime.
    #[test]
    fn odd_past_u64(k in 0_u64..1_000_000, shift in 64_usize..200) {
        let n = (BigUint::from(1_u8) << shift) + 2 * k + 1_u64;
        let prime = math_prime_util::bigint::primality::is_prob_prime(&(&n - 2_u8)) != 0;
        let pairs = big::goldbach_pairs(&n);
        prop_assert_eq!(pairs.len(), usize::from(prime));
        prop_assert_eq!(big::goldbach_pair_count(&n), BigUint::from(u8::from(prime)));
        if prime {
            prop_assert_eq!(&pairs[0], &BigUint::from(2_u8));
        }
    }
}
