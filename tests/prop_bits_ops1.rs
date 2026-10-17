//! Algebraic laws for the native 64-bit bit operations.

use math_prime_util::{bitand, bitandnot, bitnot, bitor, bitxor};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(4000))]

    #[test]
    fn bits_ops1_de_morgan(a in any::<u64>(), b in any::<u64>()) {
        prop_assert_eq!(bitnot(bitand(a, b)), bitor(bitnot(a), bitnot(b)));
        prop_assert_eq!(bitnot(bitor(a, b)), bitand(bitnot(a), bitnot(b)));
    }

    #[test]
    fn bits_ops1_double_negation(a in any::<u64>()) {
        prop_assert_eq!(bitnot(bitnot(a)), a);
    }

    #[test]
    fn bits_ops1_xor_self(a in any::<u64>()) {
        prop_assert_eq!(bitxor(a, a), 0);
    }

    #[test]
    fn bits_ops1_andnot_partition(a in any::<u64>(), b in any::<u64>()) {
        prop_assert_eq!(bitandnot(a, b), bitand(a, bitnot(b)));
        prop_assert_eq!(bitor(bitand(a, b), bitandnot(a, b)), a);
        prop_assert_eq!(bitand(bitand(a, b), bitandnot(a, b)), 0);
        prop_assert_eq!(bitxor(a, b), bitor(bitandnot(a, b), bitandnot(b, a)));
    }

    #[test]
    fn bits_ops1_identities(a in any::<u64>(), b in any::<u64>()) {
        prop_assert_eq!(bitand(a, u64::MAX), a);
        prop_assert_eq!(bitor(a, 0), a);
        prop_assert_eq!(bitxor(a, 0), a);
        prop_assert_eq!(bitand(a, b), bitand(b, a));
        prop_assert_eq!(bitor(a, b), bitor(b, a));
        prop_assert_eq!(bitxor(bitxor(a, b), b), a);
    }
}
