//! Mutation-killing tests for `is_carmichael` and `is_quasi_carmichael`.
use math_prime_util::{is_carmichael, is_prime, is_quasi_carmichael};

fn naive(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= n {
        while n % p == 0 {
            out.push(p);
            n /= p;
        }
        p += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

/// Carmichael numbers above 5,000,000, one divisible by each small prime the
/// early filters name (5 to 43), plus one with none of them.
#[test]
fn large_carmichael_numbers_with_small_prime_factors() {
    for n in [
        5_031_181_u64,
        5_049_001,
        5_148_001,
        5_310_721,
        5_632_705,
        5_968_873,
        6_054_985,
        6_313_681,
        6_840_001,
        9_494_101,
        26_932_081,
        40_280_065,
        56_052_361,
    ] {
        assert!(is_carmichael(n), "{n}");
        assert!(!is_carmichael(n + 2), "{}", n + 2);
        assert!(!is_carmichael(n - 2), "{}", n - 2);
        // Doubling or squaring a prime factor breaks Korselt.
        let p = naive(n)[0];
        assert!(!is_carmichael(n * p), "{}", n * p);
    }
}

/// Chernick numbers (6k+1)(12k+1)(18k+1) across every size class of the
/// splitter (below and above 4e9, up to the u64 range).
#[test]
fn chernick_carmichael_numbers_across_sizes() {
    let mut found = 0;
    for start in [1_u64, 300, 1000, 20_000, 100_000, 200_000, 236_000] {
        let mut hits = 0;
        let mut k = start;
        while hits < 3 && k < 240_000 {
            let (a, b, c) = (6 * k + 1, 12 * k + 1, 18 * k + 1);
            if is_prime(a) != 0 && is_prime(b) != 0 && is_prime(c) != 0 {
                let n = a * b * c;
                assert!(is_carmichael(n), "{n}");
                // One factor removed leaves a semiprime; tweaks break Korselt.
                assert!(!is_carmichael(a * b), "{}", a * b);
                assert!(!is_carmichael(n + 2), "{}", n + 2);
                if let Some(m) = (a * b).checked_mul(a * c) {
                    assert!(!is_carmichael(m), "{m}");
                }
                hits += 1;
                found += 1;
            }
            k += 1;
        }
    }
    assert!(found >= 12);
}

/// Odd composites above 5,000,000 that are not Carmichael: semiprimes and
/// three-prime products that fail Korselt.
#[test]
fn large_non_carmichael_numbers() {
    for n in 5_000_001_u64..5_003_001 {
        let f = naive(n);
        let want = f.len() >= 3
            && f.windows(2).all(|w| w[0] != w[1])
            && f.iter().all(|p| (n - 1) % (p - 1) == 0);
        assert_eq!(is_carmichael(n), want, "{n}");
    }
    assert!(!is_carmichael(1_000_003 * 1_000_033));
    assert!(!is_carmichael(18_446_744_073_709_551_557));
}

fn brute_quasi(n: u64) -> u64 {
    let fs = naive(n);
    if fs.len() < 2 || fs.windows(2).any(|w| w[0] == w[1]) {
        return 0;
    }
    (-i128::from(n)..i128::from(fs[0]))
        .filter(|&b| {
            b != 0
                && fs
                    .iter()
                    .all(|&p| (i128::from(n) - b) % (i128::from(p) - b) == 0)
        })
        .count() as u64
}

#[test]
fn quasi_carmichael_matches_definition() {
    for n in 0_u64..4_000 {
        assert_eq!(is_quasi_carmichael(n), brute_quasi(n), "{n}");
    }
}

#[test]
fn quasi_carmichael_matches_fixtures() {
    let text = include_str!("fixtures/factor.tsv");
    let mut rows = 0;
    for line in text.lines() {
        let mut it = line.split('\t');
        if it.next() != Some("is_quasi_carmichael") {
            continue;
        }
        let Ok(n) = it.next().unwrap().parse::<u64>() else {
            continue;
        };
        let want: u64 = it.next().unwrap().parse().unwrap();
        assert_eq!(is_quasi_carmichael(n), want, "{n}");
        rows += 1;
    }
    assert!(rows > 5000);
}

#[test]
fn quasi_carmichael_edges() {
    // Below 35, multiples of 4 and of small prime squares, primes, squares.
    for n in [
        0, 1, 2, 6, 15, 33, 34, 36, 44, 45, 75, 100, 147, 245, 363, 507,
    ] {
        assert_eq!(is_quasi_carmichael(n), 0, "{n}");
    }
    assert_eq!(is_quasi_carmichael(35), 1);
    assert_eq!(is_quasi_carmichael(2093), 2);
    assert_eq!(is_quasi_carmichael(u64::MAX), 0);
}
