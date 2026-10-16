//! Algebraic laws and native boundary regressions.
use math_prime_util::{
    factorial, falling_factorial, multifactorial, rising_factorial, subfactorial,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn factorial_recurrence(n in 1_u64..=21) {
        prop_assert_eq!(factorial(n), factorial(n - 1).and_then(|v| v.checked_mul(n)));
        prop_assert_eq!(factorial(n), falling_factorial(n, n));
        prop_assert_eq!(factorial(n), rising_factorial(1, n));
    }

    #[test]
    fn reverse_product(n in 1_u64..=1000, count in 1_u64..=30) {
        prop_assert_eq!(rising_factorial(n, count), falling_factorial(n + count - 1, count));
    }

    #[test]
    fn multifactorial_recurrence(n in 1_u64..=100, step in 1_u64..=100) {
        prop_assert_eq!(multifactorial(n, step), multifactorial(n.saturating_sub(step), step).and_then(|v| v.checked_mul(n)));
    }

    #[test]
    fn derangement_recurrence(n in 2_u64..=21) {
        let expected = subfactorial(n - 1).and_then(|a| subfactorial(n - 2).and_then(|b| a.checked_add(b))).and_then(|v| v.checked_mul(n - 1));
        prop_assert_eq!(subfactorial(n), expected);
    }
}

#[test]
fn native_boundaries() {
    assert_eq!(factorial(20), Some(2_432_902_008_176_640_000));
    assert_eq!(factorial(21), None);
    assert_eq!(factorial(u64::MAX), None);
    assert_eq!(subfactorial(u64::MAX), None);
    assert_eq!(multifactorial(u64::MAX, u64::MAX), Some(u64::MAX));
    assert_eq!(rising_factorial(u64::MAX, 1), Some(u64::MAX));
    assert_eq!(rising_factorial(u64::MAX, 2), None);
    assert_eq!(rising_factorial(0, u64::MAX), Some(0));
    assert_eq!(falling_factorial(100, u64::MAX), Some(0));
    assert_eq!(falling_factorial(u64::MAX, 0), Some(1));
    assert_eq!(rising_factorial(u64::MAX, 0), Some(1));
}

#[test]
#[should_panic(expected = "step must be positive")]
fn zero_step_is_invalid_even_for_empty_product() {
    let _ = multifactorial(0, 0);
}

#[test]
fn repeated_and_concurrent_calls() {
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..100 {
                    assert_eq!(factorial(10), Some(3_628_800));
                    assert_eq!(subfactorial(10), Some(1_334_961));
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}
