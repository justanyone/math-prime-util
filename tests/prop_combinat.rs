//! Algebraic laws and native boundary regressions for combinatorics.
use math_prime_util::{bell_number, binomial, catalan_number, stirling};
use proptest::prelude::*;

proptest! {
    #[test]
    fn binomial_symmetry_and_pascal(n in 1_u64..60, seed in any::<u64>()) {
        let k = seed % n;
        prop_assert_eq!(binomial(n, k), binomial(n, n - k));
        prop_assert_eq!(binomial(n + 1, k + 1),
            Some(binomial(n, k).unwrap() + binomial(n, k + 1).unwrap()));
    }

    #[test]
    fn catalan_recurrence(n in 0_u64..35) {
        let next = u128::from(catalan_number(n).unwrap()) * 2 * u128::from(2 * n + 1)
            / u128::from(n + 2);
        prop_assert_eq!(u128::from(catalan_number(n + 1).unwrap()), next);
    }

    #[test]
    fn stirling_recurrences(n in 1_u64..14, seed in any::<u64>()) {
        let m = seed % n + 1;
        let a = stirling(n, m - 1, 1).unwrap();
        let b = stirling(n, m, 1).unwrap();
        prop_assert_eq!(stirling(n + 1, m, 1), Some(a - i128::from(n) * b));
        let a = stirling(n, m - 1, 2).unwrap();
        let b = stirling(n, m, 2).unwrap();
        prop_assert_eq!(stirling(n + 1, m, 2), Some(a + i128::from(m) * b));
        let a = stirling(n, m - 1, 3).unwrap();
        let b = stirling(n, m, 3).unwrap();
        prop_assert_eq!(stirling(n + 1, m, 3), Some(a + i128::from(n + m) * b));
    }

    #[test]
    fn bell_binomial_transform(n in 0_u64..24) {
        let sum: u64 = (0..=n).map(|k| binomial(n, k).unwrap() * bell_number(k).unwrap()).sum();
        prop_assert_eq!(bell_number(n + 1), Some(sum));
    }
}

#[test]
fn native_boundaries() {
    assert_eq!(binomial(u64::MAX, 0), Some(1));
    assert_eq!(binomial(u64::MAX, 1), Some(u64::MAX));
    assert_eq!(binomial(u64::MAX, u64::MAX - 1), Some(u64::MAX));
    assert_eq!(binomial(u64::MAX, 2), None);
    for (n, k) in [(73, 25), (92, 19), (131, 15)] {
        assert_eq!(binomial(n, k), None);
    }
    assert_eq!(catalan_number(36), Some(11_959_798_385_860_453_492));
    assert_eq!(catalan_number(37), None);
    assert_eq!(catalan_number(u64::MAX), None);
    assert_eq!(bell_number(25), Some(4_638_590_332_229_999_353));
    assert_eq!(bell_number(26), None);
    assert_eq!(bell_number(u64::MAX), None);
    for kind in 1..=3 {
        assert_eq!(stirling(u64::MAX, u64::MAX, kind), Some(1));
        assert_eq!(stirling(u64::MAX, u64::MAX - 1, kind), None);
        assert_eq!(stirling(0, 1, kind), Some(0));
        assert_eq!(
            stirling(40, 1, kind),
            if kind == 2 { Some(1) } else { None }
        );
        assert_eq!(stirling(100, 50, kind), None);
    }
}

#[test]
fn invalid_stirling_kind() {
    for kind in [0, 4, u64::MAX] {
        assert!(std::panic::catch_unwind(|| stirling(0, 0, kind)).is_err());
    }
}

#[test]
fn stirling_cancellation_regressions() {
    assert_eq!(stirling(18, 2, 1), Some(1_223_405_590_579_200));
    assert_eq!(stirling(64, 62, 2), Some(1_947_792));
    assert_eq!(stirling(65, 2, 2), Some(i128::from(u64::MAX)));
    assert_eq!(stirling(66, 2, 2), None);
    assert_eq!(stirling(100, 98, 1), Some(12_087_075));
    assert_eq!(stirling(100, 98, 2), Some(11_925_375));
    assert_eq!(stirling(1000, 2, 1), None);
    assert_eq!(stirling(1000, 2, 2), None);
}

#[test]
fn independent_calls_are_repeatable() {
    let calls: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (
                    binomial(67, 31),
                    catalan_number(36),
                    stirling(22, 12, 1),
                    bell_number(25),
                )
            })
        })
        .collect();
    let expected = (
        binomial(67, 31),
        catalan_number(36),
        stirling(22, 12, 1),
        bell_number(25),
    );
    for call in calls {
        assert_eq!(call.join().unwrap(), expected);
    }
}
