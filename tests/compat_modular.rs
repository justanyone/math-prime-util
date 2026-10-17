//! Oracle compatibility tests for the four modular root APIs.
mod common;
use math_prime_util::{allrootmod, allsqrtmod, rootmod, sqrtmod};

fn unsigned(c: &common::Case) -> bool {
    // Signed radicands and negative exponents need a future signed API.
    // Skip each such fixture case by its parsed argument, rather than changing it.
    c.args.iter().all(|a| a.parse::<u64>().is_ok())
}
fn list(c: &common::Case, roots: &[u64]) {
    let got = if roots.is_empty() {
        None
    } else {
        Some(
            roots
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        )
    };
    c.check(got);
}
#[test]
fn compat_sqrtmod() {
    common::each("modular", "sqrtmod", |c| {
        if unsigned(c) {
            c.check(sqrtmod(c.arg(0), c.arg(1)));
        }
    });
}
#[test]
fn compat_allsqrtmod() {
    common::each("modular", "allsqrtmod", |c| {
        if unsigned(c) {
            list(c, &allsqrtmod(c.arg(0), c.arg(1)));
        }
    });
}
#[test]
fn compat_rootmod() {
    common::each("modular", "rootmod", |c| {
        if unsigned(c) {
            c.check(rootmod(c.arg(0), c.arg(1), c.arg(2)));
        }
    });
}
#[test]
fn compat_allrootmod() {
    common::each("modular", "allrootmod", |c| {
        if unsigned(c) {
            list(c, &allrootmod(c.arg(0), c.arg(1), c.arg(2)));
        }
    });
}
