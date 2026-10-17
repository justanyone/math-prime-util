//! Oracle checks for the assigned combinat functions from MPU 0.75.
//! Compatibility with the committed MPU combinat oracle.
mod common;

use common::each;
use math_prime_util::{
    bernfrac, harmfrac, hclassno, is_happy, is_pillai, numtoperm, pn_primorial, primorial,
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
fn is_pillai_oracle() {
    each("combinat", "is_pillai", |c| {
        c.check(Some(is_pillai(c.arg(0))));
    });
}

#[test]
fn bernfrac_oracle() {
    each("combinat", "bernfrac", |c| {
        check_fraction(c, bernfrac(c.arg(0)));
    });
}

#[test]
fn is_happy_oracle() {
    each("combinat", "is_happy", |c| {
        // Skip is_happy arguments above u64::MAX; the native API takes u64.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let base = c.args.get(1).map_or(10, |s| s.parse().unwrap());
        let exponent = c.args.get(2).map_or(2, |s| s.parse().unwrap());
        c.check(Some(is_happy(n, base, exponent)));
    });
}

#[test]
fn harmfrac_oracle() {
    each("combinat", "harmfrac", |c| {
        check_fraction(c, harmfrac(c.arg(0)));
    });
}

#[test]
fn hclassno_oracle() {
    each("combinat", "hclassno", |c| {
        // Skip hclassno(-3), hclassno(-2), hclassno(-1): unsigned native input.
        if c.args[0].starts_with('-') {
            return;
        }
        c.check(hclassno(c.arg(0)));
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
fn numtoperm_oracle() {
    each("combinat", "numtoperm", |c| {
        // Skip numtoperm(5, -1) and numtoperm(5, 100000000000000000000):
        // the native rank is unsigned and limited to u64.
        let Ok(rank) = c.args[1].parse::<u64>() else {
            return;
        };
        let value = numtoperm(c.arg(0), rank)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        c.check(Some(value));
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
