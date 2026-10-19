//! Prime counts, bounds, estimates, the nth prime and `is_polygonal` beyond
//! the 64-bit range against MPU 0.75 (pure Perl, `Math::BigFloat` at 40
//! digits): `tests/fixtures/bigint_counts.tsv`.
#![cfg(feature = "bigint")]
mod common;
use common::Case;
use math_prime_util::bigint::counts::*;
use num_bigint::BigUint;

const FAMILY: &str = "bigint_counts";

fn big(c: &Case, i: usize) -> BigUint {
    c.arg(i)
}

/// Inputs whose bounds have more than 40 digits: `Math::BigFloat` carries 40
/// digits through `log` in MPU, so the oracle's bounds end in noise there
/// (`docs/upstream-findings.md` entry 16). The port keeps 77 digits; these
/// rows are compared on their leading 36 digits.
const PRECISION_LOSS: [&str; 3] = [
    "170141183460469231731687303715884105728",
    "10000000000000000000000000000000000000000",
    "1606938044258990275541962092341162602522202993782792835301376",
];

/// Checks `got` against the oracle row, or, for a [`PRECISION_LOSS`] input,
/// the leading `digits` digits and the length. Returns whether the row was a
/// precision-loss row.
fn check_leading(c: &Case, got: &BigUint, digits: usize) -> bool {
    if !PRECISION_LOSS.contains(&c.args[0].as_str()) {
        c.check(Some(got));
        return false;
    }
    let got = got.to_string();
    let want = c.result.as_deref().expect("a value");
    assert_eq!(got.len(), want.len(), "fixture line {}", c.line);
    assert_eq!(got[..digits], want[..digits], "fixture line {}", c.line);
    true
}

#[test]
fn nth_prime_bounds_oracle() {
    let mut loss = 0;
    common::each(FAMILY, "nth_prime_lower", |c| {
        let got = nth_prime_lower(&big(c, 0)).expect("a bound");
        loss += usize::from(check_leading(c, &got, 36));
    });
    common::each(FAMILY, "nth_prime_upper", |c| {
        let got = nth_prime_upper(&big(c, 0)).expect("a bound");
        loss += usize::from(check_leading(c, &got, 36));
    });
    assert_eq!(loss, 6);
}

#[test]
fn nth_prime_approx_oracle() {
    // Skipped on every row, named here: the pure Perl `toint` keeps 15
    // significant digits of the double estimate, and the pure Perl build
    // iterates in `double`, a few ulps from the `long double` value of the
    // XS build that the port follows (docs/upstream-findings.md entry 15;
    // the XS rows above the largest 64-bit prime index in `count.tsv` are
    // replayed exactly in `tests/compat_count.rs`). The values must agree to
    // 14 significant digits.
    let mut rows = 0;
    common::each(FAMILY, "nth_prime_approx", |c| {
        let got = nth_prime_approx(&big(c, 0)).expect("an estimate");
        let want: BigUint = c
            .result
            .as_deref()
            .expect("a value")
            .parse()
            .expect("decimal");
        let difference = if got > want {
            &got - &want
        } else {
            &want - &got
        };
        assert!(
            difference * BigUint::from(10_u64.pow(14)) <= want,
            "fixture line {}: {got} vs {want}",
            c.line
        );
        rows += 1;
    });
    assert_eq!(rows, 22);
}

#[test]
fn prime_count_bounds_oracle() {
    let mut loss = 0;
    common::each(FAMILY, "prime_count_lower", |c| {
        loss += usize::from(check_leading(c, &prime_count_lower(&big(c, 0)), 36));
    });
    common::each(FAMILY, "prime_count_upper", |c| {
        loss += usize::from(check_leading(c, &prime_count_upper(&big(c, 0)), 36));
    });
    assert_eq!(loss, 6);
}

#[test]
fn prime_count_approx_oracle() {
    common::each(FAMILY, "prime_count_approx", |c| {
        c.check(Some(prime_count_approx(&big(c, 0))));
    });
}

#[test]
fn nth_prime_oracle() {
    common::each(FAMILY, "nth_prime", |c| {
        c.check(nth_prime(&big(c, 0)));
    });
}

#[test]
fn is_polygonal_oracle() {
    common::each(FAMILY, "is_polygonal", |c| {
        c.check(Some(u8::from(is_polygonal(&big(c, 0), &big(c, 1)))));
    });
}

#[test]
fn every_row_is_replayed() {
    let handled = [
        "nth_prime_lower",
        "nth_prime_upper",
        "nth_prime_approx",
        "prime_count_lower",
        "prime_count_upper",
        "prime_count_approx",
        "nth_prime",
        "is_polygonal",
    ];
    let cases = common::load(FAMILY);
    for c in &cases {
        assert!(
            handled.contains(&c.func.as_str()),
            "fixture line {}: {}",
            c.line,
            c.func
        );
        assert!(!c.error, "fixture line {}: oracle died", c.line);
    }
    assert!(cases.len() > 200, "{} rows", cases.len());
    // The branch limits of the pure Perl count bounds are covered on both
    // sides: 10^20 and 1.101e26.
    for n in [
        "99999999999999999999",
        "100000000000000000000",
        "100000000000000000001",
        "110099999999999999999999999",
        "110100000000000000000000000",
    ] {
        assert!(
            cases
                .iter()
                .any(|c| c.func == "prime_count_lower" && c.args[0] == n),
            "no row at {n}"
        );
    }
}
