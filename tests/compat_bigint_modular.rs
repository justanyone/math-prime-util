//! The arbitrary precision modular functions against MPU 0.75 (pure Perl)
//! on 65 to 500 bit inputs: `tests/fixtures/bigint_modular.tsv`.
#![cfg(feature = "bigint")]
mod common;
use common::Case;
use math_prime_util::bigint::arith::{binomial, powmod};
use math_prime_util::bigint::modular::*;
use math_prime_util::bigint::primality::is_prime;
use num_bigint::{BigInt, BigUint};

const FAMILY: &str = "bigint_modular";

fn unsigned(c: &Case, i: usize) -> BigUint {
    c.arg(i)
}

fn signed(c: &Case, i: usize) -> BigInt {
    c.arg(i)
}

fn flag(value: bool) -> u8 {
    u8::from(value)
}

fn joined(values: &[BigUint]) -> Option<String> {
    (!values.is_empty()).then(|| {
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    })
}

/// Fixture lines where pure Perl `allsqrtmod` lists wrong roots and so
/// `rootmod(a, 2, n)` is `undef` although a root exists: n has the factor
/// 2^65 and a = 4 mod 8, which leaves a root modulo 2^63 to find, where PP
/// overflows a native product. docs/upstream-findings.md entry 19.
const PP_ALLSQRTMOD_WRONG: [usize; 2] = [295, 439];
const PP_ROOTMOD_UNDEF: [usize; 2] = [294, 438];

#[test]
fn sqrtmod_oracle() {
    let mut other_root = 0;
    common::each(FAMILY, "sqrtmod", |c| {
        let (a, n) = (unsigned(c, 0), unsigned(c, 1));
        let got = sqrtmod(&a, &n);
        let oracle = c.result.as_ref().map(|r| r.parse::<BigUint>().unwrap());
        if let (Some(r), Some(other)) = (&got, &oracle) {
            if r != other {
                // MPU documents that over a composite modulus the root need
                // not be the smallest: pure Perl combines the roots modulo
                // each prime power as Hensel lifting leaves them, the port
                // takes the smaller root modulo each prime power as the
                // native code does. Both square back; a prime modulus is
                // checked exactly.
                assert!(is_prime(&n) == 0, "fixture line {}: prime modulus", c.line);
                assert_eq!(
                    powmod(r, &BigUint::from(2_u8), &n),
                    Some(&a % &n),
                    "fixture line {}",
                    c.line
                );
                other_root += 1;
                return;
            }
        }
        c.check(got);
    });
    // 15 of the 153 rows in the current fixture.
    assert!(other_root <= 20, "{other_root} other square roots");
    let mut wrong = 0;
    common::each(FAMILY, "allsqrtmod", |c| {
        let (a, n) = (unsigned(c, 0), unsigned(c, 1));
        let got = allsqrtmod(&a, &n);
        if PP_ALLSQRTMOD_WRONG.contains(&c.line) {
            // The oracle has the right count, but some of its roots do not
            // square back; the port's roots are distinct, below n, square
            // back, and include every valid oracle root.
            let oracle: Vec<BigUint> = c
                .result
                .as_deref()
                .unwrap()
                .split(' ')
                .map(|r| r.parse().unwrap())
                .collect();
            let valid =
                |r: &BigUint| r < &n && powmod(r, &BigUint::from(2_u8), &n) == Some(&a % &n);
            assert!(
                !oracle.iter().all(valid),
                "fixture line {}: oracle now right",
                c.line
            );
            assert_eq!(got.len(), oracle.len(), "fixture line {}", c.line);
            assert!(
                got.windows(2).all(|w| w[0] < w[1]),
                "fixture line {}",
                c.line
            );
            assert!(got.iter().all(valid), "fixture line {}", c.line);
            assert!(
                oracle.iter().filter(|r| valid(r)).all(|r| got.contains(r)),
                "fixture line {}",
                c.line
            );
            wrong += 1;
            return;
        }
        c.check(joined(&got));
    });
    assert_eq!(wrong, PP_ALLSQRTMOD_WRONG.len());
}

#[test]
fn rootmod_oracle() {
    let mut other_root = 0;
    let mut perl_undef = 0;
    common::each(FAMILY, "rootmod", |c| {
        let (a, k, n) = (unsigned(c, 0), unsigned(c, 1), unsigned(c, 2));
        let got = rootmod(&a, &k, &n);
        if PP_ROOTMOD_UNDEF.contains(&c.line) {
            // Pure Perl takes the first root of its wrong `allsqrtmod` list
            // (see `PP_ALLSQRTMOD_WRONG`), which does not square back.
            assert!(
                c.result.is_none(),
                "fixture line {}: oracle now defined",
                c.line
            );
            let r = got.unwrap_or_else(|| panic!("fixture line {}: no root", c.line));
            assert_eq!(powmod(&r, &k, &n), Some(&a % &n), "fixture line {}", c.line);
            perl_undef += 1;
            return;
        }
        let oracle = c.result.as_ref().map(|r| r.parse::<BigUint>().unwrap());
        if let (Some(r), Some(other)) = (&got, &oracle) {
            if r != other {
                // Over a composite modulus pure Perl and the native port can
                // pick different roots: PP takes the smallest root modulo a
                // prime power (`allrootmod`) where the port lifts the root
                // modulo p by Newton's step, and the port's `p = 9 mod 16`
                // square root branch can give the other sign. Both are k-th
                // roots; MPU documents that the root need not be the smallest.
                assert!(is_prime(&n) == 0, "fixture line {}: prime modulus", c.line);
                assert_eq!(powmod(r, &k, &n), Some(&a % &n), "fixture line {}", c.line);
                other_root += 1;
                return;
            }
        }
        c.check(got);
    });
    // 9 of the 253 rows in the current fixture.
    assert!(other_root <= 15, "{other_root} other roots");
    assert_eq!(perl_undef, PP_ROOTMOD_UNDEF.len());
}

