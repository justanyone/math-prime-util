//! Bit-position laws, checked against a bit-by-bit scan.

use math_prime_util::{bitclear, bitflip, bitscan0, bitscan1, bitset, bittest};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(4000))]

    #[test]
    fn set_clear_and_test(n in any::<u64>(), index in 0u64..64) {
        let set = bitset(n, index).unwrap();
        prop_assert!(bittest(set, index));
        prop_assert!(!bittest(bitclear(n, index), index));
        prop_assert_eq!(bitset(set, index), Some(set));
        prop_assert_eq!(bitclear(set, index), bitclear(n, index));
        // Bits on either side of the target must be unchanged.
        for other in 0..64 {
            if other != index {
                prop_assert_eq!(bittest(set, other), bittest(n, other));
                prop_assert_eq!(bittest(bitclear(n, index), other), bittest(n, other));
            }
        }
    }

    #[test]
    fn flip_twice_is_identity(n in any::<u64>(), index in 0u64..64) {
        let flipped = bitflip(n, index).unwrap();
        prop_assert_eq!(bitflip(flipped, index), Some(n));
        prop_assert_eq!(bittest(flipped, index), !bittest(n, index));
        prop_assert_eq!(flipped ^ n, 1 << index);
    }

    #[test]
    fn scan_finds_lowest_bit(n in any::<u64>(), start in 0u64..=80) {
        let one = (start..64).find(|&index| (n >> index) & 1 == 1);
        let zero = (start..64).find(|&index| (n >> index) & 1 == 0)
            .unwrap_or(start.max(64));
        prop_assert_eq!(bitscan1(n, start), one);
        prop_assert_eq!(bitscan0(n, start), zero);
    }

    #[test]
    fn high_positions(n in any::<u64>(), index in 64u64..) {
        prop_assert_eq!(bitset(n, index), None);
        prop_assert_eq!(bitflip(n, index), None);
        prop_assert_eq!(bitclear(n, index), n);
        prop_assert!(!bittest(n, index));
        prop_assert_eq!(bitscan0(n, index), index);
        prop_assert_eq!(bitscan1(n, index), None);
    }
}
