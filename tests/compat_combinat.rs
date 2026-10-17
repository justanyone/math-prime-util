//! MPU 0.75 combinat fixture checks for the four functions in this slice.
mod common;

use common::each;
use math_prime_util::{falling_factorial, integer_complexity, powersum, rising_factorial};

fn check_binary(name: &str, function: fn(u64, u64) -> Option<u64>) {
    each("combinat", name, |case| {
        // Skip negative n (signed factorial API), and bigint n or m inputs.
        let (Ok(n), Ok(m)) = (case.args[0].parse::<u64>(), case.args[1].parse::<u64>()) else {
            return;
        };
        assert!(!case.error, "unexpected oracle error: {case:?}");
        // MPU promotes overflowing results to bigint; this native API returns None.
        let expected = case
            .result
            .as_ref()
            .and_then(|value| value.parse::<u64>().ok());
        assert_eq!(function(n, m), expected, "{case:?}");
    });
}

#[test]
fn falling_factorial_oracle() {
    check_binary("falling_factorial", falling_factorial);
}

#[test]
fn rising_factorial_oracle() {
    check_binary("rising_factorial", rising_factorial);
}

#[test]
fn powersum_oracle() {
    check_binary("powersum", powersum);
}

#[test]
fn integer_complexity_oracle() {
    each("combinat", "integer_complexity", |case| {
        case.check(Some(integer_complexity(case.arg(0))));
    });
}
