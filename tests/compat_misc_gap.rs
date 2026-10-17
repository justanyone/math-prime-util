//! Oracle cases for factor removal from the existing arithmetic fixture.
mod common;

use math_prime_util::{remove_factors, remove_factors_exp};

#[test]
fn remove_factors_oracle() {
    for case in common::load("arith") {
        if case.func != "remove_factors" && case.func != "remove_factors_exp" {
            continue;
        }
        // Skip signed n: the native API accepts only u64.
        if case.args[0].starts_with('-') {
            continue;
        }
        let n = case.arg(0);
        let k = case.arg(1);
        if case.error {
            let result = std::panic::catch_unwind(|| {
                if case.func == "remove_factors" {
                    remove_factors(n, k);
                } else {
                    remove_factors_exp(n, k);
                }
            });
            assert!(result.is_err(), "{case:?}");
        } else if case.func == "remove_factors" {
            case.check(remove_factors(n, k));
        } else {
            case.check(remove_factors_exp(n, k).map(|(r, e)| format!("{r} {e}")));
        }
    }
}
