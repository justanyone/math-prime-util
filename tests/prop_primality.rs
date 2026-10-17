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
#[test]
fn repeated_concurrent_calls() {
    let threads: Vec<_> = (0..4).map(|_| std::thread::spawn(|| {
        for n in [0, 2, 97, 101, 121, u64::MAX-58] {
            let first = is_provable_prime_with_cert(n);
            assert_eq!(first, is_provable_prime_with_cert(n));
            assert_eq!(verify_prime(&first.1), Ok(first.0 == 2));
        }
    })).collect();
    for thread in threads { thread.join().unwrap(); }
}
