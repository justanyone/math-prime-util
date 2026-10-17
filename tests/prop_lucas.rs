//! Algebraic laws and regressions for Lucas sequences.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn recurrence_and_discriminant(p in -20i64..=20, q in -20i64..=20, k in 0u64..1000, n in 1u64..100_000) {
        let (u, v, qk) = lucas_sequence(n, p, q, k);
        let (u1, v1) = lucasuvmod(p, q, k + 1, n);
        let (u2, v2) = lucasuvmod(p, q, k + 2, n);
        let modulus = i128::from(n);
        let reduce = |x: i128| u64::try_from(x.rem_euclid(modulus)).unwrap();
        prop_assert_eq!(u2, reduce(i128::from(p)*i128::from(u1) - i128::from(q)*i128::from(u)));
        prop_assert_eq!(v2, reduce(i128::from(p)*i128::from(v1) - i128::from(q)*i128::from(v)));
        let d = i128::from(p)*i128::from(p) - 4*i128::from(q);
        prop_assert_eq!(reduce(i128::from(v)*i128::from(v) - d*i128::from(u)*i128::from(u)), reduce(4*i128::from(qk)));
        prop_assert_eq!(lucasumod(p,q,k,n), u);
        prop_assert_eq!(lucasvmod(p,q,k,n), v);
    }

    #[test]
    fn fibonacci_and_lucas_identities(k in 1u64..90) {
        let f = fibonacci(k).unwrap();
        let next = fibonacci(k+1).unwrap();
        prop_assert_eq!(fibonacci(k+2), Some(f+next));
        prop_assert_eq!(lucas_number(k), Some(2*next-f));
        prop_assert_eq!(lucasu(1,-1,k), Some(i128::from(f)));
        prop_assert_eq!(lucasv(1,-1,k), lucas_number(k).map(i128::from));
    }
}

#[test]
fn exact_result_bounds_and_unused_powers() {
    assert_eq!(lucasu(2, -1, 51), Some(11_749_380_235_262_596_085));
    assert_eq!(lucasu(1, i64::MAX, 4), Some(-18_446_744_073_709_551_613));
    assert_eq!(lucasuv(2, 5, 55), Some((-7_772_665_691_574_266_981, -11_983_059_433_114_136_582)));
    assert_eq!(lucas_sequence(7778, 1976, 5, 33), (6153, 1454, 7403));
    assert_eq!(fibonacci(94), None);
    assert_eq!(lucas_number(93), None);
}

#[test]
fn repeated_and_concurrent_calls() {
    let expected = lucas_sequence(u64::MAX, i64::MIN, i64::MAX, u64::MAX);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..8 {
                    assert_eq!(lucas_sequence(u64::MAX, i64::MIN, i64::MAX, u64::MAX), expected);
                }
            });
        }
    });
}
