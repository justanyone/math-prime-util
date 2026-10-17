//! Compatibility tests for the four factor predicates from MPU 0.75.
mod common;

use common::each;
use math_prime_util::{is_carmichael, is_cyclic, is_perfect_number, is_quasi_carmichael};

#[test]
fn is_carmichael_oracle() {
    each("factor", "is_carmichael", |c| {
        c.check(Some(u8::from(is_carmichael(c.arg(0)))));
    });
}

#[test]
fn is_quasi_carmichael_oracle() {
    each("factor", "is_quasi_carmichael", |c| {
        c.check(Some(is_quasi_carmichael(c.arg(0))));
    });
}

#[test]
fn is_cyclic_oracle() {
    each("factor", "is_cyclic", |c| {
        c.check(Some(u8::from(is_cyclic(c.arg(0)))));
    });
}

#[test]
fn is_perfect_number_oracle() {
    each("factor", "is_perfect_number", |c| {
        c.check(Some(u8::from(is_perfect_number(c.arg(0)))));
    });
}
