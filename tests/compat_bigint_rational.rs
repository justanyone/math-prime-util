//! Replay the MPU 0.75 pure-Perl rational fixtures.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::rational::*;
use num_bigint::BigUint;

fn pair(value: &(BigUint, BigUint)) -> String {
    format!("{} {}", value.0, value.1)
}

#[test]
fn bigint_rational_oracle() {
    let cases = common::load("bigint_rational");
    assert!(!cases.is_empty());
    for c in cases {
        let a: Vec<BigUint> = c.args.iter().map(|s| s.parse().expect("integer")).collect();
        let got = match c.func.as_str() {
            "contfrac" => Some(
                contfrac(&a[0], &a[1])
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            "convergents" => convergents(&a).map(|v| {
                v.into_iter()
                    .map(|v| pair(&v))
                    .collect::<Vec<_>>()
                    .join(" ")
            }),
            "from_contfrac" => from_contfrac(&a).map(|v| pair(&v)),
            "next_calkin_wilf" => next_calkin_wilf(&a[0], &a[1]).map(|v| pair(&v)),
            "next_stern_brocot" => next_stern_brocot(&a[0], &a[1]).map(|v| pair(&v)),
            "calkin_wilf_n" => calkin_wilf_n(&a[0], &a[1]).map(|n| n.to_string()),
            "stern_brocot_n" => stern_brocot_n(&a[0], &a[1]).map(|n| n.to_string()),
            "nth_calkin_wilf" => Some(pair(&nth_calkin_wilf(&a[0]))),
            "nth_stern_brocot" => Some(pair(&nth_stern_brocot(&a[0]))),
            "nth_stern_diatomic" => Some(nth_stern_diatomic(&a[0]).to_string()),
            "next_farey" => next_farey(&a[0], &a[1], &a[2]).map(|v| pair(&v)),
            name => panic!("unknown fixture {name}"),
        };
        c.check(got);
    }
}
