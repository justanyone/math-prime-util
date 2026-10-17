//! Native-range comparisons against the committed MPU combinatorics oracle.
mod common;

use common::each;
use math_prime_util::{bell_number, binomial, catalan_number, stirling};

fn check_unsigned(c: &common::Case, got: Option<u64>) {
    // The oracle also contains bigint answers; the native API returns None.
    let expected = c.result.as_ref().and_then(|s| s.parse::<u64>().ok());
    assert!(!c.error);
    assert_eq!(got, expected, "fixture line {}: {c:?}", c.line);
}

#[test]
fn binomial_oracle() {
    each("combinat", "binomial", |c| {
        // Signed and bigint binomial arguments are outside this u64 API.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            return;
        }
        check_unsigned(c, binomial(c.arg(0), c.arg(1)));
    });
}

#[test]
fn catalan_number_oracle() {
    each("combinat", "catalan_number", |c| {
        check_unsigned(c, catalan_number(c.arg(0)));
    });
}

#[test]
fn stirling_oracle() {
    each("combinat", "stirling", |c| {
        let kind = if c.args.len() == 3 { c.arg(2) } else { 1 };
        let got = std::panic::catch_unwind(|| stirling(c.arg(0), c.arg(1), kind));
        if c.error {
            assert!(got.is_err(), "fixture line {}", c.line);
        } else {
            let expected = c
                .result
                .as_ref()
                .and_then(|s| s.parse::<i128>().ok())
                .filter(|v| v.unsigned_abs() <= u128::from(u64::MAX));
            assert_eq!(got.unwrap(), expected, "fixture line {}: {c:?}", c.line);
        }
    });
}

#[test]
fn bell_number_oracle() {
    each("combinat", "bell_number", |c| {
        check_unsigned(c, bell_number(c.arg(0)));
    });
}
