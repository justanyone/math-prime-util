//! Compatibility checks against the committed MPU combinat oracle.
mod common;

use common::{Case, each};
use math_prime_util::{factorial, fubini, multifactorial, subfactorial};

fn check_native(case: &Case, function: impl FnOnce(&[u64]) -> Option<u64>) {
    let Ok(args) = case
        .args
        .iter()
        .map(|arg| arg.parse())
        .collect::<Result<Vec<u64>, _>>()
    else {
        // Skip negative factorial/subfactorial/multifactorial arguments and
        // the multifactorial case whose n and k exceed u64.
        return;
    };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| function(&args)));
    if case.error {
        assert!(result.is_err(), "expected panic: {case:?}");
    } else {
        // MPU's bigint answers beyond u64 map to the native API's None.
        let expected = case
            .result
            .as_ref()
            .and_then(|value| value.parse::<u64>().ok());
        assert_eq!(result.unwrap(), expected, "{case:?}");
    }
}

#[test]
fn fubini_oracle() {
    each("combinat", "fubini", |c| check_native(c, |a| fubini(a[0])));
}

#[test]
fn factorial_oracle() {
    each("combinat", "factorial", |c| {
        check_native(c, |a| factorial(a[0]));
    });
}

#[test]
fn multifactorial_oracle() {
    each("combinat", "multifactorial", |c| {
        check_native(c, |a| multifactorial(a[0], a[1]));
    });
}

#[test]
fn subfactorial_oracle() {
    each("combinat", "subfactorial", |c| {
        check_native(c, |a| subfactorial(a[0]));
    });
}
