//! Bigint family bounds preserve native answers and count exact powers.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::family_bounds;
use num_bigint::BigUint;
use proptest::prelude::*;

proptest! {
    #[test]
    fn bigint_family_bounds_native(n: u64, small in 0_u64..1000) {
        for n in [n, small] {
            let big = BigUint::from(n);
            macro_rules! compare {
                ($($name:ident),+ $(,)?) => {
                    $(prop_assert_eq!(family_bounds::$name(&big),
                        BigUint::from(math_prime_util::$name(n)));)+
                };
            }
            compare!(
                perfect_power_count_approx,
                perfect_power_count_lower,
                perfect_power_count_upper,
                prime_power_count_approx,
                prime_power_count_lower,
                prime_power_count_upper,
            );
        }
    }

    #[test]
    fn square_boundary(bits in 33_usize..128) {
        let root = BigUint::from(1_u8) << bits;
        let square = &root * &root;
        let before = family_bounds::perfect_power_count_approx(&(&square - 1_u8));
        let at = family_bounds::perfect_power_count_approx(&square);
        prop_assert_eq!(&at, &(&before + 1_u8));
        prop_assert_eq!(&at, &family_bounds::perfect_power_count_lower(&square));
        prop_assert_eq!(&at, &family_bounds::perfect_power_count_upper(&square));
        prop_assert_eq!(&at, &family_bounds::perfect_power_count_approx(&(&square + 1_u8)));
    }
}

#[test]
fn zero_one_and_small_powers() {
    for (n, expected) in [(0_u8, 0_u8), (1, 1), (3, 1), (4, 2), (7, 2), (8, 3), (16, 5)] {
        assert_eq!(family_bounds::perfect_power_count_approx(&BigUint::from(n)), BigUint::from(expected));
    }
}
