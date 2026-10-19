//! The arbitrary precision primality tests against MPU 0.75 (pure Perl) on
//! 65 to 1000 bit inputs: `tests/fixtures/bigint_primality.tsv`.
#![cfg(feature = "bigint")]
mod common;
use common::Case;
use math_prime_util::bigint::primality::*;
use num_bigint::{BigInt, BigUint};

const FAMILY: &str = "bigint_primality";

fn unsigned(c: &Case, i: usize) -> BigUint {
    c.arg(i)
}

fn bases(c: &Case) -> Vec<BigUint> {
    (1..c.args.len()).map(|i| unsigned(c, i)).collect()
}

fn flag(value: bool) -> u8 {
    u8::from(value)
}

#[test]
fn prime_oracle() {
    common::each(FAMILY, "is_prime", |c| {
        c.check(Some(is_prime(&unsigned(c, 0))));
    });
    common::each(FAMILY, "is_prob_prime", |c| {
        c.check(Some(is_prob_prime(&unsigned(c, 0))));
    });
    common::each(FAMILY, "is_bpsw_prime", |c| {
        c.check(Some(is_bpsw_prime(&unsigned(c, 0))));
    });
}

#[test]
fn base_tests_oracle() {
    common::each_may_panic(FAMILY, "is_strong_pseudoprime", |c| {
        Some(flag(is_strong_pseudoprime(&unsigned(c, 0), &bases(c))))
    });
    common::each_may_panic(FAMILY, "is_pseudoprime", |c| {
        Some(flag(is_pseudoprime(&unsigned(c, 0), &bases(c))))
    });
    common::each_may_panic(FAMILY, "is_euler_pseudoprime", |c| {
        Some(flag(is_euler_pseudoprime(&unsigned(c, 0), &bases(c))))
    });
    let mut perl_empty = 0;
    common::each(FAMILY, "is_euler_plumb_pseudoprime", |c| {
        let got = flag(is_euler_plumb_pseudoprime(&unsigned(c, 0)));
        if c.result.as_deref() == Some("") {
            // docs/upstream-findings.md entry 14: pure Perl returns the empty
            // string (Perl false) instead of 0 when the power is 1 or n - 1
            // but n mod 8 does not match; XS returns 0.
            assert_eq!(got, 0, "fixture line {}", c.line);
            perl_empty += 1;
            return;
        }
        c.check(Some(got));
    });
    assert_eq!(perl_empty, 5);
}

#[test]
fn lucas_tests_oracle() {
    common::each(FAMILY, "is_lucas_pseudoprime", |c| {
        c.check(Some(flag(is_lucas_pseudoprime(&unsigned(c, 0)))));
    });
    common::each(FAMILY, "is_strong_lucas_pseudoprime", |c| {
        c.check(Some(flag(is_strong_lucas_pseudoprime(&unsigned(c, 0)))));
    });
    common::each(FAMILY, "is_extra_strong_lucas_pseudoprime", |c| {
        c.check(Some(flag(is_extra_strong_lucas_pseudoprime(&unsigned(
            c, 0,
        )))));
    });
    common::each_may_panic(FAMILY, "is_almost_extra_strong_lucas_pseudoprime", |c| {
        Some(flag(is_almost_extra_strong_lucas_pseudoprime(
            &unsigned(c, 0),
            c.arg(1),
        )))
    });
}

#[test]
fn lucas_sequence_oracle() {
    common::each_may_panic(FAMILY, "lucas_sequence", |c| {
        let (p, q): (BigInt, BigInt) = (c.arg(1), c.arg(2));
        let (u, v, qk) = lucas_sequence(&unsigned(c, 0), &p, &q, &unsigned(c, 3));
        Some(format!("{u} {v} {qk}"))
    });
}

#[test]
fn prime_walk_oracle() {
    common::each(FAMILY, "next_prime", |c| {
        c.check(Some(next_prime(&unsigned(c, 0))));
    });
    common::each(FAMILY, "prev_prime", |c| {
        c.check(prev_prime(&unsigned(c, 0)));
    });
}

#[test]
fn mersenne_oracle() {
    common::each(FAMILY, "is_mersenne_prime", |c| {
        c.check(Some(flag(is_mersenne_prime(c.arg(0)))));
    });
}

#[test]
fn every_row_is_replayed() {
    let handled = [
        "is_prime",
        "is_prob_prime",
        "is_bpsw_prime",
        "is_strong_pseudoprime",
        "is_pseudoprime",
        "is_euler_pseudoprime",
        "is_euler_plumb_pseudoprime",
        "is_lucas_pseudoprime",
        "is_strong_lucas_pseudoprime",
        "is_extra_strong_lucas_pseudoprime",
        "is_almost_extra_strong_lucas_pseudoprime",
        "lucas_sequence",
        "next_prime",
        "prev_prime",
        "is_mersenne_prime",
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
    assert!(cases.len() > 900, "{} rows", cases.len());
    // Inputs cover the 65 to 1000 bit range, including 100-digit primes
    // and the Mersenne primes 2^127 - 1, 2^521 - 1 and 2^607 - 1.
    let primes: Vec<BigUint> = cases
        .iter()
        .filter(|c| c.func == "is_prime" && c.result.as_deref() == Some("1"))
        .map(|c| unsigned(c, 0))
        .collect();
    assert!(primes.iter().any(|p| p.bits() >= 1000), "no 1000-bit prime");
    assert!(
        primes.iter().filter(|p| p.to_string().len() >= 100).count() >= 4,
        "too few 100-digit primes"
    );
    for bits in [127_usize, 521, 607] {
        let m = (BigUint::from(1_u8) << bits) - 1_u8;
        assert!(primes.contains(&m), "2^{bits} - 1 missing");
    }
    // Strong pseudoprimes base 2 above 2^64 that BPSW rejects: the Lucas half
    // of the test does the work.
    let hard = cases
        .iter()
        .filter(|c| {
            c.func == "is_strong_pseudoprime"
                && c.args.len() == 1
                && c.result.as_deref() == Some("1")
                && unsigned(c, 0).bits() > 64
                && is_prime(&unsigned(c, 0)) == 0
        })
        .count();
    assert!(hard >= 18, "{hard} strong pseudoprimes base 2");
}
