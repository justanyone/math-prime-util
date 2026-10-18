//! Compat tests against MPU 0.75 `prime_counts.c` for the prime counts, their
//! bounds and the nth-prime bounds. Fixture: `tests/fixtures/count.tsv`.
//!
//! MPU evaluates `Li(x)` and `R(x)` in `long double`. The `prime_count_*`
//! bounds of this port use `f64`; rows whose last digit differs for that
//! reason are skipped. The `nth_prime_*` functions emulate the x87
//! `long double` and match every row in the `u64` range. Each skipped `n` is
//! named in the test that skips it.

mod common;

use common::{each, load};
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

/// `nth_prime_*` rows with `n` above `MPU_MAX_PRIME_IDX`
/// (`425_656_284_035_217_743`): MPU's XS hands them to its bigint Perl code,
/// and the `n`th prime is above `u64::MAX`, so the Rust functions return
/// `None` there. Skipped for each of the three functions:
/// `425_656_284_035_217_744` and `425_656_284_035_217_745`.
const ABOVE_MAX_PRIME_IDX: [u64; 2] = [425_656_284_035_217_744, 425_656_284_035_217_745];

/// Checks every fixture row of `func`, skipping only [`ABOVE_MAX_PRIME_IDX`].
fn check_nth(func: &str, f: fn(u64) -> Option<u64>) {
    let mut skipped = Vec::new();
    each("count", func, |c| {
        let n: u64 = c.arg(0);
        if ABOVE_MAX_PRIME_IDX.contains(&n) {
            assert_eq!(f(n), None, "{func}({n})");
            skipped.push(n);
        } else {
            c.check(f(n));
        }
    });
    assert_eq!(skipped, ABOVE_MAX_PRIME_IDX);
}

#[test]
fn compat_count_nth_prime_lower() {
    check_nth("nth_prime_lower", nth_prime_lower);
}

#[test]
fn compat_count_nth_prime_upper() {
    check_nth("nth_prime_upper", nth_prime_upper);
}

#[test]
fn compat_count_nth_prime_approx() {
    check_nth("nth_prime_approx", nth_prime_approx);
}

#[test]
fn prime_count_oracle() {
    each("count", "prime_count", |c| {
        let value = if c.args.len() == 2 {
            primes_in(c.arg(0)..=c.arg(1)).count() as u64
        } else {
            prime_count(c.arg(0))
        };
        c.check(Some(value));
    });
}

#[test]
fn legendre_phi_oracle() {
    each("count", "legendre_phi", |c| {
        c.check(Some(legendre_phi(c.arg(0), c.arg(1))));
    });
}

#[test]
fn sum_primes_oracle() {
    each("count", "sum_primes", |c| {
        let (lo, hi) = if c.args.len() == 1 {
            (0, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        c.check(sum_primes(lo, hi));
    });
}

#[test]
fn nth_prime_oracle() {
    each("count", "nth_prime", |c| {
        c.check(nth_prime(c.arg(0)));
    });
}

/// `chebyshev_theta` and `chebyshev_psi` are floats: MPU prints 15 digits
/// and sums in long double, so compare with a relative tolerance.
/// Cases with an argument above 10^8 are skipped: the `f64` sum needs a
/// sieve up to that argument, which is too slow for a default test.
#[test]
fn chebyshev_oracle() {
    for c in load("count")
        .iter()
        .filter(|c| c.func.starts_with("chebyshev_") && c.arg::<u64>(0) <= 100_000_000)
    {
        let want: f64 = c
            .result
            .as_deref()
            .expect("chebyshev results are defined")
            .parse()
            .unwrap();
        let got = if c.func == "chebyshev_theta" {
            chebyshev_theta(c.arg(0))
        } else {
            chebyshev_psi(c.arg(0))
        };
        assert!(
            (got - want).abs() <= 1e-9 * want.abs().max(1.0),
            "fixture line {}: {} {got} vs {want}",
            c.line,
            c.func
        );
    }
}
