//! Algebraic laws and boundary checks for the native factor API.
use math_prime_util::{divisors, factor, factor_exp, is_prime, trial_factor};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn factor_product(n in any::<u64>()) {
        let factors = factor(n);
        prop_assert!(factors.windows(2).all(|w| w[0] <= w[1]));
        prop_assert_eq!(factors.iter().map(|&p| u128::from(p)).product::<u128>(), u128::from(n));
        if n > 1 {
            prop_assert!(factors.iter().all(|&p| is_prime(p) != 0));
        }
        let expanded: Vec<_> = factor_exp(n).into_iter()
            .flat_map(|(p, e)| (0..e).map(move |_| p)).collect();
        prop_assert_eq!(expanded, factors);
    }

    #[test]
    fn divisor_laws(n in 1u64..=1_000_000, max in 0u64..=1_000_000) {
        let all = divisors(n, None);
        prop_assert_eq!(all.first(), Some(&1));
        prop_assert_eq!(all.last(), Some(&n));
        prop_assert!(all.windows(2).all(|w| w[0] < w[1]));
        prop_assert!(all.iter().all(|d| n % d == 0));
        for (a, b) in all.iter().zip(all.iter().rev()) {
            prop_assert_eq!(a * b, n);
        }
        let expected: Vec<_> = all.into_iter().filter(|&d| d <= max).collect();
        prop_assert_eq!(divisors(n, Some(max)), expected);
    }

    #[test]
    fn bounded_trial_product(n in any::<u64>(), limit in 1u64..=10_000) {
        let factors = trial_factor(n, limit);
        prop_assert_eq!(factors.iter().map(|&p| u128::from(p)).product::<u128>(), u128::from(n));
        if factors.len() > 1 {
            prop_assert!(factors[..factors.len()-1].iter().all(|&p| p <= limit && is_prime(p) != 0));
        }
    }
}

#[test]
fn boundaries_and_repeated_calls() {
    assert_eq!(factor(0), [0]);
    assert_eq!(factor(1), Vec::<u64>::new());
    assert_eq!(factor_exp(0), [(0, 1)]);
    assert_eq!(factor_exp(1), Vec::<(u64, u32)>::new());
    assert_eq!(divisors(0, None), Vec::<u64>::new());
    assert_eq!(divisors(360, Some(0)), Vec::<u64>::new());
    assert_eq!(divisors(360, Some(1)), [1]);
    assert_eq!(divisors(1, None), [1]);
    assert_eq!(trial_factor(49, 6), [49]);
    assert_eq!(trial_factor(49, 7), [7, 7]);
    assert_eq!(trial_factor(4, 1), [4]);
    assert_eq!(trial_factor(2017 * 2017, 0), [2017, 2017]);
    for n in [1, 0, 360, 4_294_967_291, u64::MAX, 360] {
        let expected = factor(n);
        std::thread::scope(|scope| {
            for _ in 0..4 {
                let expected = &expected;
                scope.spawn(move || assert_eq!(&factor(n), expected));
            }
        });
    }
}
