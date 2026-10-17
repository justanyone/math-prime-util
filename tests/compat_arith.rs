//! Compat tests against MPU 0.75 `inverse_sigma0.c`. Fixture: `tests/fixtures/arith.tsv`.
//! A two-argument case is `k hi` with `lo = 1`; a three-argument case is `k lo hi`.

mod common;

use common::{each, Case};
use math_prime_util::*;

fn range(c: &Case) -> (u64, u64, u64) {
    if c.args.len() == 2 {
        (c.arg(0), 1, c.arg(1))
    } else {
        (c.arg(0), c.arg(1), c.arg(2))
    }
}

#[test]
fn inverse_sigma0_oracle() {
    each("arith", "inverse_sigma0", |c| {
        let (k, lo, hi) = range(c);
        let got = inverse_sigma0(k, lo, hi)
            .map(|v| v.iter().map(u64::to_string).collect::<Vec<_>>().join(" "));
        c.check(got);
    });
}

#[test]
fn inverse_sigma0_count_oracle() {
    each("arith", "inverse_sigma0_count", |c| {
        let (k, lo, hi) = range(c);
        c.check(inverse_sigma0_count(k, lo, hi));
    });
}
