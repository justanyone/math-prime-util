// Regression tests for exact certificate and rejection-sampling boundaries.
#[allow(unused_macros)]
macro_rules! proving_cases {
    () => {
        #[test]
        fn ecpp_rejects_order_below_hasse_bound() {
            // The point has order 33. The Hasse lower bound is 35.
            assert!(!super::check_ecpp(
                &47_u8.into(), &2.into(), &2.into(), &33_u8.into(),
                &11_u8.into(), &2_u8.into(), &22_u8.into(),
            ));
        }

        #[test]
        fn sextic_twist_orders_are_exact() {
            // 4*7 = 1^2 + 3*3^2. All six twists have these orders.
            assert_eq!(
                super::curve_orders(-3, &7_u8.into(), &1_u8.into(), &3_u8.into()),
                [9_u8, 7, 13, 3, 4, 12].map(num_bigint::BigUint::from).to_vec(),
            );
        }

        #[test]
        fn zero_polynomial_on_either_side() {
            let n = 101_u8.into();
            let pm = super::PolyMod { n: &n };
            let f = [1_u8.into(), 0_u8.into(), 1_u8.into()];
            for (a, b) in [(vec![], vec![1_u8.into()]), (vec![1_u8.into()], vec![])] {
                assert!(pm.mul(&a, &b, &f).is_empty());
            }
        }

        #[test]
        fn bls_keeps_trial_factors_even_when_bound_already_holds() {
            let proofs = super::bls75(&109_u8.into()).expect("BLS proof");
            assert!(matches!(&proofs[0], super::Proof::Bls5 { q, .. } if *q == vec![3_u8.into()]));
            let proofs = super::bls75(&19_u8.into()).expect("BLS proof");
            let cert = super::Certificate { n: 19_u8.into(), proofs };
            assert_eq!(cert.verify(), Ok(true));
        }
    };
}

#[allow(unused_macros)]
macro_rules! random_cases {
    () => {
        #[test]
        fn modulus_rejects_incomplete_tail_and_reduces_the_draw() {
            let n = (num_bigint::BigUint::from(1_u8) << 64_u8) + 13_u8;
            let mut calls = 0;
            let result = super::urandomm_with(&n, |bits| {
                calls += 1;
                if calls == 1 {
                    (num_bigint::BigUint::from(1_u8) << bits) - 1_u8
                } else {
                    &n + 7_u8
                }
            });
            assert_eq!(calls, 2);
            assert_eq!(result, 7_u8.into());
        }

        #[test]
        fn factored_sampler_checks_product_and_strict_acceptance() {
            // First reject r > n without drawing an acceptance value.
            // Then reject draw == r, and finally accept draw == r - 1.
            let mut draws = [6_u8, 6, 0, 1, 0, 2, 1, 0, 1].into_iter();
            let result = super::random_factored_integer_with(&10_u8.into(), |_| {
                draws.next().expect("unexpected draw").into()
            });
            assert_eq!(result, (2_u8.into(), vec![2_u8.into()]));
            assert_eq!(draws.next(), None);
        }
    };
}

#[cfg(feature = "bigint")]
#[test]
fn seeded_large_random_results_replay() {
    use math_prime_util::bigint::random::{
        miller_rabin_random, random_prime, random_unrestricted_semiprime,
    };
    use num_bigint::BigUint;
    let low = BigUint::from(1_u8) << 66_u8;
    let high = &low + (BigUint::from(1_u8) << 66_u8);
    math_prime_util::srand(17);
    let prime = random_prime(&low, &high).expect("prime");
    assert!(prime >= low && prime <= high);
    assert_eq!(miller_rabin_random(&prime, 3), Some(1));
    let semiprime = random_unrestricted_semiprime(65).expect("semiprime");
    assert_eq!(semiprime.bits(), 65);
    math_prime_util::srand(17);
    assert_eq!(random_prime(&low, &high), Some(prime.clone()));
    assert_eq!(miller_rabin_random(&prime, 3), Some(1));
    assert_eq!(random_unrestricted_semiprime(65), Some(semiprime));
}
