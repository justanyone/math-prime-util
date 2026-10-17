//! Algebraic laws for prime power counts.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn root_sum_and_bounds(n in 0_u64..1_000_000) {
        let exact = prime_power_count(n);
        let expected: u64 = (1..=n.checked_ilog2().unwrap_or(0))
            .map(|k| prime_count(rootint(n, k))).sum();
        prop_assert_eq!(exact, expected);
        prop_assert!(prime_power_count_lower(n) <= exact);
        prop_assert!(prime_power_count_upper(n) >= exact);
        prop_assert_eq!(prime_power_count_approx(n), exact);
    }

    #[test]
    fn increments_are_prime_powers(n in 2_u64..10_000) {
        let is_power = (1..=n.ilog2()).any(|k| {
            let r = rootint(n, k);
            r.checked_pow(k) == Some(n) && is_prime(r) != 0
        });
        prop_assert_eq!(prime_power_count(n) - prime_power_count(n - 1), u64::from(is_power));
    }
}

#[test]
fn small_counts_and_repeat_calls() {
    for (n, expected) in [0, 0, 1, 2, 3, 4, 4, 5, 6, 7].into_iter().enumerate() {
        let n = n as u64;
        for _ in 0..2 {
            assert_eq!(prime_power_count(n), expected);
            assert_eq!(prime_power_count_lower(n), expected);
            assert_eq!(prime_power_count_upper(n), expected);
            assert_eq!(prime_power_count_approx(n), expected);
        }
    }
}

#[test]
fn large_estimates_and_concurrent_calls() {
    let inputs = [
        10_000_000_000_000_001,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
        u64::MAX,
    ];
    std::thread::scope(|scope| {
        for n in inputs {
            scope.spawn(move || {
                let estimate = prime_power_count_approx(n);
                assert!(prime_power_count_lower(n) <= estimate);
                assert!(estimate <= prime_power_count_upper(n));
                assert_eq!(prime_power_count_approx(n), estimate);
            });
        }
    });
}
