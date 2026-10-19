//! The arbitrary precision integer core against MPU 0.75 (pure Perl) on
//! 65 to 1000 bit inputs: `tests/fixtures/bigint_arith.tsv`.
#![cfg(feature = "bigint")]
mod common;
use common::Case;
use math_prime_util::bigint::arith::*;
use num_bigint::{BigInt, BigUint};

const FAMILY: &str = "bigint_arith";

fn unsigned(c: &Case, i: usize) -> BigUint {
    c.arg(i)
}

fn signed(c: &Case, i: usize) -> BigInt {
    c.arg(i)
}

fn flag(value: bool) -> u8 {
    u8::from(value)
}

#[test]
fn gcd_lcm_oracle() {
    common::each(FAMILY, "gcd", |c| {
        c.check(Some(gcd(&unsigned(c, 0), &unsigned(c, 1))));
    });
    common::each(FAMILY, "lcm", |c| {
        c.check(Some(lcm(&unsigned(c, 0), &unsigned(c, 1))));
    });
}

#[test]
fn gcdext_oracle() {
    common::each(FAMILY, "gcdext", |c| {
        let (g, u, v) = gcdext(&signed(c, 0), &signed(c, 1));
        c.check(Some(format!("{g} {u} {v}")));
    });
}

#[test]
fn modular_oracle() {
    common::each(FAMILY, "invmod", |c| {
        c.check(invmod(&unsigned(c, 0), &unsigned(c, 1)));
    });
    common::each(FAMILY, "divmod", |c| {
        c.check(divmod(&unsigned(c, 0), &unsigned(c, 1), &unsigned(c, 2)));
    });
    common::each(FAMILY, "addmod", |c| {
        c.check(addmod(&unsigned(c, 0), &unsigned(c, 1), &unsigned(c, 2)));
    });
    common::each(FAMILY, "mulmod", |c| {
        c.check(mulmod(&unsigned(c, 0), &unsigned(c, 1), &unsigned(c, 2)));
    });
    common::each(FAMILY, "powmod", |c| {
        c.check(powmod(&unsigned(c, 0), &unsigned(c, 1), &unsigned(c, 2)));
    });
}

#[test]
fn roots_and_logs_oracle() {
    common::each(FAMILY, "sqrtint", |c| {
        c.check(Some(sqrtint(&unsigned(c, 0))));
        c.check(Some(isqrt(&unsigned(c, 0))));
    });
    common::each(FAMILY, "is_square", |c| {
        c.check(Some(flag(is_square(&unsigned(c, 0)))));
    });
    common::each_may_panic(FAMILY, "rootint", |c| {
        Some(rootint(&unsigned(c, 0), c.arg(1)))
    });
    common::each_may_panic(FAMILY, "logint", |c| {
        Some(logint(&unsigned(c, 0), &unsigned(c, 1)))
    });
}

#[test]
fn powers_oracle() {
    common::each(FAMILY, "is_power", |c| {
        c.check(Some(flag(is_power(&unsigned(c, 0), c.arg(1)))));
    });
    common::each(FAMILY, "is_power_exp", |c| {
        c.check(Some(is_power_exp(&unsigned(c, 0))));
        let any_power = c.result.as_deref() != Some("0");
        assert_eq!(
            is_power(&unsigned(c, 0), 0),
            any_power,
            "fixture line {}",
            c.line
        );
    });
}

#[test]
fn symbols_oracle() {
    common::each(FAMILY, "jacobi", |c| {
        c.check(Some(jacobi(&signed(c, 0), &unsigned(c, 1))));
    });
    common::each(FAMILY, "kronecker", |c| {
        c.check(Some(kronecker(&signed(c, 0), &signed(c, 1))));
    });
}

#[test]
fn combinatorics_oracle() {
    let mut perl_defects = 0;
    common::each(FAMILY, "binomial", |c| {
        let (n, k) = (unsigned(c, 0), unsigned(c, 1));
        if c.error {
            // docs/upstream-findings.md: pure Perl `binomial` dies for a bigint n
            // with k >= 100 instead of reflecting k to n - k. The correct value is
            // C(n, n - k), checked here against the falling factorial.
            let small = &n - &k;
            let mut expected = BigUint::from(1_u8);
            let mut i = BigUint::from(0_u8);
            while i < small {
                expected *= &n - &i;
                i += 1_u8;
            }
            let expected = expected / factorial(u64::try_from(&small).unwrap());
            assert_eq!(binomial(&n, &k), Some(expected), "fixture line {}", c.line);
            assert_eq!(binomial(&n, &k), binomial(&n, &small));
            perl_defects += 1;
            return;
        }
        c.check(binomial(&n, &k));
    });
    assert_eq!(perl_defects, 1);
    common::each(FAMILY, "factorial", |c| {
        c.check(Some(factorial(c.arg(0))));
    });
    common::each_may_panic(FAMILY, "valuation", |c| {
        valuation(&unsigned(c, 0), &unsigned(c, 1))
    });
}

#[test]
fn every_row_is_replayed() {
    let handled = [
        "gcd",
        "lcm",
        "gcdext",
        "invmod",
        "divmod",
        "addmod",
        "mulmod",
        "powmod",
        "sqrtint",
        "is_square",
        "rootint",
        "logint",
        "is_power",
        "is_power_exp",
        "jacobi",
        "kronecker",
        "binomial",
        "factorial",
        "valuation",
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
    assert!(cases.len() > 700, "{} rows", cases.len());
    // Inputs cover the 65 to 1000 bit range.
    let widest = cases
        .iter()
        .flat_map(|c| c.args.iter())
        .filter_map(|a| a.parse::<BigUint>().ok())
        .map(|a| a.bits())
        .max();
    assert!(widest.is_some_and(|bits| bits >= 1000), "{widest:?}");
}
