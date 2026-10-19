//! Seeded MPU 0.75 PP oracle replay. Factored and unrestricted sampling
//! use property checks where PP and native-sized modulus draws differ.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::{primality::is_prob_prime, random::*};
use num_bigint::BigUint;

#[test]
fn bigint_random_oracle() {
    let rows = common::load("bigint_random");
    assert!(rows.len() >= 20);
    for c in rows {
        math_prime_util::csrand(b"bigint-random-fixed-seed-2026");
        let n = || c.arg::<BigUint>(0);
        match c.func.as_str() {
            "urandomb" => c.check(Some(urandomb(c.arg(0)))),
            "urandomm" => c.check(Some(urandomm(&n()))),
            "urandomr" => c.check(urandomr(&n(), &c.arg(1))),
            "miller_rabin_random" => c.check(miller_rabin_random(&n(), c.arg(1))),
            "random_nbit_prime" => c.check(random_nbit_prime(c.arg(0))),
            "random_safe_prime" => {
                let p = random_safe_prime(c.arg(0)).unwrap();
                assert_eq!(is_prob_prime(&p), 1);
                assert_ne!(is_prob_prime(&((&p - 1_u8) >> 1_usize)), 0);
                if c.error {
                    // MPU075_PP_SAFE_INTERNAL: PP raises its internal failure check.
                    // docs/upstream-findings.md records this seeded oracle defect.
                    assert_eq!(p.bits(), c.arg::<u64>(0));
                } else {
                    c.check(Some(p));
                }
            }
            "random_prime" => c.check(random_prime(&n(), &c.arg(1))),
            "random_ndigit_prime" => {
                // Replay the PP partition choice and test its prime form.
                let p = random_ndigit_prime(c.arg(0)).unwrap();
                assert_eq!(p.to_string().len(), c.arg::<usize>(0));
                assert_ne!(is_prob_prime(&p), 0);
                let oracle: BigUint = c.result.as_ref().unwrap().parse().unwrap();
                assert_ne!(is_prob_prime(&oracle), 0);
                c.check(Some(p));
            }
            "random_strong_prime" | "random_semiprime" | "random_unrestricted_semiprime" => {
                // Strong and balanced primes also match the seeded PP value.
                // Unrestricted sampling uses different native-sized modulus draws.
                let bits = c.arg(0);
                let p = match c.func.as_str() {
                    "random_strong_prime" => random_strong_prime(bits),
                    "random_semiprime" => random_semiprime(bits),
                    _ => random_unrestricted_semiprime(bits),
                }
                .unwrap();
                assert_eq!(p.bits(), u64::from(bits));
                let oracle: BigUint = c.result.as_ref().unwrap().parse().unwrap();
                assert_eq!(oracle.bits(), u64::from(bits));
                if c.func == "random_strong_prime" {
                    assert_ne!(is_prob_prime(&p), 0);
                }
                if c.func != "random_unrestricted_semiprime" {
                    c.check(Some(p));
                }
            }
            "random_factored_integer" => {
                // Native-sized chain terms use the native seeded sampler.
                let (r, factors) = random_factored_integer(&n());
                assert!(r >= 1_u8.into() && r <= n());
                assert_eq!(factors.iter().product::<BigUint>(), r);
                assert!(factors.iter().all(|p| is_prob_prime(p) != 0));
                let values: Vec<BigUint> = c
                    .result
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                assert_eq!(values[1..].iter().product::<BigUint>(), values[0]);
            }
            f => panic!("unreplayed function {f}"),
        }
    }
}
