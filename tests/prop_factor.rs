//! Independent factor counts and algebraic laws for factor predicates.
use math_prime_util::{is_almost_prime, is_omega_prime, is_semiprime, is_square_free};
use proptest::prelude::*;

fn counts(mut n: u64) -> (u64, u32, bool) {
    let (mut total, mut distinct, mut square_free) = (0, 0, n != 0);
    let mut p = 2;
    while n > 1 && p <= n / p {
        let mut e = 0;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        total += e;
        distinct += u32::from(e > 0);
        square_free &= e < 2;
        p += 1;
    }
    if n > 1 {
        total += 1;
        distinct += 1;
    }
    (total, distinct, square_free)
}

proptest! {
    #[test]
    fn predicates_match_independent_counts(n in 1u64..1_000_000, k in 0u32..25) {
        let (total, distinct, square_free) = counts(n);
        prop_assert_eq!(is_almost_prime(u64::from(k), n), total == u64::from(k));
        prop_assert_eq!(is_omega_prime(k, n), distinct == k);
        prop_assert_eq!(is_square_free(n), square_free);
        prop_assert_eq!(is_semiprime(n), total == 2);
        prop_assert!(is_almost_prime(total, n));
        prop_assert!(is_omega_prime(distinct, n));
    }

    #[test]
    fn multiplication_adds_multiplicity(a in 1u64..10000, b in 1u64..10000) {
        let (ka, _, _) = counts(a);
        let (kb, _, _) = counts(b);
        prop_assert!(is_almost_prime(ka + kb, a * b));
    }

    #[test]
    fn squares_are_not_square_free(n in 2u64..=u64::from(u32::MAX)) {
        prop_assert!(!is_square_free(n * n));
    }
}

#[test]
fn boundaries_and_large_counts() {
    for n in 0..=4096 {
        let (total, distinct, square_free) = counts(n);
        assert_eq!(is_square_free(n), square_free, "n={n}");
        assert_eq!(is_semiprime(n), n != 0 && total == 2, "n={n}");
        for k in 0..=15 {
            assert_eq!(
                is_almost_prime(u64::from(k), n),
                n != 0 && total == u64::from(k)
            );
            assert_eq!(is_omega_prime(k, n), n != 0 && distinct == k);
        }
    }
    assert!(is_almost_prime(63, 1 << 63));
    assert!(!is_almost_prime(64, u64::MAX));
    assert!(!is_almost_prime(u64::MAX, u64::MAX));
    assert!(!is_omega_prime(u32::MAX, u64::MAX));
    assert!(is_omega_prime(1, 13_u64.pow(16)));
    assert!(!is_omega_prime(1, 13 * 17));
}

#[test]
fn repeated_and_concurrent_calls() {
    let inputs = [0, 1, 4, 30, 49, 1_000_003 * 1_000_033, u64::MAX];
    let expected: Vec<_> = inputs.map(|n| (is_square_free(n), is_semiprime(n))).into();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for (n, expected) in inputs.into_iter().zip(&expected) {
                    assert_eq!((is_square_free(n), is_semiprime(n)), *expected);
                }
            });
        }
    });
}
