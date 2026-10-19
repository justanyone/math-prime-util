//! Seeded MPU 0.75 pure-Perl oracle replay for the proven random primes.
//! The Perl certificate text is not compared; each Rust certificate is verified.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::{primality::is_prob_prime, proving::Certificate, random::*};
use num_bigint::BigUint;

fn with_cert(f: fn(u32) -> Option<(BigUint, Certificate)>, bits: u32) -> BigUint {
    let (p, cert) = f(bits).unwrap();
    assert_eq!(cert.n, p);
    assert_eq!(cert.verify(), Ok(true));
    p
}

#[test]
fn bigint_random_proven_oracle() {
    let rows = common::load("bigint_random_proven");
    assert!(rows.len() >= 30);
    for c in rows {
        math_prime_util::csrand(b"bigint-random-proven-seed-2026");
        let bits: u32 = c.arg(0);
        if c.error {
            let r = std::panic::catch_unwind(|| match c.func.as_str() {
                "random_maurer_prime" => random_maurer_prime(bits).is_some(),
                "random_maurer_prime_with_cert" => random_maurer_prime_with_cert(bits).is_some(),
                "random_shawe_taylor_prime" => random_shawe_taylor_prime(bits).is_some(),
                "random_shawe_taylor_prime_with_cert" => {
                    random_shawe_taylor_prime_with_cert(bits).is_some()
                }
                "random_proven_prime" => random_proven_prime(bits).is_some(),
                "random_proven_prime_with_cert" => random_proven_prime_with_cert(bits).is_some(),
                f => panic!("unreplayed function {f}"),
            });
            assert!(r.is_err(), "fixture line {}: must panic", c.line);
            continue;
        }
        let p = match c.func.as_str() {
            "random_maurer_prime" => random_maurer_prime(bits).unwrap(),
            "random_maurer_prime_with_cert" => with_cert(random_maurer_prime_with_cert, bits),
            "random_shawe_taylor_prime" => random_shawe_taylor_prime(bits).unwrap(),
            "random_shawe_taylor_prime_with_cert" => {
                with_cert(random_shawe_taylor_prime_with_cert, bits)
            }
            "random_proven_prime" => random_proven_prime(bits).unwrap(),
            "random_proven_prime_with_cert" => with_cert(random_proven_prime_with_cert, bits),
            f => panic!("unreplayed function {f}"),
        };
        assert_eq!(p.bits(), u64::from(bits));
        assert_ne!(is_prob_prime(&p), 0);
        c.check(Some(p));
    }
}
