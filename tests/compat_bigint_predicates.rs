//! Replay every bigint predicate oracle row.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::predicates::*;
use num_bigint::BigUint;

#[test]
fn bigint_predicates_oracle() {
    for c in common::load("bigint_predicates") {
        let n: BigUint = c.arg(0);
        let got = match c.func.as_str() {
            "is_semiprime" => u32::from(is_semiprime(&n).unwrap()),
            "is_cyclic" => u32::from(is_cyclic(&n).unwrap()),
            "is_fundamental" => u32::from(is_fundamental(&n).unwrap()),
            "is_perfect_number" => u32::from(is_perfect_number(&n).unwrap()),
            "is_perfect_power" => u32::from(is_perfect_power(&n)),
            "is_prime_power" => is_prime_power(&n),
            "is_practical" => u32::from(is_practical(&n).unwrap()),
            "is_totient" => u32::from(is_totient(&n).unwrap()),
            "is_powerfree" => u32::from(is_powerfree(&n, &c.arg(1)).unwrap()),
            "is_powerful" => u32::from(is_powerful(&n, &c.arg(1)).unwrap()),
            "is_rough" => u32::from(is_rough(&n, &c.arg(1)).unwrap()),
            "is_smooth" => u32::from(is_smooth(&n, &c.arg(1)).unwrap()),
            "is_sum_of_squares" => u32::from(is_sum_of_squares(&n, &c.arg(1)).unwrap()),
            "is_almost_prime" => u32::from(is_almost_prime(&n, &c.arg(1)).unwrap()),
            "is_omega_prime" => u32::from(is_omega_prime(&n, &c.arg(1)).unwrap()),
            name => panic!("unknown predicate {name}"),
        };
        c.check(Some(got));
    }
}
