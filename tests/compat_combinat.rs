//! Oracle checks for the assigned combinat functions.
mod common;

use common::{each, each_may_panic};
use math_prime_util::{
    consecutive_integer_lcm, frobenius_number, is_congruent_number, is_polygonal,
};

#[test]
fn consecutive_integer_lcm_oracle() {
    each("combinat", "consecutive_integer_lcm", |case| {
        if let Ok(n) = case.args[0].parse() {
            // MPU promotes overflow to bigint; this API returns None.
            let expected = case.result.as_ref().and_then(|r| r.parse::<u64>().ok());
            assert_eq!(consecutive_integer_lcm(n), expected, "{case:?}");
        }
        // Skip bigint arguments outside the u64 API.
    });
}

#[test]
fn frobenius_number_oracle() {
    each_may_panic("combinat", "frobenius_number", |case| {
        let values: Vec<u64> = case.args.iter().map(|a| a.parse().unwrap()).collect();
        let got = frobenius_number(&values);
        // MPU promotes overflow to bigint; this API returns None.
        if case
            .result
            .as_ref()
            .is_some_and(|r| r.parse::<u64>().is_err() && r != "-1")
        {
            assert_eq!(got, None, "{case:?}");
            return case.result.clone();
        }
        got.map(|n| n.to_string())
    });
}

#[test]
fn is_congruent_number_oracle() {
    each("combinat", "is_congruent_number", |case| {
        // Skip -3: signed inputs are outside the u64 API.
        if let Ok(n) = case.args[0].parse() {
            case.check(Some(u8::from(is_congruent_number(n))));
        }
    });
}

#[test]
fn is_polygonal_oracle() {
    each("combinat", "is_polygonal", |case| {
        // Skip signed n and bigint n or k: these are outside the u64 API.
        if let (Ok(n), Ok(k)) = (case.args[0].parse(), case.args[1].parse()) {
            let result = std::panic::catch_unwind(|| is_polygonal(n, k));
            if case.error {
                assert!(result.is_err(), "{case:?}");
            } else {
                case.check(Some(u8::from(result.unwrap())));
            }
        }
    });
}
