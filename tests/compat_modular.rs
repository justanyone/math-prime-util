//! Native unsigned cases from the modular MPU oracle.
mod common;
use math_prime_util::{binomialmod, factorialmod, pisano_period, znlog};

fn native_cases(name: &str, f: impl Fn(&common::Case)) {
    for c in common::load("modular").iter().filter(|c| c.func == name) {
        // Skip signed binomialmod n/k and bigint inputs to all four functions:
        // these cannot be represented by this native u64 API.
        if c.args.iter().any(|s| s.parse::<u64>().is_err()) {
            continue;
        }
        f(c);
    }
}
#[test]
fn znlog_oracle() {
    native_cases("znlog", |c| c.check(znlog(c.arg(0), c.arg(1), c.arg(2))));
}
#[test]
fn factorialmod_oracle() {
    native_cases("factorialmod", |c| {
        c.check(factorialmod(c.arg(0), c.arg(1)));
    });
}
#[test]
fn binomialmod_oracle() {
    native_cases("binomialmod", |c| {
        c.check(binomialmod(c.arg(0), c.arg(1), c.arg(2)));
    });
}
#[test]
fn pisano_period_oracle() {
    native_cases("pisano_period", |c| c.check(Some(pisano_period(c.arg(0)))));
}
