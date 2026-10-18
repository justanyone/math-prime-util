//! Native unsigned cases from the modular MPU oracle.
mod common;
use math_prime_util::{binomialmod, factorialmod, pisano_period, znlog};

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

mod cornacchia_divmod {
    //! Oracle compatibility tests for `cornacchia` and `divmod`.
    use super::common::{self, each};
    use math_prime_util::{cornacchia, divmod};

    /// Fixture rows whose arguments are negative are skipped: the Rust API takes
    /// unsigned arguments only, so `divmod` with a signed `a`, `b` or `n` has no
    /// Rust form yet.
    fn unsigned_args(c: &common::Case) -> bool {
        c.args.iter().all(|a| !a.starts_with('-'))
    }

    #[test]
    fn compat_cornacchia() {
        each("modular", "cornacchia", |c| {
            if !unsigned_args(c) {
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
            if !unsigned_args(c) {
                return;
            }
            let got = divmod(c.arg(0), c.arg(1), c.arg(2)).map(|r| r.to_string());
            assert_eq!(got, c.result.clone(), "line {}", c.line);
        });
    }
}
