//! Replay the pure Perl oracle for `inverse_sigma0`, `inverse_sigma0_count`,
//! `farey_rank` and `is_congruent_number` past 2^64, and the 64-bit edge of
//! the one-integer names that stay native.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::{
    self as native,
    bigint::{functions, predicates, rational},
};
use num_bigint::BigUint;

fn list(v: &[BigUint]) -> Option<String> {
    (!v.is_empty()).then(|| {
        v.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    })
}

#[test]
fn bigint_oneint_oracle() {
    for c in common::load("bigint_oneint") {
        let got = match c.func.as_str() {
            "inverse_sigma0" => {
                list(&functions::inverse_sigma0(&c.arg(0), &c.arg(1), &c.arg(2)).unwrap())
            }
            "inverse_sigma0_count" => Some(
                functions::inverse_sigma0_count(&c.arg(0), &c.arg(1), &c.arg(2))
                    .unwrap()
                    .to_string(),
            ),
            "farey_rank" => Some(rational::farey_rank(c.arg(0), &c.arg(1), &c.arg(2)).to_string()),
            "is_congruent_number" => {
                Some(u8::from(predicates::is_congruent_number(&c.arg(0)).unwrap()).to_string())
            }
            // MPU croaks past 2^63 - 1; the native forms return `None` or panic.
            "hclassno" => {
                assert!(c.error, "line {}: MPU must croak", c.line);
                assert_eq!(native::hclassno(c.arg(0)), None, "line {}", c.line);
                continue;
            }
            "integer_complexity" => {
                assert!(c.error, "line {}: MPU must croak", c.line);
                let n: u64 = c.arg(0);
                assert!(
                    std::panic::catch_unwind(|| native::integer_complexity(n)).is_err(),
                    "line {}",
                    c.line
                );
                continue;
            }
            "is_pillai" => Some(native::is_pillai(c.arg(0)).to_string()),
            "is_catalan_pseudoprime" => {
                Some(u8::from(native::is_catalan_pseudoprime(c.arg(0))).to_string())
            }
            other => panic!("unhandled fixture {other}"),
        };
        c.check(got);
    }
}
