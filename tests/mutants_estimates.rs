//! Tests that pin the bound and estimate helpers behind the lucky,
//! Ramanujan-upper and almost-prime families. They use only the public
//! API, so they check results at the boundary inputs the helpers serve.

use math_prime_util::*;

/// Direct harmonic sum, used as the reference for `harmreal`.
fn direct_harmonic(n: u64) -> f64 {
    (1..=n).map(|k| 1.0 / k as f64).sum()
}

#[test]
fn harmreal_matches_direct_sum_across_split() {
    // 2002 uses the exact split, 2003 and up use the asymptotic series.
    for n in [1, 2, 2002, 2003, 2004, 5000, 100_000] {
        let got = harmreal(n);
        let want = direct_harmonic(n);
        assert!((got - want).abs() < 1e-9, "harmreal({n}) = {got}, want {want}");
    }
}

#[test]
fn nth_prime_upper_bounds_nth_prime() {
    // Covers the small-index path (n < 306) and the simple-search path (n < 15360).
    for n in (1..=400).chain([1000, 5000, 15_000]) {
        let upper = nth_prime_upper(n).expect("in range");
        let exact = nth_prime(n).expect("in range");
        assert!(upper >= exact, "nth_prime_upper({n}) = {upper} < {exact}");
    }
}

#[test]
fn nth_ramanujan_prime_upper_bounds_known_values() {
    // Ramanujan primes R_1..R_10 (OEIS A104272).
    let known = [2, 11, 17, 29, 41, 47, 59, 67, 71, 97];
    for (i, &r) in known.iter().enumerate() {
        let n = i as u64 + 1;
        let upper = nth_ramanujan_prime_upper(n).expect("in range");
        assert!(upper >= r, "nth_ramanujan_prime_upper({n}) = {upper} < R_n = {r}");
    }
}

#[test]
fn nth_almost_prime_exact_for_small_k() {
    // Semiprimes and 3-almost primes, checked against their known sequences.
    let semiprimes = [4, 6, 9, 10, 14, 15, 21, 22, 25, 26];
    for (i, &v) in semiprimes.iter().enumerate() {
        assert_eq!(nth_almost_prime(2, i as u64 + 1), Some(v));
    }
    let three = [8, 12, 18, 20, 27, 28, 30];
    for (i, &v) in three.iter().enumerate() {
        assert_eq!(nth_almost_prime(3, i as u64 + 1), Some(v));
    }
}

#[test]
fn lucky_numbers_match_known_prefix() {
    let lucky = [1, 3, 7, 9, 13, 15, 21, 25, 31, 33];
    for v in 1..=33u64 {
        assert_eq!(is_lucky(v), lucky.contains(&v), "is_lucky({v})");
    }
}

#[test]
fn nth_lucky_returns_lucky_values() {
    for n in 1..=50 {
        let v = nth_lucky(n).expect("in range");
        assert!(is_lucky(v), "nth_lucky({n}) = {v} is not lucky");
    }
}
