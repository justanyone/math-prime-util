//! Regression cases included by the four source modules to test private estimates.
//! The integration target is empty. Run with `tools/test.sh --lib estimate_mutation`.

macro_rules! public_estimate_tests {
    () => {
        use crate::{
            lucky_count, lucky_count_lower, lucky_count_upper, nth_almost_prime, nth_lucky,
            nth_lucky_lower, nth_lucky_upper, nth_ramanujan_prime_upper,
        };

        /// `ramanujan_primes.c` `small_ram_primes`, the first 23 Ramanujan primes.
        const SMALL_RAM: [u64; 23] = [
            2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227,
            229, 233, 239, 241,
        ];

        /// Lucky count bounds must bracket the exact count across the small-n
        /// branch (`n < 48`) and the first interpolation steps above it.
        #[test]
        fn lucky_count_bounds_bracket_exact_count() {
            for n in (1..=3_000).chain([4_096, 10_000]) {
                let exact = lucky_count(n);
                let lo = lucky_count_lower(n);
                let hi = lucky_count_upper(n);
                assert!(lo <= exact, "lower {lo} > exact {exact} at n={n}");
                assert!(exact <= hi, "exact {exact} > upper {hi} at n={n}");
            }
        }

        /// The `n <= 10_000` and `n <= 1_000_000` branch limits of the lucky
        /// estimate must keep the lower bound below the upper bound.
        #[test]
        fn lucky_count_bounds_are_ordered_at_branch_limits() {
            for n in [
                10_000,
                10_001,
                1_000_000,
                1_000_001,
                10_000_000,
                100_000_000,
            ] {
                assert!(lucky_count_lower(n) <= lucky_count_upper(n), "n={n}");
            }
        }

        /// `nth_lucky_lower(n) <= nth_lucky(n) <= nth_lucky_upper(n)` for small `n`.
        #[test]
        fn nth_lucky_bounds_bracket_exact_value() {
            for n in 1..=500 {
                let exact = nth_lucky(n).unwrap_or(u64::MAX);
                let lo = nth_lucky_lower(n).unwrap_or(0);
                let hi = nth_lucky_upper(n).unwrap_or(u64::MAX);
                assert!(lo <= exact, "lower {lo} > exact {exact} at n={n}");
                assert!(exact <= hi, "exact {exact} > upper {hi} at n={n}");
            }
        }

        /// The Ramanujan upper bound must not fall below the known Ramanujan primes
        /// `R_1` through `R_23`, which sit in the small-n range.
        #[test]
        fn ramanujan_upper_bounds_small_ramanujan_primes() {
            for (i, &r) in SMALL_RAM.iter().enumerate() {
                let n = (i + 1) as u64;
                let hi = nth_ramanujan_prime_upper(n).unwrap_or(0);
                assert!(hi >= r, "upper {hi} < R_{n} = {r}");
            }
        }

        /// The Ramanujan upper bound must grow with the index across the Axler
        /// band edges, where `axler_d_tail` changes its constant.
        #[test]
        fn ramanujan_upper_is_increasing_over_small_indices() {
            let mut prev = 0;
            for n in 1..=2_000 {
                let hi = nth_ramanujan_prime_upper(n).unwrap_or(0);
                assert!(hi > prev, "bound not increasing at n={n}: {hi} <= {prev}");
                prev = hi;
            }
        }

        /// `nth_almost_prime(2, n)` gives the semiprimes in order. The
        /// interpolation path must land on the exact value for small `n`.
        #[test]
        fn nth_almost_prime_k2_matches_semiprimes() {
            let semis: Vec<u64> = (4..=200u64)
                .filter(|&m| {
                    let mut count = 0;
                    let mut d = 2;
                    let mut r = m;
                    while d * d <= r {
                        while r % d == 0 {
                            r /= d;
                            count += 1;
                        }
                        d += 1;
                    }
                    if r > 1 {
                        count += 1;
                    }
                    count == 2
                })
                .collect();
            for (i, &s) in semis.iter().enumerate() {
                assert_eq!(nth_almost_prime(2, (i + 1) as u64), Some(s), "n={}", i + 1);
            }
        }

        /// `k == 0` and `n == 0` have fixed answers, which the branch checks rely on.
        #[test]
        fn nth_almost_prime_edge_inputs() {
            assert_eq!(nth_almost_prime(0, 1), Some(1));
            assert_eq!(nth_almost_prime(0, 2), None);
            assert_eq!(nth_almost_prime(3, 0), None);
        }
    };
}

macro_rules! private_estimate_tests {
    (lucky) => {
        #[test]
        fn lucky_estimate_branch_values() {
            // MPU lucky_numbers.c, _simple_lucky_count_approx.
            for (n, expected) in [
                (6, 2),
                (7, 3),
                (10_000, 1124),
                (10_001, 1081),
                (1_000_000, 72074),
                (1_000_001, 72074),
                (10_000_000, 611161),
                (10_000_001, 610607),
                (100_000_000, 5281876),
                (100_000_001, 5287381),
            ] {
                assert_eq!(super::simple_approx(n), expected, "n={n}");
            }
        }
    };
    (ramanujan) => {
        #[test]
        fn ramanujan_tail_boundaries() {
            // MPU ramanujan_primes.c, nth_ramanujan_prime_upper.
            assert_eq!(super::axler_d_tail(3_999_654_659), -2.0884);
            assert_eq!(super::axler_d_tail(3_999_654_660), -2.235);
            assert_eq!(super::axler_d_tail(28_330_126_673_435), -2.635);
            assert_eq!(super::axler_d_tail(28_330_126_673_436), -2.735);
            assert_eq!(super::axler_d_tail(117_462_814_829_787), -2.735);
            assert_eq!(super::axler_d_tail(117_462_814_829_788), -2.8);
        }

        #[test]
        fn ramanujan_prime_upper_306() {
            // MPU prime_counts.c, nth_prime_upper: exact below 306.
            assert_eq!(super::nth_prime_upper(305), 2011);
            assert_eq!(super::nth_prime_upper(306), 2028);
            assert!(super::nth_prime_upper(306) >= crate::nth_prime(306).unwrap());
        }
    };
    (almost) => {
        #[test]
        fn almost_interpolation_bracket_trace() {
            // MPU inverse_interpolate.c, interpolate_with_approx.
            // Use an exact count of even integers and a deliberately rough estimate.
            let visits = std::cell::RefCell::new(Vec::new());
            let value = super::interpolate_approx_with(
                100_000,
                |n| match n {
                    100_000 => 300_000,
                    150_000 | 50_000 => 100_000,
                    250_000 => 700_000,
                    _ => n * 2,
                },
                |x| {
                    visits.borrow_mut().push(x);
                    x / 2
                },
                |x| x % 2 == 0,
            );
            assert_eq!(value, 200_000);
            assert_eq!(&visits.borrow()[..4], &[300_000, 500_000, 100_000, 119_999]);
        }
    };
    (real) => {
        #[test]
        fn harmonic_asymptotic_boundary() {
            let exact: f64 = (1..=2003).map(|n| 1.0 / f64::from(n)).sum();
            assert!((crate::harmreal(2003) - exact).abs() < 2e-14);
            assert!(crate::harmreal(u64::MAX).is_finite());
        }
    };
}
