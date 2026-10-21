//! Regression tests for exact certificate and rejection-sampling boundaries.
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
        fn ecpp_rejects_order_above_hasse_bound() {
            // The point has order 33. A claimed order of 66 exceeds 61.
            assert!(!super::check_ecpp(
                &47_u8.into(), &2.into(), &2.into(), &66_u8.into(),
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
                // A zero product needs no polynomial reduction.
                assert!(pm.mul(&a, &b, &[]).is_empty());
            }
        }

        #[test]
        fn bls_keeps_trial_factors_even_when_bound_already_holds() {
            let proofs = super::bls75(&109_u8.into()).expect("BLS proof");
            assert!(matches!(&proofs[0], super::Proof::Bls5 { q, .. } if *q == vec![3_u8.into()]));
            // Trial division must expose the large prime cofactor.
            let proofs = super::bls75(&1_028_456_449_u64.into()).expect("BLS proof");
            assert!(matches!(&proofs[0], super::Proof::Bls5 { q, .. } if *q == vec![3_u8.into(), 5_231_u32.into()]));
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
        fn prime_sampler_last_bin_has_exact_width() {
            let low = num_bigint::BigUint::from(1_u8) << 66_u8;
            let high = &low << 1_u8;
            let binsize = num_bigint::BigUint::from(1_u8) << 32_u8;
            let bin_low = &high - (&binsize << 1_u8) + 1_u8;
            let prime = crate::bigint::primality::next_prime(&(&bin_low - 1_u8));
            let offset = (&prime - &bin_low) >> 1_u8;
            let mut calls = 0;
            let result = super::random_prime_with(&low, &high, |bound| {
                calls += 1;
                if calls == 1 {
                    assert_eq!(*bound, num_bigint::BigUint::from(1_u8) << 33_u8);
                    bound - 1_u8
                } else {
                    assert_eq!(*bound, binsize);
                    offset.clone()
                }
            });
            assert_eq!(calls, 2);
            assert_eq!(result, Some(prime));
        }

        #[test]
        fn miller_rabin_lowest_draw_is_base_two() {
            let n = (num_bigint::BigUint::from(1_u8) << 127_u8) - 1_u8;
            assert_eq!(
                super::miller_rabin_random_with(&n, 1, |bound| {
                    assert_eq!(*bound, &n - 2_u8);
                    0_u8.into()
                }),
                1
            );
        }

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
        #[test]
        fn factored_sampler_accepts_product_equal_to_limit() {
            let mut draws = [4_u8, 1, 0, 9].into_iter();
            let result = super::random_factored_integer_with(&10_u8.into(), |_| {
                draws.next().expect("unexpected draw").into()
            });
            assert_eq!(result, (10_u8.into(), vec![2_u8.into(), 5_u8.into()]));
            assert_eq!(draws.next(), None);
        }

        #[test]
        fn semiprime_first_weight_selects_two() {
            let probability = 1.912_183_974_522_43 / 65.0 + 0.195_56 / 2.0;
            let seed = (0..1000)
                .find(|&seed| {
                    crate::srand(seed);
                    crate::drand(1.0) <= probability
                })
                .expect("a draw in the first weight");
            crate::srand(seed);
            let n = super::random_unrestricted_semiprime(65).expect("semiprime");
            assert_eq!(n.bits(), 65);
            assert!(!n.bit(0), "the selected prime factor must be two");
        }

        #[cfg(feature = "bigint")]
        #[test]
        fn seeded_large_random_results_replay() {
            use crate::bigint::random::{
                miller_rabin_random, random_prime, random_unrestricted_semiprime,
            };
            use num_bigint::BigUint;
            let low = BigUint::from(1_u8) << 66_u8;
            let high = &low + (BigUint::from(1_u8) << 66_u8);
            crate::srand(17);
            let prime = random_prime(&low, &high).expect("prime");
            assert!(prime >= low && prime <= high);
            crate::srand(17);
            // This interval has 2^65 odd candidates in 2^33 equal bins.
            let bins = BigUint::from(1_u8) << 33_u8;
            let part = crate::bigint::random::urandomm(&bins);
            let bin_low = &low + 1_u8 + (part << 33_u8);
            let bin_size = BigUint::from(1_u8) << 32_u8;
            let expected = loop {
                let candidate = &bin_low + (crate::bigint::random::urandomm(&bin_size) << 1_u8);
                if crate::bigint::primality::is_prob_prime(&candidate) != 0 {
                    break candidate;
                }
            };
            assert_eq!(prime, expected);
            assert_eq!(miller_rabin_random(&prime, 3), Some(1));
            let semiprime = random_unrestricted_semiprime(65).expect("semiprime");
            assert_eq!(semiprime.bits(), 65);
            crate::srand(17);
            assert_eq!(random_prime(&low, &high), Some(prime.clone()));
            assert_eq!(miller_rabin_random(&prime, 3), Some(1));
            assert_eq!(random_unrestricted_semiprime(65), Some(semiprime));
        }
    };
}

#[allow(unused_imports)]
pub(crate) use proving_cases;
#[allow(unused_imports)]
pub(crate) use random_cases;
