//! Killing tests for surviving mutants in the lucky, Ramanujan-prime upper
//! and almost-prime estimate helpers. Each test uses the public API and
//! checks values that a flipped comparison or operator would change.

use math_prime_util::families::{
    lucky_count, lucky_count_lower, lucky_count_upper, nth_almost_prime, nth_lucky,
    nth_ramanujan_prime_upper,
};
use math_prime_util::real::harmreal;

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
