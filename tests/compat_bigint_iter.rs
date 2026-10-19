//! The big integer range iterators against MPU 0.75 (pure Perl) on inputs
//! above 2^64: `tests/fixtures/bigint_iter.tsv`.
#![cfg(feature = "bigint")]
mod common;
use common::Case;
use math_prime_util::bigint::iter::*;
use num_bigint::BigUint;

const FAMILY: &str = "bigint_iter";

fn big(c: &Case, i: usize) -> BigUint {
    c.arg(i)
}

/// A list result: `None` (the oracle's `undef`) for an empty list.
fn list(values: &[BigUint]) -> Option<String> {
    (!values.is_empty()).then(|| {
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    })
}

fn replay(func: &str, run: impl Fn(&Case) -> Vec<BigUint>) {
    let mut handled = 0;
    common::each(FAMILY, func, |c| {
        c.check(list(&run(c)));
        handled += 1;
    });
    assert!(handled > 0, "no {func} rows in {FAMILY}");
}

#[test]
fn fordivisors_rows() {
    replay("fordivisors", |c| fordivisors(&big(c, 0)).unwrap());
}

#[test]
fn foralmostprimes_rows() {
    replay("foralmostprimes", |c| {
        foralmostprimes(c.arg(0), &big(c, 1), &big(c, 2)).unwrap()
    });
}

#[test]
fn forsemiprimes_rows() {
    replay("forsemiprimes", |c| {
        forsemiprimes(&big(c, 0), &big(c, 1)).unwrap()
    });
}

#[test]
fn forsquarefree_rows() {
    replay("forsquarefree", |c| {
        forsquarefree(&big(c, 0), &big(c, 1)).unwrap()
    });
    replay("forsquarefreeint", |c| {
        forsquarefreeint(&big(c, 0), &big(c, 1)).unwrap()
    });
}

#[test]
fn forfactored_rows() {
    replay("forfactored", |c| {
        forfactored(&big(c, 0), &big(c, 1))
            .unwrap()
            .into_iter()
            .flatten()
            .collect()
    });
}
