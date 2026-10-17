//! Tests that kill mutants found by `cargo mutants` in the "rest" group:
//! `inverse_sigma0`, `convergents`, `harmreal`, `csrand`, `drand`, `primes`,
//! the prime-count bounds, and the Frobenius pseudoprime tests.

use math_prime_util::{
    convergents, csrand, divisors, drand, harmreal, inverse_sigma0, inverse_sigma0_count, irand64,
    is_frobenius_khashin_pseudoprime, is_frobenius_underwood_pseudoprime, prime_count_approx,
    primes, riemann_r, srand,
};

#[test]
fn inverse_sigma0_reversed_ranges_do_not_underflow() {
    // lo > hi must return before the range width `hi - lo + 1` is formed.
    assert_eq!(inverse_sigma0(4, 10, 5), Some(vec![]));
    assert_eq!(inverse_sigma0_count(4, 10, 5), Some(0));
    assert_eq!(inverse_sigma0_count(2, 10, 4), Some(0));
    assert_eq!(inverse_sigma0(2, 10, 4), Some(vec![]));
    assert_eq!(inverse_sigma0_count(3, 100, 1), Some(0));
}

#[test]
fn inverse_sigma0_k_equal_to_hi() {
    // tau(1) = 1 and tau(2) = 2 are the only n with tau(n) = n.
    assert_eq!(inverse_sigma0(1, 1, 1), Some(vec![1]));
    assert_eq!(inverse_sigma0_count(1, 1, 1), Some(1));
    assert_eq!(inverse_sigma0(2, 2, 2), Some(vec![2]));
    assert_eq!(inverse_sigma0_count(2, 2, 2), Some(1));
    assert_eq!(inverse_sigma0(3, 3, 3), Some(vec![]));
    assert_eq!(inverse_sigma0(4, 4, 4), Some(vec![]));
    assert_eq!(inverse_sigma0(4, 6, 6), Some(vec![6]));
    assert_eq!(inverse_sigma0(4, 8, 8), Some(vec![8]));
}

#[test]
fn inverse_sigma0_wide_range_uses_signatures() {
    // A huge k cannot be met below 2^64. The signature path says so at once;
    // the divisor sieve over such a range would never finish.
    assert_eq!(inverse_sigma0(1 << 40, 1, u64::MAX), Some(vec![]));
    assert_eq!(inverse_sigma0_count(1 << 40, 1, u64::MAX), Some(0));
    // The smallest numbers with 2^20 divisors exceed 2^64 as well.
    assert_eq!(inverse_sigma0_count(1 << 20, 1, u64::MAX), Some(0));
}

#[test]
fn convergents_edges() {
    assert_eq!(convergents(&[]), None);
    assert_eq!(convergents(&vec![1; 129]), None);
    assert_eq!(convergents(&vec![1; 200]), None);
    assert_eq!(convergents(&[3]), Some(vec![(3, 1)]));
    assert_eq!(convergents(&[1, 0]), None);
    // Only the numerator update overflows.
    assert_eq!(convergents(&[u64::MAX, 1]), None);
    // Only the denominator update overflows.
    assert_eq!(convergents(&[0, u64::MAX, 1]), None);
    // The largest terms that still fit.
    assert_eq!(
        convergents(&[0, u64::MAX]),
        Some(vec![(0, 1), (1, u64::MAX)])
    );
}

#[allow(clippy::cast_precision_loss)]
fn pairwise_harmonic(a: u64, b: u64) -> f64 {
    if b - a == 1 {
        return 1.0 / a as f64;
    }
    let m = a + (b - a) / 2;
    pairwise_harmonic(a, m) + pairwise_harmonic(m, b)
}

#[test]
fn harmreal_asymptotic_branch_matches_the_sum() {
    for n in [2_003_u64, 2_500, 5_000, 20_000, 100_000, 1_000_000] {
        let want = pairwise_harmonic(1, n + 1);
        let got = harmreal(n);
        assert!((got - want).abs() < 1e-13, "n={n}: {got} vs {want}");
    }
    // Boundary between the exact split and the series.
    for n in [2_001_u64, 2_002] {
        let want = pairwise_harmonic(1, n + 1);
        assert!((harmreal(n) - want).abs() < 1e-13, "n={n}");
    }
    assert_eq!(harmreal(0), 0.0);
    // H_10000 to 16 digits.
    assert!((harmreal(10_000) - 9.787_606_036_044_382).abs() < 1e-13);
}

#[test]
fn csrand_accepts_seeds_of_every_length() {
    let seed40: Vec<u8> = (1..=40).collect();
    let seed41: Vec<u8> = (1..=41).collect();
    let seed39: Vec<u8> = (1..=39).collect();
    csrand(&seed40);
    let a = (irand64(), irand64());
    csrand(&seed41);
    let b = (irand64(), irand64());
    csrand(&seed40);
    let c = (irand64(), irand64());
    csrand(&seed39);
    let d = (irand64(), irand64());
    // Bytes beyond 40 are ignored; a shorter seed is stretched, not padded.
    assert_eq!(a, b);
    assert_eq!(a, c);
    assert_ne!(a, d);
}

