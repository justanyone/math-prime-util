//! The arbitrary precision modular functions against MPU 0.75 (pure Perl)
//! on 65 to 500 bit inputs: `tests/fixtures/bigint_modular.tsv`.
#![cfg(feature = "bigint")]
mod common;
use common::Case;
use math_prime_util::bigint::arith::{binomial, powmod};
use math_prime_util::bigint::modular::*;
use math_prime_util::bigint::primality::is_prime;
use num_bigint::{BigInt, BigUint};

const FAMILY: &str = "bigint_modular";

fn unsigned(c: &Case, i: usize) -> BigUint {
    c.arg(i)
}

fn signed(c: &Case, i: usize) -> BigInt {
    c.arg(i)
}

fn flag(value: bool) -> u8 {
    u8::from(value)
}

fn joined(values: &[BigUint]) -> Option<String> {
    (!values.is_empty()).then(|| {
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    })
}

#[test]
fn sqrtmod_oracle() {
    common::each(FAMILY, "sqrtmod", |c| {
        c.check(sqrtmod(&unsigned(c, 0), &unsigned(c, 1)));
    });
    common::each(FAMILY, "allsqrtmod", |c| {
        c.check(joined(&allsqrtmod(&unsigned(c, 0), &unsigned(c, 1))));
    });
}

#[test]
fn rootmod_oracle() {
    let mut other_root = 0;
    common::each(FAMILY, "rootmod", |c| {
        let (a, k, n) = (unsigned(c, 0), unsigned(c, 1), unsigned(c, 2));
        let got = rootmod(&a, &k, &n);
        let oracle = c.result.as_ref().map(|r| r.parse::<BigUint>().unwrap());
        if let (Some(r), Some(other)) = (&got, &oracle) {
            if r != other {
                // Over a composite modulus pure Perl and the native port can
                // pick different roots: PP takes the smallest root modulo a
                // prime power (`allrootmod`) where the port lifts the root
                // modulo p by Newton's step, and the port's `p = 9 mod 16`
                // square root branch can give the other sign. Both are k-th
                // roots; `sqrtmod` is canonical and is checked exactly.
                assert!(is_prime(&n) == 0, "fixture line {}: prime modulus", c.line);
                assert_eq!(powmod(r, &k, &n), Some(&a % &n), "fixture line {}", c.line);
                other_root += 1;
                return;
            }
        }
        c.check(got);
    });
    assert!(other_root < 40, "{other_root} other roots");
}

#[test]
fn group_oracle() {
    common::each(FAMILY, "znorder", |c| {
        c.check(znorder(&unsigned(c, 0), &unsigned(c, 1)));
    });
    common::each(FAMILY, "znprimroot", |c| {
        c.check(znprimroot(&unsigned(c, 0)));
    });
    common::each(FAMILY, "is_primitive_root", |c| {
        c.check(is_primitive_root(&unsigned(c, 0), &unsigned(c, 1)).map(flag));
    });
    common::each(FAMILY, "znlog", |c| {
        c.check(znlog(&unsigned(c, 0), &unsigned(c, 1), &unsigned(c, 2)));
    });
}

#[test]
fn chinese_oracle() {
    common::each(FAMILY, "chinese", |c| {
        let pairs: Vec<(BigUint, BigUint)> = (0..c.args.len() / 2)
            .map(|i| (unsigned(c, 2 * i), unsigned(c, 2 * i + 1)))
            .collect();
        c.check(chinese(&pairs));
    });
}

