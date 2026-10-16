//! Compatibility with the committed MPU 0.75 Lucas oracle.
mod common;
use common::{each, each_may_panic};
use math_prime_util::*;

fn component(value: Option<i128>) -> String {
    value.map_or_else(|| "undef".to_owned(), |v| v.to_string())
}

#[test]
fn lucasu_oracle() {
    each("lucas", "lucasu", |c| {
        c.check(lucasu(c.arg(0), c.arg(1), c.arg(2)))
    });
}
#[test]
fn lucasv_oracle() {
    each("lucas", "lucasv", |c| {
        c.check(lucasv(c.arg(0), c.arg(1), c.arg(2)))
    });
}
#[test]
fn lucasuv_oracle() {
    each("lucas", "lucasuv", |c| {
        let (u, v) = lucasuv(c.arg(0), c.arg(1), c.arg(2));
        c.check(Some(format!("{} {}", component(u), component(v))));
    });
}
#[test]
fn lucasumod_oracle() {
    each("lucas", "lucasumod", |c| {
        c.check(lucasumod(c.arg(0), c.arg(1), c.arg(2), c.arg(3)))
    });
}
#[test]
fn lucasvmod_oracle() {
    each("lucas", "lucasvmod", |c| {
        c.check(lucasvmod(c.arg(0), c.arg(1), c.arg(2), c.arg(3)))
    });
}
#[test]
fn lucasuvmod_oracle() {
    each("lucas", "lucasuvmod", |c| {
        c.check(
            lucasuvmod(c.arg(0), c.arg(1), c.arg(2), c.arg(3)).map(|(u, v)| format!("{u} {v}")),
        );
    });
}
#[test]
fn lucas_sequence_oracle() {
    each_may_panic("lucas", "lucas_sequence", |c| {
        let (u, v, q) = lucas_sequence(c.arg(0), c.arg(1), c.arg(2), c.arg(3));
        Some(format!("{u} {v} {q}"))
    });
}
#[test]
fn fibonacci_oracle() {
    each("lucas", "fibonacci", |c| c.check(fibonacci(c.arg(0))));
}
#[test]
fn lucas_number_oracle() {
    each("lucas", "lucas_number", |c| c.check(lucas_number(c.arg(0))));
}
