//! Oracle checks for the native factorial subset.
mod common;

use math_prime_util::{
    bell_number, binomial, catalan_number, consecutive_integer_lcm, factorial, falling_factorial,
    fubini, multifactorial, pn_primorial, primorial, rising_factorial, subfactorial,
};

fn check_native(name: &str, function: impl Fn(&[u64]) -> Option<u64> + std::panic::RefUnwindSafe) {
    let mut checked = 0;
    for case in common::load("combinat")
        .into_iter()
        .filter(|c| c.func == name)
    {
        // Signed arguments and arguments above u64::MAX are outside this API.
        let Ok(args) = case
            .args
            .iter()
            .map(|a| a.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
        else {
            continue;
        };
        let got = std::panic::catch_unwind(|| function(&args));
        if case.error {
            assert!(got.is_err(), "oracle line {} must panic", case.line);
        } else {
            // MPU promotes overflowing results to bigint. The native API returns None.
            let expected = case.result.as_ref().and_then(|s| s.parse::<u64>().ok());
            assert_eq!(got.unwrap(), expected, "oracle line {}", case.line);
        }
        checked += 1;
    }
    assert!(checked > 0, "missing oracle cases for {name}");
}

#[test]
fn binomial_oracle() {
    check_native("binomial", |a| binomial(a[0], a[1]));
}

#[test]
fn catalan_number_oracle() {
    check_native("catalan_number", |a| catalan_number(a[0]));
}

#[test]
fn bell_number_oracle() {
    check_native("bell_number", |a| bell_number(a[0]));
}

#[test]
fn fubini_oracle() {
    check_native("fubini", |a| fubini(a[0]));
}

#[test]
fn primorial_oracle() {
    check_native("primorial", |a| primorial(a[0]));
}

#[test]
fn pn_primorial_oracle() {
    check_native("pn_primorial", |a| pn_primorial(a[0]));
}

#[test]
fn consecutive_integer_lcm_oracle() {
    check_native("consecutive_integer_lcm", |a| consecutive_integer_lcm(a[0]));
}

#[test]
fn factorial_oracle() {
    check_native("factorial", |a| factorial(a[0]));
}

#[test]
fn multifactorial_oracle() {
    check_native("multifactorial", |a| multifactorial(a[0], a[1]));
}

#[test]
fn subfactorial_oracle() {
    check_native("subfactorial", |a| subfactorial(a[0]));
}

#[test]
fn falling_factorial_oracle() {
    check_native("falling_factorial", |a| falling_factorial(a[0], a[1]));
}

#[test]
fn rising_factorial_oracle() {
    check_native("rising_factorial", |a| rising_factorial(a[0], a[1]));
}
