//! Compat tests against MPU for `rootmod.c` and `util.c` (`cornacchia`,
//! `divmod`). Fixture: `tests/fixtures/modular.tsv`.

mod common;

use common::each;
use math_prime_util::*;

/// Fixture rows whose arguments are negative are skipped: the Rust API takes
/// unsigned arguments only, so `divmod` with a signed `a`, `b` or `n` has no
/// Rust form yet.
fn unsigned(c: &common::Case) -> bool {
    c.args.iter().all(|a| !a.starts_with('-'))
}

#[test]
fn compat_cornacchia() {
    each("modular", "cornacchia", |c| {
        if !unsigned(c) {
            return;
        }
        let (d, p) = (c.arg::<u64>(0), c.arg::<u64>(1));
        let got = cornacchia(d, p).map(|(x, y)| format!("{x} {y}"));
        assert_eq!(got, c.result.clone(), "line {}", c.line);
    });
}

#[test]
fn compat_divmod() {
    each("modular", "divmod", |c| {
        if !unsigned(c) {
            return;
        }
        let got = divmod(c.arg(0), c.arg(1), c.arg(2)).map(|r| r.to_string());
        assert_eq!(got, c.result.clone(), "line {}", c.line);
    });
}

/// Oracle checks for `znlog`, `factorialmod`, `binomialmod` and `pisano_period`.
fn native_cases(name: &str, f: impl Fn(&common::Case)) {
    for c in common::load("modular").iter().filter(|c| c.func == name) {
        // Skip signed binomialmod n/k and bigint inputs to all four functions:
        // these cannot be represented by this native u64 API.
        if c.args.iter().any(|s| s.parse::<u64>().is_err()) {
            continue;
        }
        f(c);
    }
}

#[test]
fn znlog_oracle() {
    native_cases("znlog", |c| c.check(znlog(c.arg(0), c.arg(1), c.arg(2))));
}

#[test]
fn factorialmod_oracle() {
    native_cases("factorialmod", |c| {
        c.check(factorialmod(c.arg(0), c.arg(1)));
    });
}

#[test]
fn binomialmod_oracle() {
    native_cases("binomialmod", |c| {
        c.check(binomialmod(c.arg(0), c.arg(1), c.arg(2)));
    });
}

#[test]
fn pisano_period_oracle() {
    native_cases("pisano_period", |c| c.check(Some(pisano_period(c.arg(0)))));
}
