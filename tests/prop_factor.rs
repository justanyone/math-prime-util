//! Algebraic laws for factor counts, signatures, and partial factoring.
use math_prime_util::{cheb_factor, prime_bigomega, prime_omega, prime_signature};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn signature_counts(n in 1_u64..1_000_000) {
        let signature = prime_signature(n);
        prop_assert_eq!(signature.iter().sum::<u32>(), prime_bigomega(n));
        prop_assert_eq!(signature.len(), usize::try_from(prime_omega(n)).unwrap());
        prop_assert!(signature.windows(2).all(|pair| pair[0] >= pair[1]));
    }

    #[test]
    fn multiplicity_is_additive(a in 1_u64..100_000, b in 1_u64..100_000) {
        prop_assert_eq!(prime_bigomega(a * b), prime_bigomega(a) + prime_bigomega(b));
        prop_assert!(prime_omega(a * b) <= prime_omega(a) + prime_omega(b));
        if math_prime_util::gcd(a, b) == 1 {
            prop_assert_eq!(prime_omega(a * b), prime_omega(a) + prime_omega(b));
        }
    }

    #[test]
    fn chebyshev_preserves_product(n in 1_u64..10_000_000, bound in 0_u64..200, x in any::<u64>()) {
        let factors = cheb_factor(n, bound, x);
        prop_assert_eq!(factors.iter().product::<u64>(), n);
        prop_assert!(factors.windows(2).all(|pair| pair[0] <= pair[1]));
        prop_assert!(factors.iter().all(|&f| f > 1));
    }
}

#[test]
fn edges_and_repeated_calls() {
    for _ in 0..2 {
        assert_eq!(prime_signature(0), vec![1]);
        assert_eq!(prime_signature(1), Vec::<u32>::new());
        assert_eq!(prime_omega(0), 1);
        assert_eq!(prime_bigomega(1), 0);
        assert_eq!(prime_signature(1_u64 << 63), vec![63]);
        assert_eq!(cheb_factor(0, 0, 0), vec![0]);
        assert_eq!(cheb_factor(1, 0, 0), Vec::<u64>::new());
        assert_eq!(cheb_factor(143, 1, u64::MAX), vec![143]);
        assert_eq!(cheb_factor(143, u64::MAX, 1), vec![143]);
        assert_eq!(cheb_factor(143, 0, 143), vec![143]);
    }
}

#[test]
fn concurrent_calls_are_independent() {
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                assert_eq!(prime_signature(360), vec![3, 2, 1]);
                assert_eq!(prime_bigomega(360), 6);
                assert_eq!(prime_omega(360), 3);
                assert_eq!(cheb_factor(403, 0, 0), vec![13, 31]);
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}
