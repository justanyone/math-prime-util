//! Tests that kill cargo-mutants survivors in the lucky, Ramanujan-upper,
//! almost-prime and harmonic estimate code. Each test uses only the public API.

use math_prime_util::{
    harmreal, lucky_count, lucky_count_approx, lucky_count_lower, lucky_count_upper,
    nth_almost_prime, nth_prime, nth_ramanujan_prime_upper,
};

/// Count prime factors with multiplicity for every value up to `limit`.
fn omega_table(limit: usize) -> Vec<u8> {
    let mut omega = vec![0u8; limit + 1];
    let mut composite = vec![false; limit + 1];
    for p in 2..=limit {
        if composite[p] {
            continue;
        }
        for m in (2 * p..=limit).step_by(p) {
            composite[m] = true;
        }
        let mut pk = p;
        while pk <= limit {
            for j in (pk..=limit).step_by(pk) {
                omega[j] += 1;
            }
            pk *= p;
        }
    }
    omega
}

/// Brute-force nth k-almost prime (1-based) by scanning the omega table.
fn brute_nth_almost(omega: &[u8], k: u8, n: usize) -> u64 {
    let mut seen = 0;
    for (j, &w) in omega.iter().enumerate().skip(2) {
        if w == k {
            seen += 1;
            if seen == n {
                return j as u64;
            }
        }
    }
    panic!("table too short for n = {n}");
}

#[test]
fn nth_almost_prime_matches_brute_force_for_large_n() {
    // Large n drives the interpolation path in interpolate_approx.
    let omega = omega_table(4_000_000);
    for &(k, n) in &[
        (2u8, 1usize),
        (2, 1_000),
        (2, 100_000),
        (2, 500_000),
        (3, 200_000),
    ] {
        let expected = brute_nth_almost(&omega, k, n);
        assert_eq!(
            nth_almost_prime(u32::from(k), n as u64),
            Some(expected),
            "k={k} n={n}"
        );
    }
}

#[test]
fn lucky_bounds_sandwich_exact_count_near_regime_edges() {
    // simple_lower and simple_upper use simple_approx above n = 9000.
    for n in [48u64, 8_999, 9_000, 9_001, 10_000, 10_001, 20_000] {
        let exact = lucky_count(n);
        let lo = lucky_count_lower(n);
        let hi = lucky_count_upper(n);
        assert!(lo <= exact, "lower {lo} > exact {exact} at n={n}");
        assert!(exact <= hi, "exact {exact} > upper {hi} at n={n}");
        let approx = lucky_count_approx(n);
        assert!(
            approx.abs_diff(exact) * 20 <= exact,
            "approx {approx} is off by more than 5% from exact {exact} at n={n}"
        );
    }
}

#[test]
fn lucky_small_n_below_seven_uses_exact_table() {
    // simple_approx branch n < 7 and simple_lower branch n <= 6.
    for n in 1u64..=7 {
        assert_eq!(lucky_count_lower(n), lucky_count(n), "n={n}");
    }
}

#[test]
fn ramanujan_upper_bounds_prime_index_for_small_n() {
    // The Ramanujan upper bound must not fall below p_n.
    for n in [1u64, 2, 10, 100, 305, 306, 1_000, 15_360] {
        let p = nth_prime(n).unwrap();
        let up = nth_ramanujan_prime_upper(n).unwrap();
        assert!(up >= p, "upper {up} < p_{n} = {p}");
    }
}

#[test]
fn harmonic_numbers_at_small_and_split_sizes() {
    assert!((harmreal(1) - 1.0).abs() < 1e-15);
    assert!((harmreal(10) - 2.928_968_253_968_254).abs() < 1e-12);
    assert!((harmreal(2002) - harmreal(2001) - 1.0 / 2002.0).abs() < 1e-12);
}

#[test]
fn lucky_bounds_at_large_fit_transitions() {
    // MPU lucky_numbers.c::_simple_lucky_count_approx changes fits here.
    for n in [1_000_001, 10_000_000, 10_000_001, 100_000_000, 100_000_001] {
        let exact = lucky_count(n);
        assert!(lucky_count_lower(n) <= exact, "n={n}");
        assert!(lucky_count_upper(n) >= exact, "n={n}");
    }
}

#[test]
fn harmonic_asymptotic_matches_direct_sum() {
    let mut sum = 0.0;
    for n in 1..=10_000 {
        sum += 1.0 / f64::from(n);
        if [2003, 2048, 4096, 10_000].contains(&n) {
            assert!((harmreal(n as u64) - sum).abs() < 1e-12);
        }
    }
}
