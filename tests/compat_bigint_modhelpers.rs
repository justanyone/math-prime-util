//! Replay the MPU 0.75 pure Perl bigint helper oracle.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::modular::*;
use num_bigint::BigUint;

#[test]
fn bigint_modhelpers_oracle() {
    for c in common::load("bigint_modhelpers") {
        let a = |i| c.arg::<BigUint>(i);
        let list = |xs: Vec<BigUint>| {
            (!xs.is_empty()).then(|| {
                xs.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
        };
        let got = match c.func.as_str() {
            "submod" => submod(&a(0), &a(1), &a(2)).map(|v| v.to_string()),
            "negmod" => negmod(&a(0), &a(1)).map(|v| v.to_string()),
            "muladdmod" => muladdmod(&a(0), &a(1), &a(2), &a(3)).map(|v| v.to_string()),
            "mulsubmod" => mulsubmod(&a(0), &a(1), &a(2), &a(3)).map(|v| v.to_string()),
            "is_qr" => Some(u8::from(is_qr(&a(0), &a(1))).to_string()),
            "qnr" => qnr(&a(0)).map(|v| v.to_string()),
            "pisano_period" => Some(pisano_period(&a(0)).to_string()),
            "allrootmod" => list(allrootmod(&a(0), &a(1), &a(2))),
            "cornacchia" => cornacchia(&a(0), &a(1)).map(|(x, y)| format!("{x} {y}")),
            "chinese2" => chinese2(&[(a(0), a(1)), (a(2), a(3))]).map(|(x, y)| format!("{x} {y}")),
            "lucasuvmod" => {
                let (u, v) = lucasuvmod(&c.arg(0), &c.arg(1), &a(2), &a(3));
                Some(format!("{u} {v}"))
            }
            name => panic!("unhandled fixture function {name}"),
        };
        c.check(got);
    }
}
