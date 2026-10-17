//! Compat tests against MPU for `t/19-rational.t` and `t/26-farey.t`.
//! Fixture: `tests/fixtures/rational.tsv`.
//!
//! Native API skips: signed or bigint arguments in contfrac, convergents,
//! from_contfrac, next_calkin_wilf, next_stern_brocot, calkin_wilf_n,
//! stern_brocot_n, nth_calkin_wilf, nth_stern_brocot and nth_stern_diatomic.
//! Bigint results in tree successors, indices and continued fractions are
//! skipped. next_farey with a reduced denominator above the order uses
//! Perl fallback semantics and is skipped. bestrational skips non-finite
//! inputs, whose validation belongs to the Perl wrapper. Tree index overflow
//! (including a Perl fallback error) and Stern-Brocot successors needing
//! an index above u64 are skipped; the native C path returns failure.

mod common;

use common::Case;
use math_prime_util::*;
use std::fmt::Display;

fn each_native<T: Display>(name: &str, f: impl Fn(&Case) -> Option<T>) {
    let mut count = 0;
    for c in common::load("rational").iter().filter(|c| c.func == name) {
        if name == "bestrational" {
            if c.args[0].parse::<f64>().map_or(true, |x| !x.is_finite()) {
                continue;
            }
        } else if name != "next_farey"
            && name != "farey_rank"
            && c.args.iter().any(|a| a.parse::<u64>().is_err())
        {
            continue;
        }
        if !c.error
            && name != "bestrational"
            && c.result
                .as_ref()
                .is_some_and(|r| r.split_whitespace().any(|v| v.parse::<u64>().is_err()))
        {
            continue;
        }
        if name == "next_farey" {
            let (p, q) = frac(&c.args[1]);
            if q > c.arg::<u64>(0) && q / math_prime_util::gcd(p, q) > c.arg::<u64>(0) {
                continue;
            }
        }
        if matches!(
            name,
            "calkin_wilf_n" | "stern_brocot_n" | "next_stern_brocot"
        ) {
            let (p, q) = (c.arg::<u64>(0), c.arg::<u64>(1));
            if p > 0 && q > 0 && math_prime_util::gcd(p, q) == 1 && calkin_wilf_n(p, q).is_none() {
                continue;
            }
        }
        let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(c)));
        if c.error {
            assert!(got.is_err(), "oracle expected panic: {c:?}");
        } else {
            c.check(got.unwrap_or_else(|_| panic!("unexpected panic: {c:?}")));
        }
        count += 1;
    }
    assert!(count > 0, "no cases for {name}");
}

/// Parses a fixture `p/q` argument.
fn frac(s: &str) -> (u64, u64) {
    let (p, q) = s.split_once('/').expect("p/q");
    (p.parse().unwrap(), q.parse().unwrap())
}

#[test]
fn contfrac_oracle() {
    each_native("contfrac", |c| {
        let got: Vec<String> = contfrac(c.arg(0), c.arg(1))
            .iter()
            .map(u64::to_string)
            .collect();
        Some(got.join(" "))
    });
}

#[test]
fn from_contfrac_oracle() {
    each_native("from_contfrac", |c| {
        let cf: Vec<u64> = c.args.iter().map(|s| s.parse().unwrap()).collect();
        let (n, d) = from_contfrac(&cf).expect("from_contfrac");
        Some(format!("{n} {d}"))
    });
}

#[test]
fn convergents_oracle() {
    each_native("convergents", |c| {
        if c.args.is_empty() {
            return Some(String::new());
        }
        let cf = contfrac(c.arg(0), c.arg(1));
        let pairs = convergents(&cf).expect("convergents");
        let got: Vec<String> = pairs
            .iter()
            .flat_map(|&(p, q)| [p.to_string(), q.to_string()])
            .collect();
        Some(got.join(" "))
    });
}

#[test]
fn bestrational_oracle() {
    each_native("bestrational", |c| {
        let x: f64 = c.arg(0);
        let got = bestrational(x, c.arg(1)).map(|(n, d)| format!("{n} {d}"));
        got
    });
}

#[test]
fn next_calkin_wilf_oracle() {
    each_native("next_calkin_wilf", |c| {
        next_calkin_wilf(c.arg(0), c.arg(1)).map(|(a, b)| format!("{a} {b}"))
    });
}

#[test]
fn next_stern_brocot_oracle() {
    each_native("next_stern_brocot", |c| {
        next_stern_brocot(c.arg(0), c.arg(1)).map(|(a, b)| format!("{a} {b}"))
    });
}

#[test]
fn calkin_wilf_n_oracle() {
    each_native("calkin_wilf_n", |c| calkin_wilf_n(c.arg(0), c.arg(1)));
}

#[test]
fn stern_brocot_n_oracle() {
    each_native("stern_brocot_n", |c| stern_brocot_n(c.arg(0), c.arg(1)));
}

#[test]
fn nth_calkin_wilf_oracle() {
    each_native("nth_calkin_wilf", |c| {
        let Ok(n) = c.args[0].parse::<u64>() else {
            return None;
        };
        let (p, q) = nth_calkin_wilf(n);
        Some(format!("{p} {q}"))
    });
}

#[test]
fn nth_stern_brocot_oracle() {
    each_native("nth_stern_brocot", |c| {
        let (p, q) = nth_stern_brocot(c.arg(0));
        Some(format!("{p} {q}"))
    });
}

#[test]
fn nth_stern_diatomic_oracle() {
    each_native("nth_stern_diatomic", |c| Some(nth_stern_diatomic(c.arg(0))));
}

#[test]
fn next_farey_oracle() {
    each_native("next_farey", |c| {
        let (p, q) = frac(&c.args[1]);
        next_farey(c.arg(0), p as u32, q as u32).map(|(a, b)| format!("{a} {b}"))
    });
}

#[test]
fn farey_rank_oracle() {
    each_native("farey_rank", |c| {
        let (p, q) = frac(&c.args[1]);
        Some(farey_rank(c.arg(0), p as u32, q as u32))
    });
}

#[test]
fn convergents_list_oracle() {
    each_native("convergents_list", |c| {
        let cf: Vec<u64> = c.args.iter().map(|a| a.parse().unwrap()).collect();
        assert!(
            cf.iter().skip(1).all(|&a| a > 0),
            "invalid continued fraction"
        );
        convergents(&cf).map(|v| {
            v.iter()
                .flat_map(|&(p, q)| [p.to_string(), q.to_string()])
                .collect::<Vec<_>>()
                .join(" ")
        })
    });
}

#[test]
fn farey_oracle() {
    each_native("farey", |c| {
        if c.args.len() == 2 {
            farey_nth(c.arg(0), c.arg(1)).map(|(p, q)| format!("{p} {q}"))
        } else {
            Some(
                farey(c.arg(0))
                    .iter()
                    .flat_map(|&(p, q)| [p.to_string(), q.to_string()])
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        }
    });
}

#[test]
fn farey_len_oracle() {
    each_native("farey_len", |c| Some(farey_len(c.arg(0))));
}
