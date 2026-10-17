//! MPU 0.75 combinat fixture checks for the assigned functions.
mod common;

use common::each;
use math_prime_util::{
    bernfrac, falling_factorial, harmfrac, integer_complexity, pn_primorial, powersum, primorial,
    rising_factorial,
};

fn check_binary(name: &str, function: fn(u64, u64) -> Option<u64>) {
    each("combinat", name, |case| {
        // Skip negative n (signed factorial API), and bigint n or m inputs.
        let (Ok(n), Ok(m)) = (case.args[0].parse::<u64>(), case.args[1].parse::<u64>()) else {
            return;
        };
        assert!(!case.error, "unexpected oracle error: {case:?}");
        // MPU promotes overflowing results to bigint; this native API returns None.
        let expected = case
            .result
            .as_ref()
            .and_then(|value| value.parse::<u64>().ok());
        assert_eq!(function(n, m), expected, "{case:?}");
    });
}

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
fn falling_factorial_oracle() {
    check_binary("falling_factorial", falling_factorial);
}

#[test]
fn rising_factorial_oracle() {
    check_binary("rising_factorial", rising_factorial);
}

#[test]
fn powersum_oracle() {
    check_binary("powersum", powersum);
}

#[test]
fn integer_complexity_oracle() {
    each("combinat", "integer_complexity", |case| {
        case.check(integer_complexity(case.arg(0)));
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
