//! Algebraic laws for the combinatorics group, plus validation of bad input.

use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 200,
        failure_persistence: Some(Box::new(
            proptest::test_runner::FileFailurePersistence::WithSource("proptest-regressions"),
        )),
        ..ProptestConfig::default()
    })]

    #[test]
    fn binomial_symmetry(n in 0u64..60, k in 0u64..60) {
        if k <= n {
            prop_assert_eq!(
                binomial(i128::from(n), i128::from(k)),
                binomial(i128::from(n), i128::from(n - k))
            );
        }
    }

    #[test]
    fn binomial_pascal(n in 1u64..40, k in 1u64..40) {
        if k <= n {
            let a = binomial(i128::from(n), i128::from(k));
            let b = binomial(i128::from(n - 1), i128::from(k));
            let c = binomial(i128::from(n - 1), i128::from(k - 1));
            if let (Some(b), Some(c)) = (b, c) {
                prop_assert_eq!(a, b.checked_add(c));
            }
        }
    }

    #[test]
    fn factorial_recurrence(n in 1i64..20) {
        let prev = factorial(n - 1).unwrap();
        prop_assert_eq!(factorial(n), Some(prev * u128::from(n as u64)));
    }

    #[test]
    fn falling_matches_factorial(n in 0u64..15, m in 0u64..15) {
        if m <= n {
            let num = factorial(i64::try_from(n).unwrap()).unwrap();
            let den = factorial(i64::try_from(n - m).unwrap()).unwrap();
            prop_assert_eq!(falling_factorial(i128::from(n), m), Some(i128::try_from(num / den).unwrap()));
        }
    }

    #[test]
    fn rising_is_shifted_falling(n in 0i128..30, m in 0u64..12) {
        let rise = rising_factorial(n, m);
        let fall = n.checked_add(i128::from(m) - 1).and_then(|t| falling_factorial(t, m));
        prop_assert_eq!(rise, fall);
    }

    #[test]
    fn catalan_from_binomial(n in 0u64..12) {
        let c = catalan_number(n).unwrap();
        let b = binomial(i128::from(2 * n), i128::from(n)).unwrap();
        prop_assert_eq!(c * u128::from(n + 1), u128::try_from(b).unwrap());
    }

    #[test]
    fn stirling2_recurrence(n in 1u64..18, k in 1u64..18) {
        if k <= n {
            let s = stirling(n, k, 2);
            let a = stirling(n - 1, k, 2).unwrap_or(0);
            let b = stirling(n - 1, k - 1, 2).unwrap_or(0);
            let expect = a.checked_mul(i128::from(k)).and_then(|v| v.checked_add(b));
            prop_assert_eq!(s, expect);
        }
    }

    #[test]
    fn bell_is_sum_of_stirling2(n in 0u64..16) {
        let mut sum = 0i128;
        for k in 0..=n {
            sum += stirling(n, k, 2).unwrap();
        }
        prop_assert_eq!(bell_number(n), Some(u128::try_from(sum).unwrap()));
    }

    #[test]
    fn perm_roundtrip(n in 0u64..7, k in 0i128..600) {
        if n == 0 {
            prop_assert!(numtoperm(0, k).is_empty());
        } else {
            let fact: i128 = factorial(i64::try_from(n).unwrap()).unwrap() as i128;
            let rank = k.rem_euclid(fact);
            let p = numtoperm(n, rank);
            prop_assert_eq!(permtonum(&p), Some(u128::try_from(rank).unwrap()));
        }
    }

    #[test]
    fn frobenius_two_coprime(a in 2u64..80, b in 2u64..80) {
        if math_prime_util::gcd(a, b) == 1 {
            let expect = u128::from(a) * u128::from(b) - u128::from(a) - u128::from(b);
            prop_assert_eq!(frobenius_number(&[a, b]), Some(i128::try_from(expect).unwrap()));
        }
    }

    #[test]
    fn powersum_linear(n in 0u64..500) {
        let expect = u128::from(n) * u128::from(n + 1) / 2;
        prop_assert_eq!(powersum(u128::from(n), 1), Some(expect));
    }

    #[test]
    fn harm_adds_one_term(n in 1u64..25) {
        let (a, b) = harmfrac(n - 1).unwrap();
        let (c, d) = harmfrac(n).unwrap();
        // (a/b + 1/n) = c/d, so (a*n + b) * d = c * b * n.
        let b_i = i128::try_from(b).unwrap();
        let d_i = i128::try_from(d).unwrap();
        let lhs = (a * i128::from(n) + b_i) * d_i;
        let rhs = c * b_i * i128::from(n);
        prop_assert_eq!(lhs, rhs);
    }

    #[test]
    fn triangular_is_polygonal(n in 0u64..200) {
        let t = u128::from(n) * u128::from(n + 1) / 2;
        prop_assert!(is_polygonal(i128::try_from(t).unwrap(), 3));
    }

    #[test]
    fn primorial_grows(n in 2u64..30) {
        let a = primorial(n);
        let b = primorial(n - 1);
        if let (Some(a), Some(b)) = (a, b) {
            prop_assert!(a % b == 0 || b == 1);
            prop_assert!(a >= b);
        }
    }
}

#[test]
fn bad_input_panics() {
    let _ = catch();
}

fn catch() {
    assert!(std::panic::catch_unwind(|| factorial(-1)).is_err());
    assert!(std::panic::catch_unwind(|| multifactorial(4, 0)).is_err());
    assert!(std::panic::catch_unwind(|| multifactorial(-2, 1)).is_err());
    assert!(std::panic::catch_unwind(|| stirling(3, 1, 4)).is_err());
    assert!(std::panic::catch_unwind(|| is_polygonal(10, 2)).is_err());
    assert!(std::panic::catch_unwind(|| frobenius_number(&[20, 44])).is_err());
    assert!(std::panic::catch_unwind(|| frobenius_number(&[0, 5])).is_err());
    assert!(std::panic::catch_unwind(|| permtonum(&[1, 1])).is_err());
    assert!(std::panic::catch_unwind(|| partitions(-1)).is_err());
    assert!(std::panic::catch_unwind(|| is_congruent_number(-3)).is_err());
    assert!(std::panic::catch_unwind(|| is_happy(1, 1, 2)).is_err());
}
