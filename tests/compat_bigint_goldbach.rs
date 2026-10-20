//! Replay the pure-Perl Goldbach oracle on `BigUint` input.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::families::{goldbach_pair_count, goldbach_pairs};
use num_bigint::BigUint;

#[test]
fn bigint_goldbach_oracle() {
    let rows = common::load("bigint_goldbach");
    assert_eq!(rows.len(), 78, "the oracle must cover every input");
    for c in rows {
        let n: BigUint = c.arg(0);
        let got = match c.func.as_str() {
            "goldbach_pair_count" => Some(goldbach_pair_count(&n).to_string()),
            "goldbach_pairs" => {
                let pairs = goldbach_pairs(&n);
                (!pairs.is_empty()).then(|| {
                    pairs
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                })
            }
            _ => panic!("unknown row {}", c.func),
        };
        c.check(got);
    }
}
