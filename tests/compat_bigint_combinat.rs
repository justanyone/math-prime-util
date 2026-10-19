//! Replay each MPU 0.75 pure-Perl combinatorial fixture row.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::combinat;
use num_bigint::BigUint;

#[test]
fn bigint_combinat_oracle() {
    for c in common::load("bigint_combinat") {
        match c.func.as_str() {
            "falling_factorial" => c.check(Some(combinat::falling_factorial(&c.arg(0), &c.arg(1)))),
            "rising_factorial" => c.check(Some(combinat::rising_factorial(&c.arg(0), &c.arg(1)))),
            "multifactorial" => c.check(Some(combinat::multifactorial(&c.arg(0), &c.arg(1)))),
            "numtoperm" => {
                let got = combinat::numtoperm(c.arg(0), &c.arg(1));
                // MPU returns undef for an empty list in the fixture helper.
                if got.is_empty() {
                    assert!(c.result.is_none());
                } else {
                    c.check(Some(
                        got.iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(" "),
                    ));
                }
            }
            "permtonum" => {
                let p: Vec<BigUint> = c.args.iter().map(|v| v.parse().unwrap()).collect();
                let got = combinat::permtonum(&p);
                if c.error {
                    assert_eq!(got, Err(math_prime_util::PermutationError::Invalid));
                } else {
                    c.check(Some(got.unwrap()));
                }
            }
            other => panic!("unhandled fixture function {other}"),
        }
    }
}
