//! Algebraic laws and boundary regressions for the factor predicates.
//! Independent algebraic checks for the factor predicates.
use math_prime_util::{
    gcd, is_carmichael, is_cyclic, is_perfect_number, is_practical, is_quasi_carmichael, is_rough,
    is_smooth, smooth_count,
};
use proptest::prelude::*;

fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut p = 2;
    while p <= n / p {
        while n % p == 0 {
            result.push(p);
            n /= p;
        }
        p += 1;
    }
    if n > 1 {
        result.push(n);
    }
    result
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn smooth_count_difference(n in 1u64..2000, k in 0u64..50) {
        prop_assert_eq!(smooth_count(n, k) - smooth_count(n - 1, k),
            u64::from(is_smooth(n, k)));
    }

    #[test]
    fn smooth_and_rough_product(a in 1u64..10_000, b in 1u64..10_000, k in 0u64..100) {
        prop_assert_eq!(is_smooth(a * b, k), is_smooth(a, k) && is_smooth(b, k));
        prop_assert_eq!(is_rough(a * b, k), is_rough(a, k) && is_rough(b, k));
    }

    #[test]
    fn bound_monotonicity(n in any::<u64>(), k in 0u64..5000) {
        prop_assert!(!is_smooth(n, k) || is_smooth(n, k + 1));
        prop_assert!(!is_rough(n, k + 1) || is_rough(n, k));
    }

    #[test]
    fn practical_doubling(n in 1u64..1_000_000) {
        prop_assert!(!is_practical(n) || is_practical(2 * n));
    }

    #[test]
    fn cyclic_iff_coprime_totient(n in 1_u64..100_000) {
        let fs = factors(n);
        let mut phi = n;
        let mut prev = 0;
        for p in fs {
            if p != prev { phi = phi / p * (p - 1); }
            prev = p;
        }
        prop_assert_eq!(is_cyclic(n), gcd(n, phi) == 1);
    }

    #[test]
    fn carmichael_iff_korselt(n in 2_u64..1_000_000) {
        let fs = factors(n);
        let expected = fs.len() > 1
            && fs.windows(2).all(|w| w[0] != w[1])
            && fs.iter().all(|p| (n - 1) % (p - 1) == 0);
        prop_assert_eq!(is_carmichael(n), expected);
    }

    #[test]
    fn perfect_iff_proper_divisor_sum(n in 1_u64..10_000) {
        let sum: u64 = (1..=n / 2).filter(|d| n % d == 0).sum();
        prop_assert_eq!(is_perfect_number(n), sum == n);
    }

    #[test]
    fn quasi_bases_match_integer_base_definition(n in 35_u32..20_000) {
        let n = u64::from(n);
        let fs = factors(n);
        let expected = if fs.len() < 2 || fs.windows(2).any(|w| w[0] == w[1]) {
            0
        } else {
            (-i128::from(n)..i128::from(fs[0]))
                .filter(|&b| b != 0 && fs.iter().all(|&p| (i128::from(n) - b) % (i128::from(p) - b) == 0))
                .count() as u64
        };
        prop_assert_eq!(is_quasi_carmichael(n), expected);
    }
}

#[test]
fn native_boundaries_and_repeated_calls() {
    for k in [0, 1, 2, 3, u64::MAX] {
        assert!(is_smooth(0, k));
        assert!(is_smooth(1, k));
        assert!(is_rough(1, k));
        assert_eq!(is_rough(0, k), k == 0);
        assert_eq!(smooth_count(0, k), 0);
        assert_eq!(smooth_count(1, k), 1);
    }
    for exponent in 0..64 {
        let n = 1u64 << exponent;
        assert!(is_practical(n));
        assert!(is_smooth(n, 2));
    }
    // The divisor sum of this practical number exceeds u64::MAX.
    assert!(is_practical(3u64 << 62));
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..4 {
                    assert_eq!(smooth_count(100, 17), 67);
                    assert!(is_practical(120));
                    assert!(is_smooth(1_000_003 * 1_000_033, 1_000_033));
                    assert!(is_rough(1_000_003 * 1_000_033, 1_000_003));
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}

#[test]
fn practical_matches_subset_sums() {
    for n in 1..=256 {
        let mut sums = vec![false; usize::try_from(n + 1).unwrap()];
        sums[0] = true;
        for d in 1..=n {
            if n % d == 0 {
                for j in (d..=n).rev() {
                    let j = usize::try_from(j).unwrap();
                    sums[j] |= sums[j - usize::try_from(d).unwrap()];
                }
            }
        }
        assert_eq!(is_practical(n), sums.into_iter().all(|b| b), "n={n}");
    }
}

#[test]
fn perfect_numbers_and_u64_boundaries() {
    for n in [
        6,
        28,
        496,
        8128,
        33_550_336,
        8_589_869_056,
        137_438_691_328,
        2_305_843_008_139_952_128,
    ] {
        assert!(is_perfect_number(n), "{n}");
    }
    for n in [0, 1, 2, 4, 12, 24, u64::MAX, u64::MAX - 1, 1 << 63] {
        assert!(!is_perfect_number(n), "{n}");
    }
    assert!(!is_carmichael(u64::MAX));
    assert!(!is_cyclic(u64::MAX));
    assert_eq!(is_quasi_carmichael(0), 0);
}

#[test]
fn quasi_carmichael_includes_negative_bases() {
    // 2093 = 7 * 13 * 23 admits the bases 5 and -1.
    assert_eq!(is_quasi_carmichael(2093), 2);
}

#[test]
fn repeated_and_concurrent_calls_agree() {
    let check = || {
        assert!(is_carmichael(561));
        assert!(!is_carmichael(47 * 53));
        assert!(is_cyclic(47 * 53));
        assert!(!is_cyclic(47 * 47));
        assert_eq!(is_quasi_carmichael(2093), 2);
        assert!(is_perfect_number(8128));
        assert!(is_practical(120));
        assert!(is_smooth(100, 5));
        assert!(is_rough(97, 2));
        assert_eq!(smooth_count(100, 17), 67);
    };
    check();
    std::thread::scope(|scope| {
        let first = scope.spawn(check);
        let second = scope.spawn(check);
        first.join().unwrap();
        second.join().unwrap();
    });
    check();
}
