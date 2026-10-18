//! Compat tests against MPU for `rootmod.c` (`sqrtmod`, `allsqrtmod`,
//! `rootmod`, `allrootmod`), `util.c` (`cornacchia`, `divmod`), and
//! `pisano_period`. Fixture: `tests/fixtures/modular.tsv`.

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
#[test]
fn pisano_period_oracle() {
    // Two fixture rows use moduli past `u64` (Perl bigint). Skip those;
    // every u64 row must match, which fails if the native export is dropped.
    each("modular", "pisano_period", |c| {
        if c.args.iter().all(|a| a.parse::<u64>().is_ok()) {
            c.check(Some(pisano_period(c.arg(0))));
        }
    });
}

mod roots {
    //! Oracle compatibility tests for the four modular root APIs.
    use super::common;
    use math_prime_util::{allrootmod, allsqrtmod, rootmod, sqrtmod};

    fn unsigned(c: &common::Case) -> bool {
        // Skip signed radicands (such as sqrtmod(-1, 17)) and negative
        // exponents (rootmod(14, -3, 101), (13, -6, 107), and (9, -2, 101)).
        // Bigint arguments also need a future API.
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
}
>>>>>>> origin/dev-001