#[test]
fn group_oracle() {
    common::each(FAMILY, "znorder", |c| {
        c.check(znorder(&unsigned(c, 0), &unsigned(c, 1)));
    });
    common::each(FAMILY, "znprimroot", |c| {
        c.check(znprimroot(&unsigned(c, 0)));
    });
    common::each(FAMILY, "is_primitive_root", |c| {
        c.check(is_primitive_root(&unsigned(c, 0), &unsigned(c, 1)).map(flag));
    });
    common::each(FAMILY, "znlog", |c| {
        c.check(znlog(&unsigned(c, 0), &unsigned(c, 1), &unsigned(c, 2)));
    });
}

#[test]
fn chinese_oracle() {
    common::each(FAMILY, "chinese", |c| {
        let pairs: Vec<(BigUint, BigUint)> = (0..c.args.len() / 2)
            .map(|i| (unsigned(c, 2 * i), unsigned(c, 2 * i + 1)))
            .collect();
        c.check(chinese(&pairs));
    });
}

#[test]
fn factorial_binomial_oracle() {
    common::each(FAMILY, "factorialmod", |c| {
        c.check(factorialmod(&unsigned(c, 0), &unsigned(c, 1)));
    });
    let mut perl_died = 0;
    common::each(FAMILY, "binomialmod", |c| {
        let (n, k, m) = (unsigned(c, 0), unsigned(c, 1), unsigned(c, 2));
        let got = binomialmod(&n, &k, &m);
        if c.error {
            // docs/upstream-findings.md entry 8 (confirmed): pure Perl
            // `binomial` with a Math::BigInt n and k >= 100 dies inside
            // Math::BigInt::bnok, so `binomialmod` dies too; the exact
            // binomial reduced mod m is the oracle here.
            let exact = binomial(&n, &k).unwrap() % &m;
            assert_eq!(got, Some(exact), "fixture line {}", c.line);
            perl_died += 1;
            return;
        }
        c.check(got);
    });
    assert!(perl_died >= 30, "{perl_died} rows where Perl died");
}

#[test]
fn lucas_oracle() {
    common::each_may_panic(FAMILY, "lucasumod", |c| {
        Some(lucasumod(
            &signed(c, 0),
            &signed(c, 1),
            &unsigned(c, 2),
            &unsigned(c, 3),
        ))
    });
    common::each_may_panic(FAMILY, "lucasvmod", |c| {
        Some(lucasvmod(
            &signed(c, 0),
            &signed(c, 1),
            &unsigned(c, 2),
            &unsigned(c, 3),
        ))
    });
}

#[test]
fn carmichael_oracle() {
    common::each(FAMILY, "is_carmichael", |c| {
        c.check(Some(flag(is_carmichael(&unsigned(c, 0)))));
    });
    common::each(FAMILY, "is_quasi_carmichael", |c| {
        c.check(Some(is_quasi_carmichael(&unsigned(c, 0))));
    });
}

#[test]
fn every_row_is_replayed() {
    let handled = [
        "sqrtmod",
        "rootmod",
        "allsqrtmod",
        "znorder",
        "znprimroot",
        "is_primitive_root",
        "znlog",
        "chinese",
        "factorialmod",
        "binomialmod",
        "lucasumod",
        "lucasvmod",
        "is_carmichael",
        "is_quasi_carmichael",
    ];
    let cases = common::load(FAMILY);
    for c in &cases {
        assert!(
            handled.contains(&c.func.as_str()),
            "fixture line {}: {}",
            c.line,
            c.func
        );
    }
    assert!(cases.len() > 1000, "{} rows", cases.len());
    // Every function has a defined result on an input of 500 bits or more.
    let widest = |func: &str, arg: usize| {
        cases
            .iter()
            .filter(|c| c.func == func && c.result.is_some() && c.args.len() > arg)
            .map(|c| unsigned(c, arg).bits())
            .max()
            .unwrap_or(0)
    };
    for func in [
        "sqrtmod",
        "rootmod",
        "allsqrtmod",
        "factorialmod",
        "lucasumod",
        "lucasvmod",
        "is_carmichael",
        "is_quasi_carmichael",
        "znlog",
    ] {
        let arg = match func {
            "sqrtmod" | "allsqrtmod" | "factorialmod" => 1,
            "rootmod" | "znlog" => 2,
            "lucasumod" | "lucasvmod" => 3,
            _ => 0,
        };
        assert!(widest(func, arg) >= 500, "{func} stays below 500 bits");
    }
    assert!(widest("znorder", 1) >= 500);
    assert!(widest("znprimroot", 0) >= 500);
    assert!(widest("is_primitive_root", 1) >= 500);
    assert!(widest("binomialmod", 2) >= 500);
    assert!(widest("chinese", 5) >= 500);
    let rows = |func: &str| cases.iter().filter(|c| c.func == func).count();
    assert!(rows("znlog") >= 50 && rows("rootmod") >= 100 && rows("binomialmod") >= 200);
    // Carmichael numbers above the 50-digit line where MPU stops factoring.
    let big_carmichael = cases
        .iter()
        .filter(|c| {
            c.func == "is_carmichael"
                && c.result.as_deref() == Some("1")
                && unsigned(c, 0).to_string().len() > 50
        })
        .count();
    assert!(
        big_carmichael >= 3,
        "{big_carmichael} large Carmichael numbers"
    );
}
