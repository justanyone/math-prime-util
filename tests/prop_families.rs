//! Perfect-power algebraic laws and native boundaries.
use math_prime_util::{
    is_perfect_power, next_perfect_power, perfect_power_count, prev_perfect_power,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn count_changes_at_positive_perfect_powers(n in 1..=u64::MAX) {
        prop_assert_eq!(perfect_power_count(n) - perfect_power_count(n - 1),
                        u64::from(is_perfect_power(n)));
    }

    #[test]
    fn neighbors_are_strict_and_adjacent(n in any::<u64>()) {
        if let Some(next) = next_perfect_power(n) {
            prop_assert!(next > n);
            prop_assert!(is_perfect_power(next));
            prop_assert!(prev_perfect_power(next).is_some_and(|prev| prev <= n));
            prop_assert_eq!(perfect_power_count(next), perfect_power_count(n) + 1);
        }
        if let Some(prev) = prev_perfect_power(n) {
            prop_assert!(prev < n);
            prop_assert!(is_perfect_power(prev));
            prop_assert!(next_perfect_power(prev).is_none_or(|next| next >= n));
        }
    }

    #[test]
    fn generated_powers_are_recognized(base in 2u64..=1000, exponent in 2u32..=63) {
        if let Some(power) = base.checked_pow(exponent) {
            prop_assert!(is_perfect_power(power));
            prop_assert_eq!(next_perfect_power(power - 1), Some(power));
            prop_assert_eq!(prev_perfect_power(power + 1), Some(power));
        }
    }
}

#[test]
fn small_values_match_enumeration() {
    let powers: Vec<u64> = (0u64..=1000)
        .filter(|&n| n <= 1 || (2u64..=31).any(|b| (2..=10).any(|k| b.checked_pow(k) == Some(n))))
        .collect();
    for n in 0..1000 {
        assert_eq!(is_perfect_power(n), powers.contains(&n));
        assert_eq!(
            next_perfect_power(n),
            powers.iter().copied().find(|&p| p > n)
        );
        assert_eq!(
            prev_perfect_power(n),
            powers.iter().copied().rev().find(|&p| p < n)
        );
        assert_eq!(
            perfect_power_count(n),
            u64::try_from(powers.iter().filter(|&&p| p != 0 && p <= n).count()).unwrap()
        );
    }
}

#[test]
fn native_boundaries_and_repeat_calls() {
    let largest = 18_446_744_065_119_617_025;
    for _ in 0..2 {
        assert_eq!(next_perfect_power(largest - 1), Some(largest));
        assert_eq!(next_perfect_power(largest), None);
        assert_eq!(next_perfect_power(u64::MAX), None);
        assert_eq!(prev_perfect_power(u64::MAX), Some(largest));
        assert_eq!(perfect_power_count(u64::MAX), 4_297_615_581);
        assert!(is_perfect_power(largest));
        assert!(!is_perfect_power(u64::MAX));
    }
}

#[test]
fn concurrent_calls_agree() {
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                assert_eq!(next_perfect_power(64), Some(81));
                assert_eq!(prev_perfect_power(64), Some(49));
                assert_eq!(perfect_power_count(64), 11);
                assert!(is_perfect_power(64));
            });
        }
    });
}

#[test]
fn all_binary_exponents_are_perfect_powers() {
    for k in 2..=63 {
        let n = 1u64 << k;
        assert!(is_perfect_power(n), "exponent {k}");
        assert_eq!(next_perfect_power(n - 1), Some(n));
        assert_eq!(prev_perfect_power(n + 1), Some(n));
    }
}
