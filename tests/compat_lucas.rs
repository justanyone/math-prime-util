//! Compat tests against MPU for `lucas_seq.c`.
//! Fixture: `tests/fixtures/lucas.tsv`.
//!
//! Skipped cases: negative `k` for `fibonacci` and `lucas_number` (the Rust
//! functions take `u64`, and MPU returns `undef` for them). `lucasuv` cases
//! where MPU gives a value in `U` but `undef` in `V`: the Rust function returns
//! `None` for the whole pair when one value does not fit `i64`.

mod common;

use common::each;
use math_prime_util::*;

/// Formats a pair as the fixture does: `"U V"`.
fn pair<T: std::fmt::Display>(p: (T, T)) -> String {
    format!("{} {}", p.0, p.1)
}

#[test]
fn lucas_sequence_oracle() {
    each("lucas", "lucas_sequence", |c| {
        if c.error {
            assert!(std::panic::catch_unwind(|| lucas_sequence(c.arg(0), c.arg(1), c.arg(2), c.arg(3))).is_err());
            return;
        }
        let (u, v, qk) = lucas_sequence(c.arg(0), c.arg(1), c.arg(2), c.arg(3));
        c.check(Some(format!("{u} {v} {qk}")));
    });
}

#[test]
fn lucasuvmod_oracle() {
    each("lucas", "lucasuvmod", |c| {
        if c.error {
            assert!(std::panic::catch_unwind(|| lucasuvmod(c.arg(0), c.arg(1), c.arg(2), c.arg(3))).is_err());
            return;
        }
        c.check(Some(pair(lucasuvmod(c.arg(0), c.arg(1), c.arg(2), c.arg(3)))));
    });
}

#[test]
fn lucasumod_oracle() {
    each("lucas", "lucasumod", |c| {
        if c.error {
            assert!(std::panic::catch_unwind(|| lucasumod(c.arg(0), c.arg(1), c.arg(2), c.arg(3))).is_err());
            return;
        }
        c.check(Some(lucasumod(c.arg(0), c.arg(1), c.arg(2), c.arg(3))));
    });
}

#[test]
fn lucasvmod_oracle() {
    each("lucas", "lucasvmod", |c| {
        if c.error {
            assert!(std::panic::catch_unwind(|| lucasvmod(c.arg(0), c.arg(1), c.arg(2), c.arg(3))).is_err());
            return;
        }
        c.check(Some(lucasvmod(c.arg(0), c.arg(1), c.arg(2), c.arg(3))));
    });
}

#[test]
fn lucasuv_oracle() {
    each("lucas", "lucasuv", |c| {
        // Skipped: MPU gives a U beyond i64 with V undef; the Rust pair is None.
        if c.result.as_deref().is_some_and(|r| r.contains("undef")) {
            return;
        }
        c.check(lucasuv(c.arg(0), c.arg(1), c.arg(2)).map(pair));
    });
}

#[test]
fn lucasu_oracle() {
    each("lucas", "lucasu", |c| {
        c.check(lucasu(c.arg(0), c.arg(1), c.arg(2)));
    });
}

#[test]
fn lucasv_oracle() {
    each("lucas", "lucasv", |c| {
        c.check(lucasv(c.arg(0), c.arg(1), c.arg(2)));
    });
}

#[test]
fn fibonacci_oracle() {
    each("lucas", "fibonacci", |c| {
        // Skipped: negative k; the Rust argument is u64.
        if c.args[0].starts_with('-') {
            return;
        }
        c.check(fibonacci(c.arg(0)));
    });
}

#[test]
fn lucas_number_oracle() {
    each("lucas", "lucas_number", |c| {
        // Skipped: negative k; the Rust argument is u64.
        if c.args[0].starts_with('-') {
            return;
        }
        c.check(lucas_number(c.arg(0)));
    });
}
