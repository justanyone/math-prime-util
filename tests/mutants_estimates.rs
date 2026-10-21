//! Boundary checks for the lucky, Ramanujan-upper and almost-prime estimates.
//!
//! These tests reach the private helpers through public functions only. Each
//! test checks that a bound still holds at the inputs where a surviving
//! cargo-mutants mutant changes a comparison.

use math_prime_util::{
    lucky_count, lucky_count_lower, lucky_count_upper, nth_almost_prime, nth_lucky,
    nth_lucky_lower, nth_lucky_upper, nth_ramanujan_prime_upper,
};

/// `ramanujan_primes.c` `small_ram_primes`, the first 23 Ramanujan primes.
const SMALL_RAM: [u64; 23] = [
    2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227, 229, 233,
    239, 241,
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
    for n in [10_000, 10_001, 1_000_000, 1_000_001, 10_000_000, 100_000_000] {
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
