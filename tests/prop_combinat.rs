//! Algebraic laws and native boundary checks for the factorial family.
use math_prime_util::{factorial, fubini, multifactorial, subfactorial};
use proptest::prelude::*;

proptest! {
    #[test]
    fn factorial_recurrence(n in 1_u64..=20) {
        prop_assert_eq!(factorial(n), factorial(n - 1).and_then(|v| v.checked_mul(n)));
        prop_assert_eq!(multifactorial(n, 1), factorial(n));
    }

    #[test]
    fn multifactorial_recurrence(n in any::<u64>(), k in 1_u64..) {
        let expected = if n == 0 {
            Some(1)
        } else if n <= k {
            Some(n)
        } else {
            multifactorial(n - k, k).and_then(|value| value.checked_mul(n))
        };
        prop_assert_eq!(multifactorial(n, k), expected);
    }

    #[test]
    fn derangement_recurrence(n in 2_u64..=20) {
        let previous = subfactorial(n - 1).unwrap() + subfactorial(n - 2).unwrap();
        prop_assert_eq!(subfactorial(n), Some((n - 1) * previous));
    }

    #[test]
    fn ordered_partition_recurrence(n in 1_u64..=18) {
        let mut binomial = 1_u128;
        let mut sum = 0_u128;
        for k in 1..=n {
            binomial = binomial * u128::from(n - k + 1) / u128::from(k);
            sum += binomial * u128::from(fubini(n - k).unwrap());
        }
        prop_assert_eq!(fubini(n), u64::try_from(sum).ok());
    }
}

#[test]
fn native_boundaries() {
    assert_eq!(factorial(0), Some(1));
    assert_eq!(factorial(20), Some(2_432_902_008_176_640_000));
    assert_eq!(factorial(21), None);
    assert_eq!(factorial(u64::MAX), None);
    assert_eq!(subfactorial(0), Some(1));
    assert_eq!(subfactorial(1), Some(0));
    assert_eq!(subfactorial(2), Some(1));
    assert_eq!(subfactorial(3), Some(2));
    assert_eq!(subfactorial(20), Some(895_014_631_192_902_121));
    assert_eq!(subfactorial(21), None);
    assert_eq!(subfactorial(u64::MAX), None);
    assert_eq!(fubini(0), Some(1));
    assert_eq!(fubini(15), Some(230_283_190_977_853));
    assert_eq!(fubini(16), Some(5_315_654_681_981_355));
    assert_eq!(fubini(17), Some(130_370_767_029_135_901));
    assert_eq!(fubini(18), Some(3_385_534_663_256_845_323));
    assert_eq!(fubini(19), None);
    assert_eq!(fubini(u64::MAX), None);
    assert_eq!(multifactorial(0, 1), Some(1));
    assert_eq!(multifactorial(0, u64::MAX), Some(1));
    assert_eq!(multifactorial(u64::MAX, u64::MAX), Some(u64::MAX));
    assert_eq!(multifactorial(u64::MAX, u64::MAX - 1), Some(u64::MAX));
    assert_eq!(multifactorial(u64::MAX, 1), None);
    assert_eq!(multifactorial(33, 2), Some(6_332_659_870_762_850_625));
    assert_eq!(multifactorial(35, 2), None);
}

#[test]
#[should_panic(expected = "k must be positive")]
fn zero_step_rejected_for_empty_product() {
    let _ = multifactorial(0, 0);
}

#[test]
fn concurrent_repeated_calls() {
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..3 {
                    native_boundaries();
                }
            });
        }
    });
}
