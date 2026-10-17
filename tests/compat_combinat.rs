//! Compatibility with the committed MPU combinat oracle.
mod common;

use common::{Case, each};
use math_prime_util::{
    bell_number, bernfrac, binomial, catalan_number, factorial, fubini, harmfrac, multifactorial,
    pn_primorial, primorial, stirling, subfactorial,
};

fn check_fraction(c: &common::Case, got: Option<(i128, u128)>) {
    // Fractions outside i128/u128 are represented by None in this native API.
    let expected = c.result.as_deref().and_then(|s| {
        let (num, den) = s.split_once(' ')?;
        Some((num.parse::<i128>().ok()?, den.parse::<u128>().ok()?))
    });
    assert!(!c.error);
    assert_eq!(got, expected, "{} fixture line {}", c.func, c.line);
}

#[test]
fn bernfrac_oracle() {
    each("combinat", "bernfrac", |c| {
        check_fraction(c, bernfrac(c.arg(0)));
    });
}

#[test]
fn harmfrac_oracle() {
    each("combinat", "harmfrac", |c| {
        check_fraction(c, harmfrac(c.arg(0)));
    });
}

#[test]
fn primorial_oracle() {
    each("combinat", "primorial", |c| {
        let expected = c.result.as_deref().and_then(|s| s.parse::<u64>().ok());
        assert!(!c.error);
        assert_eq!(primorial(c.arg(0)), expected, "fixture line {}", c.line);
    });
}

#[test]
fn pn_primorial_oracle() {
    each("combinat", "pn_primorial", |c| {
        let expected = c.result.as_deref().and_then(|s| s.parse::<u64>().ok());
        assert!(!c.error);
        assert_eq!(pn_primorial(c.arg(0)), expected, "fixture line {}", c.line);
    });
}

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

fn check_native(case: &Case, function: impl FnOnce(&[u64]) -> Option<u64>) {
    let Ok(args) = case
        .args
        .iter()
        .map(|arg| arg.parse())
        .collect::<Result<Vec<u64>, _>>()
    else {
        // Skip negative factorial/subfactorial/multifactorial arguments and
        // the multifactorial case whose n and k exceed u64.
        return;
    };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| function(&args)));
    if case.error {
        assert!(result.is_err(), "expected panic: {case:?}");
    } else {
        // MPU's bigint answers beyond u64 map to the native API's None.
        let expected = case
            .result
            .as_ref()
            .and_then(|value| value.parse::<u64>().ok());
        assert_eq!(result.unwrap(), expected, "{case:?}");
    }
}

#[test]
fn fubini_oracle() {
    each("combinat", "fubini", |c| check_native(c, |a| fubini(a[0])));
}

#[test]
fn factorial_oracle() {
    each("combinat", "factorial", |c| {
        check_native(c, |a| factorial(a[0]));
    });
}

#[test]
fn multifactorial_oracle() {
    each("combinat", "multifactorial", |c| {
        check_native(c, |a| multifactorial(a[0], a[1]));
    });
}

#[test]
fn subfactorial_oracle() {
    each("combinat", "subfactorial", |c| {
        check_native(c, |a| subfactorial(a[0]));
    });
}
