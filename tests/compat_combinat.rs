//! Oracle checks for the assigned combinat functions from MPU 0.75.
mod common;

use common::each;
use math_prime_util::{hclassno, is_happy, is_pillai, numtoperm};

#[test]
fn is_pillai_oracle() {
    each("combinat", "is_pillai", |c| {
        c.check(Some(is_pillai(c.arg(0))));
    });
}

#[test]
fn is_happy_oracle() {
    each("combinat", "is_happy", |c| {
        // Skip is_happy arguments above u64::MAX; the native API takes u64.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let base = c.args.get(1).map_or(10, |s| s.parse().unwrap());
        let exponent = c.args.get(2).map_or(2, |s| s.parse().unwrap());
        c.check(Some(is_happy(n, base, exponent)));
    });
}

#[test]
fn hclassno_oracle() {
    each("combinat", "hclassno", |c| {
        // Skip hclassno(-3), hclassno(-2), hclassno(-1): unsigned native input.
        if c.args[0].starts_with('-') {
            return;
        }
        c.check(hclassno(c.arg(0)));
    });
}

#[test]
fn numtoperm_oracle() {
    each("combinat", "numtoperm", |c| {
        // Skip numtoperm(5, -1) and numtoperm(5, 100000000000000000000):
        // the native rank is unsigned and limited to u64.
        let Ok(rank) = c.args[1].parse::<u64>() else {
            return;
        };
        let value = numtoperm(c.arg(0), rank)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        c.check(Some(value));
    });
}
