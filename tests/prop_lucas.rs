//! Algebraic laws for exact and modular Lucas sequences.
use math_prime_util::*;
use proptest::prelude::*;

fn reduced(value: i128, n: u64) -> u64 {
    u64::try_from(value.rem_euclid(i128::from(n))).unwrap()
}

proptest! {
    #[test]
    fn recurrence(p in -5_i64..=5, q in -5_i64..=5, k in 0_u64..16) {
        let (u, v) = lucasuv(p, q, k);
        let (u1, v1) = lucasuv(p, q, k + 1);
        let (u2, v2) = lucasuv(p, q, k + 2);
        prop_assert_eq!(u2, Some(i128::from(p) * u1.unwrap() - i128::from(q) * u.unwrap()));
        prop_assert_eq!(v2, Some(i128::from(p) * v1.unwrap() - i128::from(q) * v.unwrap()));
    }

    #[test]
    fn exact_matches_modular(p in -5_i64..=5, q in -5_i64..=5, k in 0_u64..16, n in 1_u64..) {
        let (u, v) = lucasuv(p, q, k);
        prop_assert_eq!(lucasuvmod(p, q, k, n), Some((reduced(u.unwrap(), n), reduced(v.unwrap(), n))));
    }

    #[test]
    fn discriminant_identity(p in any::<i64>(), q in any::<i64>(), k in any::<u64>(), n in 1_u64..) {
        let (u, v, qk) = lucas_sequence(n, p, q, k);
        let modulus = u128::from(n);
        let square = |a: u64| u128::from(a) * u128::from(a) % modulus;
        let d = reduced(i128::from(p) * i128::from(p) - 4 * i128::from(q), n);
        let du2 = u128::from(d) * square(u) % modulus;
        prop_assert_eq!((square(v) + modulus - du2) % modulus, 4 * u128::from(qk) % modulus);
        prop_assert_eq!(lucasuvmod(p, q, k, n), Some((u, v)));
    }

    #[test]
    fn fibonacci_lucas_relation(k in 1_i64..90) {
        prop_assert_eq!(lucas_number(k), Some(fibonacci(k - 1).unwrap() + fibonacci(k + 1).unwrap()));
        prop_assert_eq!(fibonacci(-k), fibonacci(k).map(|f| if k % 2 == 0 { -f } else { f }));
        prop_assert_eq!(lucas_number(-k), lucas_number(k).map(|v| if k % 2 == 0 { v } else { -v }));
    }
}

#[test]
fn modulus_boundaries_and_repeat_calls() {
    assert_eq!(lucasumod(1, -1, 10, 0), None);
    assert_eq!(lucasvmod(1, -1, 10, 0), None);
    assert_eq!(lucasuvmod(1, -1, 10, 0), None);
    assert_eq!(lucas_sequence(1, i64::MIN, i64::MIN, u64::MAX), (0, 0, 0));
    for _ in 0..2 {
        assert_eq!(lucas_sequence(u64::MAX, 1, -1, 10), (55, 123, 1));
    }
}

#[test]
#[should_panic(expected = "modulus must be positive")]
fn sequence_rejects_zero_modulus() {
    lucas_sequence(0, 1, -1, 0);
}

#[test]
fn exact_boundaries_and_degenerate_sequences() {
    assert_eq!(fibonacci(93), Some(12_200_160_415_121_876_738));
    assert_eq!(fibonacci(94), None);
    assert_eq!(lucasu(2, 2, 256), Some(0));
    assert_eq!(lucasv(2, 2, 258), Some(0));
    assert_eq!(lucasu(3, 3, 300), Some(0));
    assert_eq!(lucasv(3, 3, 303), Some(0));
    assert_eq!(lucas_number(92), Some(16_860_207_025_497_407_047));
    assert_eq!(lucas_number(93), None);
    assert_eq!(fibonacci(i64::MIN), None);
    assert_eq!(lucasu(0, i64::MIN, u64::MAX - 1), Some(0));
    assert_eq!(lucasv(0, i64::MIN, u64::MAX), Some(0));
    assert_eq!(
        lucasuv(2, 1, u64::MAX),
        (Some(i128::from(u64::MAX)), Some(2))
    );
}
