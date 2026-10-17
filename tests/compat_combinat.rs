//! Compat tests for the combinatorics group. Fixture: `tests/fixtures/combinat.tsv`.
//!
//! Skipped cases: `is_polygonal` with
//! `n = 510622052816898545467859772308206986101878`
//! (8 rows; `k` is 3, 4, 5, 6, 8, 12, 100, or 4294967297).
//! That `n` does not fit in `u128`, so the Rust signature cannot take it.
//!
//! An oracle integer wider than the return type is `None`. That covers
//! `factorial` above 34!, and the same cutoff for `subfactorial`,
//! `multifactorial`, `bell_number`, `fubini`, `catalan_number`, `powersum`,
//! `partitions`, `partitionsq`, `primorial`, `pn_primorial`,
//! `consecutive_integer_lcm`, `stirling`, `falling_factorial`,
//! `rising_factorial`, `binomial`, `bernfrac`, `harmfrac`, and `permtonum`.

mod common;

use std::panic::{AssertUnwindSafe, catch_unwind};

use common::{Case, load};
use math_prime_util::*;

fn rows(func: &str) -> Vec<Case> {
    let v: Vec<Case> = load("combinat").into_iter().filter(|c| c.func == func).collect();
    assert!(!v.is_empty(), "no fixture cases for {func}");
    v
}

/// `n` of the named `is_polygonal` rows does not fit in `i128`.
fn skip_case(c: &Case) -> bool {
    c.func == "is_polygonal" && c.args.first().is_some_and(|s| s.parse::<i128>().is_err())
}

#[derive(Clone, Copy)]
enum Wide {
    U128,
    I128,
}

fn fits(exp: &str, wide: Wide) -> bool {
    match wide {
        Wide::U128 => exp.parse::<u128>().is_ok(),
        Wide::I128 => exp.parse::<i128>().is_ok(),
    }
}

fn check_num(c: &Case, got: Option<String>, wide: Wide) {
    match (&c.result, got) {
        (None, None) => {}
        (Some(exp), Some(g)) => {
            assert_eq!(g, *exp, "line {} {} {:?}", c.line, c.func, c.args);
        }
        (Some(exp), None) => {
            assert!(
                !fits(exp, wide),
                "line {}: returned None but oracle {exp} fits",
                c.line
            );
        }
        (None, Some(g)) => panic!("line {}: oracle undef, got {g}", c.line),
    }
}

fn drive(func: &str, mut body: impl FnMut(&Case) -> Option<String>, wide: Wide) {
    let mut saw_skip = false;
    for c in rows(func) {
        if c.error {
            let r = catch_unwind(AssertUnwindSafe(|| {
                let _ = body(&c);
            }));
            assert!(r.is_err(), "line {}: expected a panic", c.line);
        } else if skip_case(&c) {
            saw_skip = true;
        } else {
            check_num(&c, body(&c), wide);
        }
    }
    if func == "is_polygonal" {
        assert!(saw_skip, "the 42-digit is_polygonal rows were not skipped");
    }
}

fn drive_exact(func: &str, mut body: impl FnMut(&Case) -> Option<String>) {
    for c in rows(func) {
        if c.error {
            let r = catch_unwind(AssertUnwindSafe(|| {
                let _ = body(&c);
            }));
            assert!(r.is_err(), "line {}: expected a panic", c.line);
        } else if skip_case(&c) {
            // Named in the module comment.
        } else {
            c.check(body(&c));
        }
    }
}

fn show_opt<T: ToString>(v: Option<T>) -> Option<String> {
    v.map(|x| x.to_string())
}

#[test]
fn binomial_oracle() {
    drive(
        "binomial",
        |c| show_opt(binomial(c.arg(0), c.arg(1))),
        Wide::I128,
    );
}

#[test]
fn catalan_number_oracle() {
    drive(
        "catalan_number",
        |c| show_opt(catalan_number(c.arg(0))),
        Wide::U128,
    );
}

#[test]
fn stirling_oracle() {
    drive(
        "stirling",
        |c| {
            let typ: u32 = if c.args.len() >= 3 { c.arg(2) } else { 1 };
            show_opt(stirling(c.arg(0), c.arg(1), typ))
        },
        Wide::I128,
    );
}

#[test]
fn bell_number_oracle() {
    drive(
        "bell_number",
        |c| show_opt(bell_number(c.arg(0))),
        Wide::U128,
    );
}

#[test]
fn fubini_oracle() {
    drive("fubini", |c| show_opt(fubini(c.arg(0))), Wide::U128);
}

#[test]
fn factorial_oracle() {
    drive(
        "factorial",
        |c| show_opt(factorial(c.arg(0))),
        Wide::U128,
    );
}

#[test]
fn multifactorial_oracle() {
    drive(
        "multifactorial",
        |c| show_opt(multifactorial(c.arg(0), c.arg(1))),
        Wide::U128,
    );
}

#[test]
fn subfactorial_oracle() {
    drive(
        "subfactorial",
        |c| show_opt(subfactorial(c.arg(0))),
        Wide::U128,
    );
}

#[test]
fn falling_factorial_oracle() {
    drive(
        "falling_factorial",
        |c| show_opt(falling_factorial(c.arg(0), c.arg(1))),
        Wide::I128,
    );
}

#[test]
fn rising_factorial_oracle() {
    drive(
        "rising_factorial",
        |c| show_opt(rising_factorial(c.arg(0), c.arg(1))),
        Wide::I128,
    );
}

#[test]
fn powersum_oracle() {
    drive(
        "powersum",
        |c| show_opt(powersum(c.arg(0), c.arg(1))),
        Wide::U128,
    );
}

