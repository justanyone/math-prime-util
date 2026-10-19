//! The arbitrary precision factoring and divisor functions agree with the
//! native path on the `u64` range (and `factor` with `factor_u128` on the
//! `u128` range), and every factoring method returns a valid split.
#![cfg(feature = "bigint")]
use math_prime_util::Int;
use math_prime_util::bigint::primality::is_prob_prime;
use math_prime_util::bigint::{divisors, factor};
use num_bigint::BigUint;
use proptest::prelude::*;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

fn product(values: &[BigUint]) -> BigUint {
    values.iter().fold(BigUint::from(1_u8), |acc, v| acc * v)
}

/// A valid split: the parts multiply to `n` and none is 1 (or 0 for n > 0).
fn valid_split(n: u64, parts: &[BigUint]) -> bool {
    if n == 0 {
        return parts == [big(0)];
    }
    product(parts) == big(n) && parts.iter().all(|f| *f > BigUint::from(1_u8))
}

proptest! {
    #[test]
    fn factoring_agrees(n: u64, small in 0_u64..1_000_000, wide: u128) {
        for n in [n, small] {
            let bn = big(n);
            let native: Vec<BigUint> = math_prime_util::factor(n).into_iter().map(big).collect();
            let got = factor::factor(&bn).unwrap();
            prop_assert_eq!(&got, &native);
            prop_assert_eq!(bn.factor().unwrap(), native);
            prop_assert!(got.iter().all(|p| is_prob_prime(p) != 0));
            prop_assert_eq!(product(&got), if n == 0 { big(0) } else { bn.clone() });
            let exp: Vec<(BigUint, u32)> = math_prime_util::factor_exp(n).into_iter().map(|(p, e)| (big(p), e)).collect();
            prop_assert_eq!(factor::factor_exp(&bn).unwrap(), exp);
            let within = factor::factor_within(&bn, std::time::Duration::from_secs(10));
            prop_assert!(within.unsplit.is_empty());
            prop_assert_eq!(within.primes, got);
        }
        let bw = BigUint::from(wide);
        let native: Result<Vec<BigUint>, BigUint> = math_prime_util::factor_u128(wide)
            .map(|v| v.into_iter().map(BigUint::from).collect())
            .map_err(BigUint::from);
        let got = factor::factor(&bw);
        prop_assert_eq!(&got, &native);
        if let Ok(got) = got {
            prop_assert!(got.iter().all(|p| is_prob_prime(p) != 0));
            prop_assert_eq!(product(&got), bw);
        }
    }

    #[test]
    fn divisor_functions_agree(n: u64, small in 0_u64..100_000, k in 0_u64..4) {
        for n in [n, small] {
            let bn = big(n);
            prop_assert_eq!(divisors::euler_phi(&bn).unwrap(), big(math_prime_util::euler_phi(n)));
            prop_assert_eq!(divisors::carmichael_lambda(&bn).unwrap(), big(math_prime_util::carmichael_lambda(n)));
            prop_assert_eq!(divisors::moebius(&bn).unwrap(), math_prime_util::moebius(n));
            prop_assert_eq!(divisors::is_square_free(&bn).unwrap(), math_prime_util::is_square_free(n));
            if n > 0 {
                prop_assert_eq!(divisors::prime_bigomega(&bn).unwrap(), u64::from(math_prime_util::prime_bigomega(n)));
                prop_assert_eq!(divisors::prime_omega(&bn).unwrap(), u64::from(math_prime_util::prime_omega(n)));
            }
            if let Some(sigma) = math_prime_util::divisor_sum(n, k) {
                prop_assert_eq!(divisors::divisor_sum(&bn, k).unwrap(), big(sigma));
            }
            let count = math_prime_util::divisor_sum(n, 0).unwrap_or(u64::MAX);
            if count <= 4096 {
                let native: Vec<BigUint> = math_prime_util::divisors(n, None).into_iter().map(big).collect();
                prop_assert_eq!(divisors::divisors(&bn, None).unwrap(), native);
            }
            let max = n / 7 + 1;
            let native: Vec<BigUint> = math_prime_util::divisors(n, Some(max)).into_iter().map(big).collect();
            prop_assert_eq!(divisors::divisors(&bn, Some(&big(max))).unwrap(), native);
        }
    }

    #[test]
    fn methods_return_valid_splits(n: u64, small in 1_u64..(1 << 40), rounds in 1_u64..5000) {
        for n in [n, small] {
            let bn = big(n);
            prop_assert!(valid_split(n, &factor::trial_factor(&bn, Some(rounds))));
            prop_assert!(valid_split(n, &factor::prho_factor(&bn, Some(rounds))));
            prop_assert!(valid_split(n, &factor::pbrent_factor(&bn, Some(rounds), Some(rounds % 7))));
            prop_assert!(valid_split(n, &factor::pminus1_factor(&bn, Some(rounds), Some(rounds * 3))));
            prop_assert!(valid_split(n, &factor::pplus1_factor(&bn, Some(rounds % 500))));
            prop_assert!(valid_split(n, &factor::holf_factor(&bn, Some(rounds), None)));
            prop_assert!(valid_split(n, &factor::squfof_factor(&bn, Some(rounds))));
            prop_assert!(valid_split(n, &factor::ecm_factor(&bn, Some(rounds % 100 + 2), Some(rounds), Some(2))));
        }
        let full: Vec<BigUint> = math_prime_util::factor(small).into_iter().map(big).collect();
        prop_assert_eq!(factor::trial_factor(&big(small), None), full);
        if small % 2 == 1 && math_prime_util::is_prime(small) == 0 && small > 1 {
            // SQUFOF and Brent with the default rounds split every composite of this size.
            let got = factor::squfof_factor(&big(small), None);
            prop_assert!(got.len() >= 2 || math_prime_util::factor(small).len() == 1);
            let got = factor::pbrent_factor(&big(small), None, None);
            prop_assert!(got.len() >= 2);
        }
    }
}
