//! Compatibility tests for the factor family against the MPU 0.75 fixtures.
mod common;

// The u64 API skips signed arguments (including -0) and the bigint cases
// 18450931744652885087, 36803912137803496523, and 36832540935240707603.
use common::each;
use math_prime_util::{
    fermat_factor, holf_factor, is_carmichael, is_cyclic, is_perfect_number, is_quasi_carmichael,
    lehman_factor, squfof_factor,
};

fn check(name: &str, method: impl Fn(u64, Option<u64>) -> Vec<u64>) {
    let mut count = 0;
    for case in common::load("factor")
        .into_iter()
        .filter(|c| c.func == name)
    {
        // Skip signed arguments and bigint arguments above u64::MAX.
        let Ok(n) = case.args[0].parse::<u64>() else {
            continue;
        };
        let rounds = case.args.get(1).map(|r| r.parse::<u64>().unwrap());
        let got = method(n, rounds);
        if got.is_empty() {
            case.check::<String>(None);
        } else {
            case.check(Some(
                got.iter().map(u64::to_string).collect::<Vec<_>>().join(" "),
            ));
        }
        count += 1;
    }
    assert!(count > 0);
}

#[test]
fn fermat_factor_oracle() {
    check("fermat_factor", fermat_factor);
}
#[test]
fn holf_factor_oracle() {
    check("holf_factor", holf_factor);
}
#[test]
fn lehman_factor_oracle() {
    check("lehman_factor", |n, trial| {
        lehman_factor(n, trial.map(|t| t != 0))
    });
}
#[test]
fn squfof_factor_oracle() {
    check("squfof_factor", squfof_factor);
}

#[test]
fn is_carmichael_oracle() {
    each("factor", "is_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_carmichael(n))));
        }
    });
}

#[test]
fn is_quasi_carmichael_oracle() {
    each("factor", "is_quasi_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(is_quasi_carmichael(n)));
        }
    });
}

#[test]
fn is_cyclic_oracle() {
    each("factor", "is_cyclic", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_cyclic(n))));
        }
    });
}

#[test]
fn is_perfect_number_oracle() {
    each("factor", "is_perfect_number", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_perfect_number(n))));
        }
    });
}
