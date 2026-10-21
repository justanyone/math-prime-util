//! Regression tests for lucky, Ramanujan-upper, almost-prime, and harmonic
//! estimates. Each test uses only the public API.

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
    // k = 3 uses interpolate_approx; k = 2 uses the semiprime path.
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
    // Public callers use SMALL_LUCKY_COUNT here, bypassing simple_approx.
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
    for n in 1_u32..=10_000 {
        sum += 1.0 / f64::from(n);
        if [2003, 2048, 4096, 10_000].contains(&n) {
            assert!((harmreal(u64::from(n)) - sum).abs() < 1e-12);
        }
    }
}

#[test]
fn ramanujan_upper_at_axler_tail_boundaries() {
    // Indices from MPU prime_counts.c::ramanujan_axler with C = 0.
    // ramanujan_primes.c::nth_ramanujan_prime_upper selects D = -2.0884
    // through 3_999_654_659, then D = -2.235.
    for (n, index) in [
        (512_000_001, 1_057_141_056),
        (3_999_654_658, 8_234_647_401),
        (3_999_654_659, 8_234_647_403),
        (3_999_654_660, 8_234_549_410),
    ] {
        assert_eq!(
            nth_ramanujan_prime_upper(n),
            math_prime_util::nth_prime_upper(index),
            "n={n}, Axler index={index}"
        );
    }
}

#[test]
fn ramanujan_upper_at_prime_bound_transition() {
    // MPU prime_counts.c::ramanujan_axler(7140, 0.1212, 0.7973)
    // truncates to 15360. nth_prime_upper switches formula at this index.
    assert_eq!(
        nth_ramanujan_prime_upper(7140),
        math_prime_util::nth_prime_upper(15_360)
    );
}

#[test]
fn lucky_estimates_at_every_fit_boundary() {
    // MPU lucky_numbers.c::_simple_lucky_count_approx supplies the lower
    // search endpoint. Check all public estimates on both sides of each fit.
    for n in [
        999_999,
        1_000_000,
        1_000_001,
        9_999_999,
        10_000_000,
        10_000_001,
        99_999_999,
        100_000_000,
        100_000_001,
        3_999_999_999,
        4_000_000_000,
        4_000_000_001,
    ] {
        let lower = lucky_count_lower(n);
        let approx = lucky_count_approx(n);
        let upper = lucky_count_upper(n);
        assert!(lower <= approx, "n={n}: lower={lower}, approx={approx}");
        assert!(approx <= upper, "n={n}: approx={approx}, upper={upper}");
    }
}

#[test]
fn fourth_almost_primes_match_independent_factor_counts() {
    // MPU almost_primes.c::nth_almost_prime also uses interpolation for k=4.
    let omega = omega_table(4_000_000);
    for n in [8, 20_000, 100_000, 200_000, 400_000] {
        assert_eq!(
            nth_almost_prime(4, n as u64),
            Some(brute_nth_almost(&omega, 4, n)),
            "n={n}"
        );
    }
}

#[test]
fn lucky_estimates_keep_their_order_across_native_scales() {
    // The simple fit only seeds inverse interpolation. Exercise seeds at
    // geometric scales and adjacent integers rather than just decimal edges.
    let mut n = 9_001_u64;
    while n < 18_000_000_000_000_000_000 {
        for x in [n - 1, n, n + 1] {
            let lower = lucky_count_lower(x);
            let approx = lucky_count_approx(x);
            let upper = lucky_count_upper(x);
            assert!(lower <= approx, "n={x}: lower={lower}, approx={approx}");
            assert!(approx <= upper, "n={x}: approx={approx}, upper={upper}");
        }
        n += n / 8;
    }
}
