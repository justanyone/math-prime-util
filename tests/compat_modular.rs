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
