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
            "bell_number" => c.check(combinat::bell_number(&c.arg(0))),
            "catalan_number" => c.check(combinat::catalan_number(&c.arg(0))),
            "subfactorial" => c.check(combinat::subfactorial(&c.arg(0))),
            "fubini" => c.check(combinat::fubini(&c.arg(0))),
            "partitions" => c.check(combinat::partitions(&c.arg(0))),
            "partitionsq" => c.check(combinat::partitionsq(&c.arg(0))),
            "stirling" => {
                let (n, m, kind): (u64, u64, u64) = (c.arg(0), c.arg(1), c.arg(2));
                let got = combinat::stirling(&n.into(), &m.into(), kind);
                // MPU gives the signed first kind; the BigUint form is the magnitude.
                let negative = kind == 1 && m < n && m > 0 && (n - m) % 2 == 1;
                let want = c.result.as_deref().map(|r| r.trim_start_matches('-'));
                assert_eq!(
                    got.map(|v| v.to_string()).as_deref(),
                    want,
                    "line {}",
                    c.line
                );
                assert_eq!(
                    c.result.as_deref().is_some_and(|r| r.starts_with('-')),
                    negative
                );
            }
            "bernfrac" => {
                let (num, den) = combinat::bernfrac(&c.arg(0)).expect("small index");
                c.check(Some(format!("{num} {den}")));
            }
            "harmfrac" => {
                let (num, den) = combinat::harmfrac(&c.arg(0)).expect("small index");
                c.check(Some(format!("{num} {den}")));
            }
            "ramanujan_tau" => c.check(combinat::ramanujan_tau(&c.arg(0))),
            other => panic!("unhandled fixture function {other}"),
        }
    }
}
