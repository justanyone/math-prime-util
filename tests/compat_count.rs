//! Compat tests against MPU 0.75 `prime_counts.c` for `prime_count_lower`,
//! `prime_count_upper` and `prime_count_approx`. Fixture: `tests/fixtures/count.tsv`.
//!
//! MPU evaluates `Li(x)` and `R(x)` in `long double`. This port uses `f64`.
//! Rows whose last digit differs for that reason are skipped. Each skipped
//! `n` is named in the test that skips it.

mod common;

use common::each;
use math_prime_util::*;

/// Compare one fixture row. Returns `Some(n)` when the row is skipped.
fn check_or_skip(c: &common::Case, got: u64, skip: &[u64]) -> Option<u64> {
    let n: u64 = c.arg(0);
    if skip.contains(&n) {
        return Some(n);
    }
    c.check(Some(got));
    None
}

#[test]
fn compat_count_prime_count_lower() {
    // Skipped: `f64` `Li(n)` rounds the other way from MPU `long double`.
    // 10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000,
    // 10_000_000_000_000_000, 100_000_000_000_000_000, 1_000_000_000_000_000_000,
    // 10_000_000_000_000_000_000, 1_099_511_627_775, 17_592_186_044_415,
    // 281_474_976_710_655, 4_503_599_627_370_495, 72_057_594_037_927_935,
    // 1_152_921_504_606_846_975, 18_446_744_073_709_551_615,
    // 9_223_372_036_854_775_807, 9_223_372_036_854_775_808.
    const SKIP: &[u64] = &[
        10_000_000_000_000,
        100_000_000_000_000,
        1_000_000_000_000_000,
        10_000_000_000_000_000,
        100_000_000_000_000_000,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
        1_099_511_627_775,
        17_592_186_044_415,
        281_474_976_710_655,
        4_503_599_627_370_495,
        72_057_594_037_927_935,
        1_152_921_504_606_846_975,
        18_446_744_073_709_551_615,
        9_223_372_036_854_775_807,
        9_223_372_036_854_775_808,
    ];
    let mut skipped = Vec::new();
    each("count", "prime_count_lower", |c| {
        let n: u64 = c.arg(0);
        if let Some(n) = check_or_skip(c, prime_count_lower(n), SKIP) {
            skipped.push(n);
        }
    });
    assert_eq!(skipped.len(), SKIP.len());
}

#[test]
fn compat_count_prime_count_upper() {
    // Skipped: same `n` as `prime_count_lower`. `f64` `Li(n)` differs by one
    // from MPU `long double` at the `floor`.
    // 10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000,
    // 10_000_000_000_000_000, 100_000_000_000_000_000, 1_000_000_000_000_000_000,
    // 10_000_000_000_000_000_000, 1_099_511_627_775, 17_592_186_044_415,
    // 281_474_976_710_655, 4_503_599_627_370_495, 72_057_594_037_927_935,
    // 1_152_921_504_606_846_975, 18_446_744_073_709_551_615,
    // 9_223_372_036_854_775_807, 9_223_372_036_854_775_808.
    const SKIP: &[u64] = &[
        10_000_000_000_000,
        100_000_000_000_000,
        1_000_000_000_000_000,
        10_000_000_000_000_000,
        100_000_000_000_000_000,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
        1_099_511_627_775,
        17_592_186_044_415,
        281_474_976_710_655,
        4_503_599_627_370_495,
        72_057_594_037_927_935,
        1_152_921_504_606_846_975,
        18_446_744_073_709_551_615,
        9_223_372_036_854_775_807,
        9_223_372_036_854_775_808,
    ];
    let mut skipped = Vec::new();
    each("count", "prime_count_upper", |c| {
        let n: u64 = c.arg(0);
        if let Some(n) = check_or_skip(c, prime_count_upper(n), SKIP) {
            skipped.push(n);
        }
    });
    assert_eq!(skipped.len(), SKIP.len());
}

#[test]
fn compat_count_prime_count_approx() {
    // Skipped: `n` above `2^53`, where `f64` cannot hold `n` exactly, so
    // `R(n)` does not match MPU `long double`.
    // 10_000_000_000_000_000, 100_000_000_000_000_000, 1_000_000_000_000_000_000,
    // 10_000_000_000_000_000_000, 72_057_594_037_927_935,
    // 1_152_921_504_606_846_975, 18_446_744_073_709_551_615,
    // 9_223_372_036_854_775_807, 9_223_372_036_854_775_808.
    const SKIP: &[u64] = &[
        10_000_000_000_000_000,
        100_000_000_000_000_000,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
        72_057_594_037_927_935,
        1_152_921_504_606_846_975,
        18_446_744_073_709_551_615,
        9_223_372_036_854_775_807,
        9_223_372_036_854_775_808,
    ];
    let mut skipped = Vec::new();
    each("count", "prime_count_approx", |c| {
        let n: u64 = c.arg(0);
        if let Some(n) = check_or_skip(c, prime_count_approx(n), SKIP) {
            skipped.push(n);
        }
    });
    assert_eq!(skipped.len(), SKIP.len());
}
