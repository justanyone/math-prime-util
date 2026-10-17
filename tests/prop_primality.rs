//! Algebraic laws for native proving.
use math_prime_util::*;
use proptest::prelude::*;
proptest! {
    #[test]
    fn certificates_agree(n in any::<u64>()) {
        let (status, cert) = is_provable_prime_with_cert(n);
        prop_assert_eq!(status, is_prime(n));
        prop_assert_eq!(verify_prime(&cert), Ok(status == 2));
        prop_assert_eq!(prime_certificate(n), cert);
    }
    #[test]
    fn aks_agrees(n in 0u64..10_000) { prop_assert_eq!(is_aks_prime(n), is_prime(n) == 2); }
    #[test]
    fn squares_are_composite(n in 2u64..=u64::from(u32::MAX)) {
        prop_assert!(!is_aks_prime(n*n));
        prop_assert_eq!(is_provable_prime(n*n), 0);
    }
}
proptest! {
    #[test]
    fn products_have_no_certificate(a in 2_u64..=u64::from(u32::MAX), b in 2_u64..=u64::from(u32::MAX)) {
        let n = a * b;
        prop_assert_eq!(is_provable_prime(n), 0);
        prop_assert!(prime_certificate(n).is_empty());
        prop_assert_eq!(is_provable_prime_with_cert(n), (0, String::new()));
    }
}

#[test]
fn repeated_concurrent_calls() {
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for n in [0, 2, 97, 101, 121, u64::MAX - 58] {
                    let first = is_provable_prime_with_cert(n);
                    assert_eq!(first, is_provable_prime_with_cert(n));
                    assert_eq!(verify_prime(&first.1), Ok(first.0 == 2));
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
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

proptest! {
    #[test]
    fn primes_are_catalan_pseudoprimes(n in 2_u64..100_000) {
        if is_prime(n) != 0 {
            prop_assert!(is_catalan_pseudoprime(n));
        }
    }
    #[test]
    fn chen_and_delicate_are_prime(n in 0_u64..100_000) {
        if is_chen_prime(n) {
            prop_assert!(is_prime(n) != 0);
        }
        if is_delicate_prime(n, 10) == Some(true) {
            prop_assert!(is_prime(n) != 0);
        }
    }
}

#[test]
#[should_panic(expected = "restriction must be between 0 and 3")]
fn perrin_bad_restriction() {
    let _ = is_perrin_pseudoprime(271_441, 4);
}
