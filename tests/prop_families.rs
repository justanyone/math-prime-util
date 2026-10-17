//! Algebraic laws for positive perfect powers.
use math_prime_util::{
    is_power_exp, nth_perfect_power, perfect_power_count_approx, perfect_power_count_lower,
    perfect_power_count_upper,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn count_bounds_are_exact(n in any::<u64>()) {
        let count = perfect_power_count_approx(n);
        prop_assert_eq!(perfect_power_count_lower(n), count);
        prop_assert_eq!(perfect_power_count_upper(n), count);
        if n > 0 {
            let previous = perfect_power_count_approx(n - 1);
            let increment = u64::from(n == 1 || is_power_exp(n) > 1);
            prop_assert_eq!(count, previous + increment);
        }
    }

    #[test]
    fn nth_and_count_are_inverses(n in 1u64..=4_297_615_581) {
        let power = nth_perfect_power(n).unwrap();
        prop_assert_eq!(perfect_power_count_approx(power), n);
        prop_assert_eq!(perfect_power_count_approx(power - 1), n - 1);
        prop_assert!(power == 1 || is_power_exp(power) > 1);
    }
}

#[test]
fn exhaustive_small_powers() {
    let mut count = 0;
    for n in 0..=10_000_u64 {
        if n == 1
            || (n > 1
                && (2..=n.ilog2())
                    .any(|k| (2..=100).any(|base: u64| base.checked_pow(k) == Some(n))))
        {
            count += 1;
            assert_eq!(nth_perfect_power(count), Some(n));
        }
        assert_eq!(perfect_power_count_approx(n), count);
    }
}

#[test]
fn repeated_and_concurrent_calls_agree() {
    let expected: Vec<_> = (0..100).map(nth_perfect_power).collect();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let expected = &expected;
            scope.spawn(move || {
                for (n, &power) in (0u64..).zip(expected) {
                    assert_eq!(nth_perfect_power(n), power);
                }
            });
        }
    });
}
