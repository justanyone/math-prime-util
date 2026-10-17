//! Oracle checks for the u128 factoring paths.
mod common;
use math_prime_util::{factor_u128, is_prime, is_prime_u128};

#[test]
fn factor_oracle() {
    for case in common::load("factor")
        .into_iter()
        .filter(|c| c.func == "factor")
    {
        let n = case.arg::<u128>(0);
        let factors = factor_u128(n).expect("fixture must split");
        let result = factors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        case.check(Some(result));
        for p in factors {
            if p > 1 {
                assert_ne!(is_prime_u128(p), 0);
            }
        }
    }
}

#[test]
fn is_prime_oracle() {
    // The factor family has no is_prime rows. Check its inputs against u64,
    // and use the existing primality oracle for direct primality cases.
    for case in common::load("factor")
        .into_iter()
        .filter(|c| c.func == "factor")
    {
        if let Ok(n) = case.args[0].parse::<u64>() {
            assert_eq!(is_prime_u128(u128::from(n)), is_prime(n));
        }
    }
    for case in common::load("primality")
        .into_iter()
        .filter(|c| c.func == "is_prime")
    {
        // Negative inputs need the signed API, which this u128 path cannot take.
        if let Ok(n) = case.args[0].parse::<u128>() {
            case.check(Some(is_prime_u128(n)));
        }
    }
    assert_eq!(is_prime_u128((1u128 << 127) - 1), 1);
    assert_eq!(is_prime_u128(u128::MAX), 0);
}
