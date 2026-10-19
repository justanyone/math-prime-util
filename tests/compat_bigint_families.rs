//! Replay the pure-Perl bigint family oracle.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::families::*;
use num_bigint::BigUint;

fn list(v: Vec<BigUint>) -> Option<String> {
    if v.is_empty() {
        None
    } else {
        Some(
            v.into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

#[test]
fn bigint_families_oracle() {
    for c in common::load("bigint_families") {
        let a: BigUint = c.arg(0);
        let b = || c.arg::<BigUint>(1);
        let got = match c.func.as_str() {
            "primes" => list(primes(&a, &b())),
            "twin_primes" => list(twin_primes(&a, &b())),
            "prime_powers" => list(prime_powers(&a, &b())),
            "semi_primes" => list(semi_primes(&a, &b()).unwrap()),
            "almost_primes" => list(almost_primes(c.arg(0), &b(), &c.arg(2)).unwrap()),
            "omega_primes" => list(omega_primes(c.arg(0), &b(), &c.arg(2)).unwrap()),
            "sieve_range" => list(sieve_range(&a, &b(), &c.arg(2))),
            "sieve_prime_cluster" => list(sieve_prime_cluster(&a, &b(), &[c.arg(2)])),
            "next_prime_power" => Some(next_prime_power(&a).to_string()),
            "prev_prime_power" => prev_prime_power(&a).map(|n| n.to_string()),
            "next_perfect_power" => Some(next_perfect_power(&a).to_string()),
            "prev_perfect_power" => prev_perfect_power(&a).map(|n| n.to_string()),
            "next_chen_prime" => Some(next_chen_prime(&a).unwrap().to_string()),
            "minimal_goldbach_pair" => minimal_goldbach_pair(&a).map(|p| p.to_string()),
            "print_primes" => {
                let mut out = Vec::new();
                print_primes(&a, &b(), &mut out).unwrap();
                let text = String::from_utf8(out).unwrap();
                let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
                if text.is_empty() { None } else { Some(text) }
            }
            _ => panic!("unknown row {}", c.func),
        };
        c.check(got);
    }
}