#[test]
fn integer_complexity_oracle() {
    drive_exact("integer_complexity", |c| show_opt(integer_complexity(c.arg(0))));
}

#[test]
fn bernfrac_oracle() {
    for c in rows("bernfrac") {
        let got = bernfrac(c.arg(0));
        match (&c.result, got) {
            (None, None) => {}
            (Some(exp), Some((n, d))) => {
                assert_eq!(format!("{n} {d}"), *exp, "line {}", c.line);
            }
            (Some(exp), None) => {
                let mut parts = exp.split_whitespace();
                let a = parts.next().unwrap();
                let b = parts.next().unwrap();
                let ok = a.parse::<i128>().is_ok() && b.parse::<u128>().is_ok();
                assert!(!ok, "line {}: None but {exp} fits", c.line);
            }
            (None, Some(v)) => panic!("line {}: undef vs {v:?}", c.line),
        }
    }
}

#[test]
fn harmfrac_oracle() {
    for c in rows("harmfrac") {
        let got = harmfrac(c.arg(0));
        match (&c.result, got) {
            (Some(exp), Some((n, d))) => {
                assert_eq!(format!("{n} {d}"), *exp, "line {}", c.line);
            }
            (Some(exp), None) => {
                let mut parts = exp.split_whitespace();
                let a = parts.next().unwrap();
                let b = parts.next().unwrap();
                let ok = a.parse::<i128>().is_ok() && b.parse::<u128>().is_ok();
                assert!(!ok, "line {}: None but {exp} fits", c.line);
            }
            other => panic!("line {}: {other:?} does not match", c.line),
        }
    }
}

#[test]
fn primorial_oracle() {
    drive("primorial", |c| show_opt(primorial(c.arg(0))), Wide::U128);
}

#[test]
fn pn_primorial_oracle() {
    drive(
        "pn_primorial",
        |c| show_opt(pn_primorial(c.arg(0))),
        Wide::U128,
    );
}

#[test]
fn consecutive_integer_lcm_oracle() {
    drive(
        "consecutive_integer_lcm",
        |c| show_opt(consecutive_integer_lcm(c.arg(0))),
        Wide::U128,
    );
}

#[test]
fn frobenius_number_oracle() {
    drive(
        "frobenius_number",
        |c| {
            let vals: Vec<u64> = c.args.iter().map(|s| s.parse().unwrap()).collect();
            show_opt(frobenius_number(&vals))
        },
        Wide::I128,
    );
}

#[test]
fn is_congruent_number_oracle() {
    drive_exact("is_congruent_number", |c| {
        Some(u8::from(is_congruent_number(c.arg(0))).to_string())
    });
}

#[test]
fn is_polygonal_oracle() {
    drive_exact("is_polygonal", |c| {
        Some(u8::from(is_polygonal(c.arg(0), c.arg(1))).to_string())
    });
}

#[test]
fn is_pillai_oracle() {
    drive_exact("is_pillai", |c| Some(is_pillai(c.arg(0)).to_string()));
}

#[test]
fn is_happy_oracle() {
    drive_exact("is_happy", |c| {
        let base: u64 = if c.args.len() >= 2 { c.arg(1) } else { 10 };
        let exp: u32 = if c.args.len() >= 3 { c.arg(2) } else { 2 };
        Some(is_happy(c.arg(0), base, exp).to_string())
    });
}

#[test]
fn hclassno_oracle() {
    drive_exact("hclassno", |c| Some(hclassno(c.arg(0)).to_string()));
}

#[test]
fn numtoperm_oracle() {
    drive_exact("numtoperm", |c| {
        let v = numtoperm(c.arg(0), c.arg(1));
        Some(v.iter().map(ToString::to_string).collect::<Vec<_>>().join(" "))
    });
}

#[test]
fn permtonum_oracle() {
    drive(
        "permtonum",
        |c| {
            let p: Vec<u64> = c.args.iter().map(|s| s.parse().unwrap()).collect();
            show_opt(permtonum(&p))
        },
        Wide::U128,
    );
}

#[test]
fn partitions_oracle() {
    drive("partitions", |c| show_opt(partitions(c.arg(0))), Wide::U128);
}

#[test]
fn partitionsq_oracle() {
    drive(
        "partitionsq",
        |c| show_opt(partitionsq(c.arg(0))),
        Wide::U128,
    );
}

/// Calling `integer_complexity(0)` must not wipe a filled cache.
#[test]
fn integer_complexity_zero_keeps_cache() {
    let first = integer_complexity(100);
    assert_eq!(integer_complexity(0), None);
    assert_eq!(integer_complexity(100), first);
    let wider = integer_complexity(5_000);
    assert_eq!(integer_complexity(100), first);
    assert_eq!(integer_complexity(5_000), wider);
}

/// Two threads may fill the complexity cache at the same time.
#[test]
fn integer_complexity_concurrent() {
    let mut handles = Vec::new();
    for k in 0..4u64 {
        handles.push(std::thread::spawn(move || {
            let a = integer_complexity(2_000 + k);
            let b = integer_complexity(2_000 + k);
            assert_eq!(a, b);
            a
        }));
    }
    for h in handles {
        assert!(h.join().unwrap().is_some());
    }
}

/// A second, larger request must agree with the values already cached.
#[test]
fn integer_complexity_second_launch() {
    let small = integer_complexity(30);
    let big = integer_complexity(8_000);
    assert_eq!(integer_complexity(30), small);
    assert_eq!(integer_complexity(8_000), big);
    assert_eq!(integer_complexity(10), Some(7));
}
