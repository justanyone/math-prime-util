//! Compat tests against MPU for `t/19-rational.t` and `t/26-farey.t`.
//! Fixture: `tests/fixtures/rational.tsv`.
//!
//! Skipped: `farey` and `farey_len` (MPU `kth_farey`, `farey_length`),
//! not ported yet. `convergents` takes a continued fraction in Rust; the
//! fixture gives `num den`, so the test expands it with `contfrac` first.
//! Skipped cases: `from_contfrac` with an empty list (oracle `0 1`; Rust
//! takes a non-empty list); `nth_calkin_wilf 18446744073709551616` (over
//! `u64`); `next_farey` where `q > n` (oracle accepts, MPU C returns 0).

mod common;

use common::{each, each_may_panic};
use math_prime_util::*;

/// Parses a fixture `p/q` argument.
fn frac(s: &str) -> (u64, u64) {
    let (p, q) = s.split_once('/').expect("p/q");
    (p.parse().unwrap(), q.parse().unwrap())
}

#[test]
fn contfrac_oracle() {
    each("rational", "contfrac", |c| {
        let got: Vec<String> = contfrac(c.arg(0), c.arg(1))
            .iter()
            .map(u64::to_string)
            .collect();
        c.check(Some(got.join(" ")));
    });
}

#[test]
fn from_contfrac_oracle() {
    each("rational", "from_contfrac", |c| {
        if c.args.is_empty() {
            return;
        }
        let cf: Vec<u64> = c.args.iter().map(|s| s.parse().unwrap()).collect();
        let (n, d) = from_contfrac(&cf).expect("from_contfrac");
        c.check(Some(format!("{n} {d}")));
    });
}

#[test]
fn convergents_oracle() {
    each("rational", "convergents", |c| {
        let cf = contfrac(c.arg(0), c.arg(1));
        let pairs = convergents(&cf).expect("convergents");
        let got: Vec<String> = pairs
            .iter()
            .flat_map(|&(p, q)| [p.to_string(), q.to_string()])
            .collect();
        c.check(Some(got.join(" ")));
    });
}

#[test]
fn bestrational_oracle() {
    each("rational", "bestrational", |c| {
        let x: f64 = c.arg(0);
        let got = bestrational(x, c.arg(1)).map(|(n, d)| format!("{n} {d}"));
        c.check(got);
    });
}

#[test]
fn next_calkin_wilf_oracle() {
    each_may_panic("rational", "next_calkin_wilf", |c| {
        next_calkin_wilf(c.arg(0), c.arg(1)).map(|(a, b)| format!("{a} {b}"))
    });
}

#[test]
fn next_stern_brocot_oracle() {
    each_may_panic("rational", "next_stern_brocot", |c| {
        next_stern_brocot(c.arg(0), c.arg(1)).map(|(a, b)| format!("{a} {b}"))
    });
}

#[test]
fn calkin_wilf_n_oracle() {
    each_may_panic("rational", "calkin_wilf_n", |c| {
        calkin_wilf_n(c.arg(0), c.arg(1))
    });
}

#[test]
fn stern_brocot_n_oracle() {
    each_may_panic("rational", "stern_brocot_n", |c| {
        stern_brocot_n(c.arg(0), c.arg(1))
    });
}

#[test]
fn nth_calkin_wilf_oracle() {
    each_may_panic("rational", "nth_calkin_wilf", |c| {
        let Ok(n) = c.args[0].parse::<u64>() else {
            return None;
        };
        let (p, q) = nth_calkin_wilf(n);
        Some(format!("{p} {q}"))
    });
}

#[test]
fn nth_stern_brocot_oracle() {
    each_may_panic("rational", "nth_stern_brocot", |c| {
        let (p, q) = nth_stern_brocot(c.arg(0));
        Some(format!("{p} {q}"))
    });
}

#[test]
fn nth_stern_diatomic_oracle() {
    each("rational", "nth_stern_diatomic", |c| {
        c.check(Some(nth_stern_diatomic(c.arg(0))));
    });
}

#[test]
fn next_farey_oracle() {
    each_may_panic("rational", "next_farey", |c| {
        let (p, q) = frac(&c.args[1]);
        if q > c.arg::<u64>(0) {
            return None;
        }
        next_farey(c.arg(0), p as u32, q as u32).map(|(a, b)| format!("{a} {b}"))
    });
}

#[test]
fn farey_rank_oracle() {
    each_may_panic("rational", "farey_rank", |c| {
        let (p, q) = frac(&c.args[1]);
        Some(farey_rank(c.arg(0), p as u32, q as u32))
    });
}
