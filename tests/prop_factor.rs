//! Product, bound, and repeatability laws for bounded factor methods.
use math_prime_util::{pbrent_factor, pminus1_factor, pplus1_factor, prho_factor};
use proptest::prelude::*;

fn methods(n: u64, bound: u64) -> [Vec<u64>; 4] {
    [prho_factor(n, Some(bound)), pbrent_factor(n, Some(bound), None),
     pminus1_factor(n, Some(bound), Some(bound.saturating_mul(10))),
     pplus1_factor(n, Some(bound))]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn factors_preserve_product(n in any::<u64>(), bound in 0_u64..256) {
        for factors in methods(n, bound) {
            prop_assert_eq!(factors.iter().map(|&f| u128::from(f)).product::<u128>(), u128::from(n));
            prop_assert!(factors.windows(2).all(|w| w[0] <= w[1]));
            prop_assert!(n < 2 || factors.iter().all(|&f| f > 1 && n % f == 0));
        }
    }
}

#[test]
fn edge_inputs_and_zero_work() {
    for n in [0, 1, 2, 3, 5, 7, 8, 25, 49, 77, 91, u64::MAX] {
        let first = methods(n, 0);
        assert_eq!(first, methods(n, 0));
        for factors in first {
            assert_eq!(factors.iter().map(|&f| u128::from(f)).product::<u128>(), u128::from(n));
        }
    }
    assert_eq!(prho_factor(77, Some(0)), vec![77]);
    assert_eq!(pbrent_factor(77, Some(0), Some(0)), vec![77]);
}

#[test]
fn concurrent_calls_are_independent() {
    let expected = methods(1_000_036_000_099, 256);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let expected = &expected;
            scope.spawn(move || assert_eq!(&methods(1_000_036_000_099, 256), expected));
        }
    });
}
