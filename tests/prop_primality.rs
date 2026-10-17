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