#[test]
fn drand_is_a_scaled_unit_draw() {
    srand(15);
    let raw = irand64();
    srand(15);
    let unit = drand(1.0);
    #[allow(clippy::cast_precision_loss)]
    let want = raw as f64 * (1.0 / 18_446_744_073_709_551_616.0);
    assert!(unit > 0.0 && unit < 1.0);
    assert_eq!(unit, want);
    srand(15);
    assert_eq!(drand(0.0), want);
    srand(15);
    assert_eq!(drand(8.0), want * 8.0);
}

#[test]
fn primes_empty_ranges() {
    assert_eq!(primes(20, 10), Vec::<u64>::new());
    assert_eq!(primes(5, 3), Vec::<u64>::new());
    assert_eq!(primes(0, 1), Vec::<u64>::new());
    assert_eq!(primes(1, 1), Vec::<u64>::new());
    assert_eq!(primes(0, 0), Vec::<u64>::new());
    assert_eq!(primes(100, 1), Vec::<u64>::new());
    assert_eq!(primes(2, 2), vec![2]);
}

#[test]
fn divisors_with_limit_one() {
    assert_eq!(divisors(1, None), vec![1]);
    assert_eq!(divisors(1, Some(5)), vec![1]);
    assert_eq!(divisors(12, Some(1)), vec![1]);
    assert_eq!(divisors(12, Some(0)), Vec::<u64>::new());
    assert_eq!(divisors(0, None), Vec::<u64>::new());
}

#[test]
fn prime_count_approx_switches_at_three_million() {
    let at = prime_count_approx(3_000_000);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let want = (riemann_r(3_000_000.0) + 0.5) as u64;
    assert_eq!(at, want);
}

#[test]
fn khashin_rejects_large_prime_square_quickly() {
    // A square would otherwise search for a Jacobi symbol of -1 up to the root.
    assert!(!is_frobenius_khashin_pseudoprime(
        4_294_967_291 * 4_294_967_291
    ));
    assert!(!is_frobenius_khashin_pseudoprime(1_000_003 * 1_000_003));
    assert!(is_frobenius_khashin_pseudoprime(4_294_967_291));
}

#[test]
fn underwood_accepts_primes_with_late_parameter() {
    // For these primes the parameter search passes x = 20 without a Jacobi
    // symbol of -1; they are prime, so the test must accept them.
    for p in [53_881_u64, 87_481, 92_569, 102_001, 130_729, 144_169] {
        assert!(is_frobenius_underwood_pseudoprime(p), "{p}");
    }
    assert!(!is_frobenius_underwood_pseudoprime(53_881 * 53_881));
    assert!(!is_frobenius_underwood_pseudoprime(7 * 7));
}

/// Values at each branch threshold, where `<` and `<=` pick different formulas.
#[test]
fn prime_count_lower_at_branch_thresholds() {
    use math_prime_util::prime_count_lower;
    for (n, want) in [
        (33_000_u64, 3_531_u64),
        (69_720, 6_909),
        (70_120, 6_947),
        (88_800, 8_599),
        (176_000, 15_990),
        (299_270, 25_938),
        (300_070, 26_001),
        (300_071, 25_999),
        (303_000, 26_234),
        (1_100_000, 85_696),
        (4_500_000, 315_864),
        (10_200_000, 676_945),
        (36_900_000, 2_255_822),
        (38_100_000, 2_324_684),
        (4_000_000_000, 189_958_870),
    ] {
        assert_eq!(prime_count_lower(n), want, "n={n}");
    }
}

#[test]
fn prime_count_upper_at_branch_thresholds() {
    use math_prime_util::prime_count_upper;
    for (n, want) in [
        (33_000_u64, 3_549_u64),
        (59_000, 5_972),
        (355_991, 30_456),
        (3_550_000, 253_554),
        (3_560_000, 254_207),
        (5_000_000, 348_529),
        (8_000_000, 539_869),
        (13_000_000, 849_324),
        (18_000_000, 1_151_380),
        (31_000_000, 1_916_016),
        (41_000_000, 2_490_825),
        (48_000_000, 2_888_253),
        (119_000_000, 6_788_286),
        (182_000_000, 10_135_593),
        (192_000_000, 10_660_511),
        (213_000_000, 11_758_402),
        (271_000_000, 14_763_234),
        (322_000_000, 17_377_669),
        (400_000_000, 21_336_489),
        (510_000_000, 26_854_939),
        (682_000_000, 35_368_902),
        (821_800_000, 42_209_845),
        (1_100_000_000, 55_662_749),
        (2_953_652_287, 142_326_789),
        (10_010_000_000, 455_487_974),
        (101_260_000_000, 4_167_800_541),
    ] {
        assert_eq!(prime_count_upper(n), want, "n={n}");
    }
}
