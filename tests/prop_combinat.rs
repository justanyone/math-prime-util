//! Algebraic laws and boundary checks for the assigned combinat functions.
use math_prime_util::{falling_factorial, integer_complexity, powersum, rising_factorial};
use proptest::prelude::*;

proptest! {
    #[test]
    fn factorial_products(n in 0_u64..1000, m in 0_u64..30) {
        let falling = if m > n { Some(0) } else {
            (0..m).try_fold(1_u64, |r, i| r.checked_mul(n - i))
        };
        let rising = (0..m).try_fold(1_u64, |r, i| r.checked_mul(n + i));
        prop_assert_eq!(falling_factorial(n, m), falling);
        prop_assert_eq!(rising_factorial(n, m), rising);
    }

    #[test]
    fn power_sum_direct(n in 0_u64..200, k in 0_u32..20) {
        let expected = (1..=n).try_fold(0_u64, |sum, i| sum.checked_add(i.checked_pow(k)?));
        prop_assert_eq!(powersum(n, u64::from(k)), expected);
    }

    #[test]
    fn complexity_composition(a in 1_u64..100, b in 1_u64..100) {
        let bound = integer_complexity(a) + integer_complexity(b);
        prop_assert!(integer_complexity(a + b) <= bound);
        prop_assert!(integer_complexity(a * b) <= bound);
    }
}

#[test]
fn native_boundaries() {
    assert_eq!(falling_factorial(u64::MAX, 0), Some(1));
    assert_eq!(falling_factorial(u64::MAX, 1), Some(u64::MAX));
    assert_eq!(falling_factorial(u64::MAX, 2), None);
    assert_eq!(rising_factorial(u64::MAX, 2), None);
    assert_eq!(rising_factorial(0, u64::MAX), Some(0));
    assert_eq!(powersum(u64::MAX, 0), Some(u64::MAX));
    assert_eq!(powersum(0, u64::MAX), Some(0));
    assert_eq!(powersum(1, u64::MAX), Some(1));
    assert_eq!(powersum(2, 64), None);
    for (k, n) in [
        (1, 6_074_000_999),
        (2, 3_810_777),
        (3, 92_681),
        (4, 9839),
        (5, 2190),
        (6, 745),
        (7, 331),
        (8, 175),
    ] {
        assert!(powersum(n, k).is_some());
        assert_eq!(powersum(n + 1, k), None);
    }
}

#[test]
fn complexity_matches_exhaustive_recurrence() {
    let mut exact = vec![0_u64; 301];
    exact[1] = 1;
    for n in 2..exact.len() {
        exact[n] = (1..=n / 2).map(|a| exact[a] + exact[n - a]).min().unwrap();
        for a in 2..=n / 2 {
            if n % a == 0 {
                exact[n] = exact[n].min(exact[a] + exact[n / a]);
            }
        }
        assert_eq!(integer_complexity(u64::try_from(n).unwrap()), exact[n]);
    }
}
