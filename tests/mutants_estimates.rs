//! Regression tests for estimate mutations.

#[allow(unused_macros)]
macro_rules! public_estimate_tests {
    () => {
        use crate::families::{
            lucky_count, lucky_count_lower, lucky_count_upper, nth_almost_prime, nth_lucky,
            nth_ramanujan_prime_upper,
        };
        use crate::real::harmreal;

        #[test]
        fn harmreal_matches_small_harmonic_numbers() {
            assert!((harmreal(1) - 1.0).abs() < 1e-12);
            assert!((harmreal(2) - 1.5).abs() < 1e-12);
            assert!((harmreal(4) - 25.0 / 12.0).abs() < 1e-12);
        }

        #[test]
        fn ramanujan_prime_upper_bounds_known_values() {
            // Ramanujan primes R_1..R_10: 2, 11, 17, 29, 41, 47, 59, 67, 71, 97.
            let r = [2u64, 11, 17, 29, 41, 47, 59, 67, 71, 97];
            for (i, &rn) in r.iter().enumerate() {
                let upper = nth_ramanujan_prime_upper((i + 1) as u64).unwrap();
                assert!(upper >= rn, "upper {upper} < R_{} = {rn}", i + 1);
            }
        }

        #[test]
        fn nth_almost_prime_semiprimes_and_three_almost() {
            let semi = [4u64, 6, 9, 10, 14, 15, 21, 22, 25, 26];
            for (i, &v) in semi.iter().enumerate() {
                assert_eq!(nth_almost_prime(2, (i + 1) as u64), Some(v));
            }
            let three = [8u64, 12, 18, 20, 27, 28, 30];
            for (i, &v) in three.iter().enumerate() {
                assert_eq!(nth_almost_prime(3, (i + 1) as u64), Some(v));
            }
        }

        #[test]
        fn lucky_bounds_bracket_exact_count() {
            for n in 1..=300u64 {
                let exact = lucky_count(n);
                assert!(lucky_count_lower(n) <= exact, "lower > exact at n={n}");
                assert!(lucky_count_upper(n) >= exact, "upper < exact at n={n}");
            }
        }

        #[test]
        fn lucky_numbers_small_values() {
            let lucky = [1u64, 3, 7, 9, 13, 15, 21, 25, 31, 33, 37, 43];
            for (i, &v) in lucky.iter().enumerate() {
                assert_eq!(nth_lucky((i + 1) as u64), Some(v));
            }
            // lucky_count(x) counts lucky numbers <= x.
            assert_eq!(lucky_count(12), 4);
            assert_eq!(lucky_count(43), 12);
        }
    };
}

// The source modules include this file so tests can reach private helpers.
#[allow(unused_macros)]
macro_rules! private_estimate_tests {
    (lucky) => {
        #[test]
        fn mutation_simple_approx_boundaries() {
            for (n, expected) in [
                (7, 3),
                (1_000_001, 71_653),
                (10_000_000, 609_491),
                (10_000_001, 608_818),
                (100_000_000, 5_286_045),
                (100_000_001, 5_281_586),
            ] {
                assert_eq!(super::simple_approx(n), expected, "n={n}");
            }
        }
    };
    (ramanujan) => {
        #[test]
        fn mutation_axler_tail_boundaries() {
            for (n, expected) in [
                (3_999_654_659, -2.0884),
                (3_999_654_660, -2.235),
                (514_808_375_201, -2.435),
                (514_808_375_202, -2.535),
                (84_086_679_236, -2.235),
                (84_086_679_237, -2.435),
            ] {
                assert!((super::axler_d_tail(n) - expected).abs() < f64::EPSILON);
            }
        }
        #[test]
        fn mutation_nth_prime_upper_boundary() {
            assert_eq!(super::nth_prime_upper(306), 2_017);
            assert_eq!(super::nth_prime_upper(15_360), 168_432);
        }
    };
    (almost) => {
        #[test]
        fn mutation_interpolation_bracket_repairs() {
            // Exact count is the identity. Deliberately poor estimates force
            // the same bracket repair paths as the C callback algorithm.
            for (estimate, expected) in [
                (
                    (|n: u64| if n == 100_000 { 50_000 } else { 200_000 }) as fn(u64) -> u64,
                    vec![50_000, 69_999, 89_998],
                ),
                (
                    (|n: u64| if n == 100_000 { 150_000 } else { 0 }) as fn(u64) -> u64,
                    vec![150_000, 130_001, 110_002],
                ),
            ] {
                let guesses = std::cell::RefCell::new(Vec::new());
                assert_eq!(
                    super::interpolate_approx_with(
                        100_000,
                        estimate,
                        |x| {
                            guesses.borrow_mut().push(x);
                            x
                        },
                        |_| true,
                    ),
                    100_000
                );
                assert_eq!(*guesses.borrow(), expected);
            }
        }
    };
    (real) => {
        #[test]
        #[allow(clippy::cast_precision_loss)]
        fn mutation_harmreal_asymptotic_boundary() {
            for n in [2003_u64, 10_000, u64::MAX] {
                let expected = (1..=n.min(10_000)).map(|k| 1.0 / k as f64).sum::<f64>();
                if n <= 10_000 {
                    assert!((super::harmreal(n) - expected).abs() < 1e-12);
                } else {
                    assert!(super::harmreal(n).is_finite());
                }
            }
        }
    };
}
