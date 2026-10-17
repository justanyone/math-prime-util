//! Algebraic checks for the native pseudoprime tests.
use math_prime_util::*;
use proptest::prelude::*;
proptest! {
    #[test]
    fn primality_matches_trial_division(n in 0_u64..100_000) {
        let prime = n >= 2 && (2..=sqrtint(n)).all(|d| n % d != 0);
        prop_assert_eq!(is_prob_prime(n), 2 * u8::from(prime));
        prop_assert_eq!(is_bpsw_prime(n), 2 * u8::from(prime));
    }
    #[test]
    fn strong_implies_fermat(n in 3_u64..1_000_000, a in 2_u64..1000) {
        if gcd(n,a) == 1 && is_strong_pseudoprime(n,a) {
            prop_assert!(is_pseudoprime(n,a));
        }
    }
    #[test]
    fn extra_strong_implies_almost(n in 0_u64..1_000_000) {
        if is_extra_strong_lucas_pseudoprime(n) {
            prop_assert!(is_almost_extra_strong_lucas_pseudoprime(n,1));
        }
    }
}
#[test]
fn repeat_and_concurrent_calls() {
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..10 {
                    assert_eq!(is_prob_prime(u64::MAX - 58), 2);
                    assert_eq!(is_prob_prime(u64::MAX), 0);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

proptest! {
    #[test]
    fn primes_pass_frobenius(n in 2_u64..1_000_000) {
        if is_prime(n) != 0 {
            prop_assert!(is_frobenius_pseudoprime(n));
            prop_assert!(is_frobenius_khashin_pseudoprime(n));
            prop_assert!(is_frobenius_underwood_pseudoprime(n));
        }
    }
    #[test]
    fn odd_squares_fail_frobenius(root in 3_u64..100_000) {
        let n = root * root;
        prop_assert!(!is_frobenius_pseudoprime(n));
        prop_assert!(!is_frobenius_khashin_pseudoprime(n));
        prop_assert!(!is_frobenius_underwood_pseudoprime(n));
    }
}

#[test]
fn frobenius_parameter_validation() {
    for (p, q) in [
        (3_037_000_498, 1),
        (-3_037_000_498, 1),
        (1, 3_037_000_498),
        (1, -3_037_000_498),
        (3, 2),
    ] {
        assert!(std::panic::catch_unwind(|| is_frobenius_pseudoprime_pq(101, p, q)).is_err());
    }
    assert!(is_frobenius_pseudoprime_pq(2, i64::MIN, i64::MAX));
    assert!(!is_frobenius_pseudoprime_pq(4, i64::MIN, i64::MAX));
    assert!(!is_frobenius_pseudoprime_pq(u64::MAX, 1, -1));
}

#[test]
fn frobenius_repeat_and_concurrent_calls() {
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..10 {
                    for test in [
                        is_frobenius_pseudoprime,
                        is_frobenius_khashin_pseudoprime,
                        is_frobenius_underwood_pseudoprime,
                    ] {
                        assert!(test(u64::MAX - 58));
                        assert!(!test(u64::MAX));
                    }
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
