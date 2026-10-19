//! The arbitrary precision factoring and divisor functions against MPU 0.75
//! (pure Perl) on 65 to 200 bit inputs: `tests/fixtures/bigint_factor.tsv`.
#![cfg(feature = "bigint")]
mod common;
use common::Case;
use math_prime_util::bigint::divisors::*;
use math_prime_util::bigint::factor::*;
use math_prime_util::bigint::primality::is_prob_prime;
use num_bigint::BigUint;

const FAMILY: &str = "bigint_factor";

fn unsigned(c: &Case, i: usize) -> BigUint {
    c.arg(i)
}

fn opt(c: &Case, i: usize) -> Option<u64> {
    (i < c.args.len()).then(|| c.arg(i))
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

fn product(values: &[BigUint]) -> BigUint {
    values.iter().fold(BigUint::from(1_u8), |acc, v| acc * v)
}

#[test]
fn factor_oracle() {
    common::each(FAMILY, "factor", |c| {
        let n = unsigned(c, 0);
        let got = factor(&n).unwrap_or_else(|e| panic!("fixture line {}: unsplit {e}", c.line));
        c.check(list(&got));
        // Acceptance: the factors multiply back and every one is a probable prime.
        assert_eq!(
            product(&got),
            if n.bits() == 0 {
                BigUint::from(0_u8)
            } else {
                n.clone()
            }
        );
        if n.bits() != 0 {
            assert!(
                got.iter().all(|p| is_prob_prime(p) != 0),
                "fixture line {}",
                c.line
            );
        }
    });
    common::each(FAMILY, "factor_exp", |c| {
        let pairs = factor_exp(&unsigned(c, 0)).unwrap();
        c.check((!pairs.is_empty()).then(|| {
            pairs
                .iter()
                .map(|(p, e)| format!("{p}^{e}"))
                .collect::<Vec<_>>()
                .join(" ")
        }));
    });
}

#[test]
fn totient_oracle() {
    common::each(FAMILY, "euler_phi", |c| {
        c.check(Some(euler_phi(&unsigned(c, 0)).unwrap()));
    });
    common::each(FAMILY, "carmichael_lambda", |c| {
        c.check(Some(carmichael_lambda(&unsigned(c, 0)).unwrap()));
    });
    common::each(FAMILY, "moebius", |c| {
        c.check(Some(moebius(&unsigned(c, 0)).unwrap()));
    });
    common::each(FAMILY, "is_square_free", |c| {
        c.check(Some(u8::from(is_square_free(&unsigned(c, 0)).unwrap())));
    });
    common::each(FAMILY, "prime_bigomega", |c| {
        c.check(Some(prime_bigomega(&unsigned(c, 0)).unwrap()));
    });
    common::each(FAMILY, "prime_omega", |c| {
        c.check(Some(prime_omega(&unsigned(c, 0)).unwrap()));
    });
}

#[test]
fn divisor_oracle() {
    common::each(FAMILY, "divisor_sum", |c| {
        c.check(Some(divisor_sum(&unsigned(c, 0), c.arg(1)).unwrap()));
    });
    common::each(FAMILY, "divisors", |c| {
        let max = (c.args.len() > 1).then(|| unsigned(c, 1));
        c.check(list(&divisors(&unsigned(c, 0), max.as_ref()).unwrap()));
    });
}

#[test]
fn method_oracle() {
    common::each(FAMILY, "trial_factor", |c| {
        c.check(list(&trial_factor(&unsigned(c, 0), opt(c, 1))));
    });
    common::each(FAMILY, "prho_factor", |c| {
        c.check(list(&prho_factor(&unsigned(c, 0), opt(c, 1))));
    });
    common::each(FAMILY, "pbrent_factor", |c| {
        c.check(list(&pbrent_factor(&unsigned(c, 0), opt(c, 1), opt(c, 2))));
    });
    common::each(FAMILY, "pminus1_factor", |c| {
        c.check(list(&pminus1_factor(&unsigned(c, 0), opt(c, 1), opt(c, 2))));
    });
    common::each(FAMILY, "holf_factor", |c| {
        c.check(list(&holf_factor(&unsigned(c, 0), opt(c, 1), opt(c, 2))));
    });
    common::each(FAMILY, "ecm_factor", |c| {
        c.check(list(&ecm_factor(
            &unsigned(c, 0),
            opt(c, 1),
            opt(c, 2),
            opt(c, 3),
        )));
    });
}

/// MPU's pure Perl `pplus1_factor` and `squfof_factor` are Pollard-Brent
/// stubs with the bound as the round count (docs/upstream-findings.md,
/// API differences). The port runs the C methods: the rows are checked as
/// valid splits, and equal to the oracle where both sides split.
fn stub_method(func: &str, run: impl Fn(&Case, &BigUint) -> Vec<BigUint>) {
    let mut agreed = 0;
    common::each(FAMILY, func, |c| {
        let n = unsigned(c, 0);
        let got = run(c, &n);
        assert_eq!(product(&got), n, "fixture line {}", c.line);
        assert!(
            got.iter().all(|f| *f > BigUint::from(1_u8)),
            "fixture line {}",
            c.line
        );
        let oracle_split = c.result.as_deref().is_some_and(|r| r.contains(' '));
        if oracle_split && got.len() > 1 {
            c.check(list(&got));
            agreed += 1;
        }
    });
    assert!(agreed > 0, "{func}: no row where both sides split");
}

#[test]
fn stub_method_oracle() {
    stub_method("pplus1_factor", |c, n| pplus1_factor(n, opt(c, 1)));
    stub_method("squfof_factor", |c, n| squfof_factor(n, opt(c, 1)));
}

#[test]
fn every_row_is_replayed() {
    let handled = [
        "factor",
        "factor_exp",
        "euler_phi",
        "carmichael_lambda",
        "moebius",
        "is_square_free",
        "prime_bigomega",
        "prime_omega",
        "divisor_sum",
        "divisors",
        "trial_factor",
        "prho_factor",
        "pbrent_factor",
        "pminus1_factor",
        "pplus1_factor",
        "holf_factor",
        "squfof_factor",
        "ecm_factor",
    ];
    let cases = common::load(FAMILY);
    for c in &cases {
        assert!(
            handled.contains(&c.func.as_str()),
            "fixture line {}: {}",
            c.line,
            c.func
        );
        assert!(!c.error, "fixture line {}: oracle died", c.line);
    }
    assert!(cases.len() > 700, "{} rows", cases.len());
    // Inputs reach 200 bits and the factor rows include products the
    // recipe must split (no factor below 5000 and not a prime power).
    let factored: Vec<BigUint> = cases
        .iter()
        .filter(|c| c.func == "factor")
        .map(|c| unsigned(c, 0))
        .collect();
    assert!(factored.iter().any(|n| n.bits() >= 200), "no 200-bit input");
    let split_rows = cases
        .iter()
        .filter(|c| c.func == "factor")
        .filter(|c| {
            let n = unsigned(c, 0);
            let parts: Vec<BigUint> = c
                .result
                .as_deref()
                .unwrap_or("")
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            n.bits() > 128
                && parts.len() >= 2
                && parts.iter().all(|p| *p > BigUint::from(5000_u16))
                && parts.windows(2).any(|w| w[0] != w[1])
        })
        .count();
    assert!(split_rows >= 10, "{split_rows} rows need the recipe");
}
