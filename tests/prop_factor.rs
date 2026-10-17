//! Algebraic laws for bounded ECM splitting.
use math_prime_util::ecm_factor;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn ecm_product(n in any::<u64>(), b1 in 0_u64..100, curves in 0_u64..4) {
        let factors = ecm_factor(n, Some(b1), Some(b1 * 4), Some(curves));
        prop_assert_eq!(factors.iter().map(|&f| u128::from(f)).product::<u128>(), u128::from(n));
        prop_assert!(factors.iter().all(|&f| n == 0 || f > 1 && n % f == 0));
    }

    #[test]
    fn ecm_small_factors(n in 1_u64..1_000_000) {
        let factors = ecm_factor(n * 30, Some(0), Some(0), Some(0));
        prop_assert!(factors.contains(&2));
        prop_assert!(factors.contains(&3));
        prop_assert!(factors.contains(&5));
        prop_assert_eq!(factors.iter().product::<u64>(), n * 30);
    }
}

#[test]
fn ecm_repeated_and_concurrent_calls() {
    let expected = ecm_factor(1_000_036_000_099, None, None, None);
    let handles: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| ecm_factor(1_000_036_000_099, None, None, None)))
        .collect();
    for handle in handles {
        assert_eq!(handle.join().unwrap(), expected);
    }
    assert_eq!(ecm_factor(1_000_036_000_099, None, None, None), expected);
}

#[test]
#[should_panic(expected = "B1 must be specified")]
fn ecm_requires_first_bound() {
    ecm_factor(91, None, Some(100), None);
}