#[test]
fn factorial_binomial_oracle() {
    common::each(FAMILY, "factorialmod", |c| {
        c.check(factorialmod(&unsigned(c, 0), &unsigned(c, 1)));
    });
    let mut perl_died = 0;
    common::each(FAMILY, "binomialmod", |c| {
        let (n, k, m) = (unsigned(c, 0), unsigned(c, 1), unsigned(c, 2));
        let got = binomialmod(&n, &k, &m);
        if c.error {
            // docs/upstream-findings.md entry 8 (confirmed): pure Perl
            // `binomial` with a Math::BigInt n and k >= 100 dies inside
            // Math::BigInt::bnok, so `binomialmod` dies too; the exact
            // binomial reduced mod m is the oracle here.
            let exact = binomial(&n, &k).unwrap() % &m;
            assert_eq!(got, Some(exact), "fixture line {}", c.line);
            perl_died += 1;
            return;
        }
        c.check(got);
    });
    assert!(perl_died >= 30, "{perl_died} rows where Perl died");
}

#[test]
fn lucas_oracle() {
    common::each_may_panic(FAMILY, "lucasumod", |c| {
        Some(lucasumod(
            &signed(c, 0),
            &signed(c, 1),
            &unsigned(c, 2),
            &unsigned(c, 3),
        ))
    });
    common::each_may_panic(FAMILY, "lucasvmod", |c| {
        Some(lucasvmod(
            &signed(c, 0),
            &signed(c, 1),
            &unsigned(c, 2),
            &unsigned(c, 3),
        ))
    });
}

#[test]
fn carmichael_oracle() {
    common::each(FAMILY, "is_carmichael", |c| {
        c.check(Some(flag(is_carmichael(&unsigned(c, 0)))));
    });
    common::each(FAMILY, "is_quasi_carmichael", |c| {
        c.check(Some(is_quasi_carmichael(&unsigned(c, 0))));
    });
}

#[test]
fn every_row_is_replayed() {
    let handled = [
        "sqrtmod",
        "rootmod",
        "allsqrtmod",
        "znorder",
        "znprimroot",
        "is_primitive_root",
        "znlog",
        "chinese",
        "factorialmod",
        "binomialmod",
        "lucasumod",
        "lucasvmod",
        "is_carmichael",
        "is_quasi_carmichael",
    ];
    let cases = common::load(FAMILY);
    for c in &cases {
        assert!(
            handled.contains(&c.func.as_str()),
            "fixture line {}: {}",
            c.line,
            c.func
        );
    }
    assert!(cases.len() > 1000, "{} rows", cases.len());
    // Every function has a defined result on an input of 65 bits or more,
    // and the root, order, factorial, binomial, Lucas and Carmichael rows
    // reach 500 bits; znlog reaches 128 bits.
    let widest = |func: &str, arg: usize| {
        cases
            .iter()
            .filter(|c| c.func == func && c.result.is_some())
            .map(|c| unsigned(c, arg).bits())
            .max()
            .unwrap_or(0)
    };
    for func in [
        "sqrtmod",
        "rootmod",
        "allsqrtmod",
        "factorialmod",
        "lucasumod",
        "lucasvmod",
        "is_carmichael",
        "is_quasi_carmichael",
    ] {
        let arg = match func {
            "factorialmod" => 1,
            "lucasumod" | "lucasvmod" => 3,
            _ => 0,
        };
        assert!(widest(func, arg) >= 500, "{func} stays below 500 bits");
    }
    assert!(widest("znorder", 1) >= 500);
    assert!(widest("znprimroot", 0) >= 500);
    assert!(widest("is_primitive_root", 1) >= 500);
    assert!(widest("binomialmod", 2) >= 500);
    assert!(widest("chinese", 5) >= 500);
    assert!(widest("znlog", 2) >= 128);
    let rows = |func: &str| cases.iter().filter(|c| c.func == func).count();
    assert!(rows("znlog") >= 50 && rows("rootmod") >= 100 && rows("binomialmod") >= 200);
    // Carmichael numbers above the 50-digit line where MPU stops factoring.
    let big_carmichael = cases
        .iter()
        .filter(|c| {
            c.func == "is_carmichael"
                && c.result.as_deref() == Some("1")
                && unsigned(c, 0).to_string().len() > 50
        })
        .count();
    assert!(
        big_carmichael >= 3,
        "{big_carmichael} large Carmichael numbers"
    );
}
