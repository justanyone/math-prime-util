//! Goldbach and Chen compatibility with the committed MPU 0.75 oracle.
mod common;

use common::each;
use math_prime_util::{
    goldbach_pair_count, goldbach_pairs, minimal_goldbach_pair, next_chen_prime,
};

#[test]
fn minimal_goldbach_pair_oracle() {
    each("families", "minimal_goldbach_pair", |c| {
        // Skip minimal_goldbach_pair(83778272185315920949659591651127238812):
        // this bigint argument exceeds the native u64 API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(minimal_goldbach_pair(n));
        }
    });
}

#[test]
fn goldbach_pair_count_oracle() {
    each("families", "goldbach_pair_count", |c| {
        c.check(Some(goldbach_pair_count(c.arg(0))));
    });
}

#[test]
fn goldbach_pairs_oracle() {
    each("families", "goldbach_pairs", |c| {
        let pairs = goldbach_pairs(c.arg(0));
        let text = pairs
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        c.check((!pairs.is_empty()).then_some(text));
    });
}

#[test]
fn next_chen_prime_oracle() {
    each("families", "next_chen_prime", |c| {
        c.check(next_chen_prime(c.arg(0)));
    });
}
