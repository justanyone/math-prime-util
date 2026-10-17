//! MPU factor fixtures. The native API takes unsigned 64-bit integers.
mod common;

use common::{Case, each};
use math_prime_util::{divisors, factor, factor_exp, trial_factor};

fn native(c: &Case) -> bool {
    // Skip signed inputs, values above u64::MAX, and invalid text such as
    // trial_factor(18446744073709551616, "10foo"). These need a string API.
    c.args.iter().all(|a| a.parse::<u64>().is_ok())
}

fn check_list(c: &Case, values: &[u64]) {
    c.check((!values.is_empty()).then(|| {
        values
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    }));
}

#[test]
fn factor_oracle() {
    each("factor", "factor", |c| {
        if native(c) {
            check_list(c, &factor(c.arg(0)));
        }
    });
}

#[test]
fn factor_exp_oracle() {
    each("factor", "factor_exp", |c| {
        if native(c) {
            let pairs = factor_exp(c.arg(0));
            c.check((!pairs.is_empty()).then(|| {
                pairs
                    .iter()
                    .map(|(p, e)| format!("{p}^{e}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            }));
        }
    });
}

#[test]
fn divisors_oracle() {
    each("factor", "divisors", |c| {
        if native(c) {
            let max = c.args.get(1).map(|_| c.arg(1));
            check_list(c, &divisors(c.arg(0), max));
        }
    });
}

#[test]
fn trial_factor_oracle() {
    each("factor", "trial_factor", |c| {
        if native(c) {
            let limit = c.args.get(1).map_or(0, |_| c.arg(1));
            check_list(c, &trial_factor(c.arg(0), limit));
        }
    });
}
