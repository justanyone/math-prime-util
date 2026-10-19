//! The arbitrary precision primality proofs agree with the native path on the
//! `u64` range: the same status, the same certificate text, and the same
//! verdict of `verify_prime` on it; AKS agrees on small inputs.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::proving;
use num_bigint::BigUint;
use proptest::prelude::*;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

proptest! {
    #[test]
    fn proofs_agree(n: u64, small in 0_u64..100_000) {
        for n in [n, small, n | 1] {
            let bn = big(n);
            prop_assert_eq!(proving::is_provable_prime(&bn), math_prime_util::is_provable_prime(n));
            let (status, cert) = proving::is_provable_prime_with_cert(&bn);
            prop_assert_eq!((status, cert.clone()), math_prime_util::is_provable_prime_with_cert(n));
            prop_assert_eq!(proving::prime_certificate(&bn), math_prime_util::prime_certificate(n));
            prop_assert_eq!(proving::verify_prime(&cert), math_prime_util::verify_prime(&cert));
            if status == 2 {
                prop_assert_eq!(proving::verify_prime(&cert), Ok(true));
                let parsed = proving::Certificate::parse(&cert).map_err(TestCaseError::fail)?;
                prop_assert_eq!(parsed.n.clone(), bn.clone());
                prop_assert_eq!(parsed.to_string(), cert);
                prop_assert_eq!(proving::Certificate::prove(&bn), Some(parsed));
            } else {
                prop_assert_eq!(proving::Certificate::prove(&bn), None);
            }
        }
    }

    #[test]
    fn aks_agrees(n in 0_u64..1_000_000, wide in 0_u64..(1_u64 << 40)) {
        for n in [n, wide] {
            prop_assert_eq!(proving::is_aks_prime(&big(n)), math_prime_util::is_aks_prime(n));
        }
    }

    #[test]
    fn native_certificates_verify_here(n in 2_u64..=u64::MAX) {
        let cert = math_prime_util::prime_certificate(n);
        prop_assert_eq!(proving::verify_prime(&cert), math_prime_util::verify_prime(&cert));
        // A tampered target is rejected by both.
        let bad = cert.replace(&format!("Proof for:\nN {n}"), "Proof for:\nN 4");
        prop_assert_eq!(proving::verify_prime(&bad), math_prime_util::verify_prime(&bad));
    }
}
