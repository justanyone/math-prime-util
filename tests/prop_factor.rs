<<<<<<< HEAD
//! Independent factor counts and algebraic laws for factor predicates.
use math_prime_util::{is_almost_prime, is_omega_prime, is_semiprime, is_square_free};
use proptest::prelude::*;

fn counts(mut n: u64) -> (u64, u32, bool) {
    let (mut total, mut distinct, mut square_free) = (0, 0, n != 0);
    let mut p = 2;
    while n > 1 && p <= n / p {
        let mut e = 0;
        while n % p == 0 {
            n /= p;
            e += 1;
        }
        total += e;
        distinct += u32::from(e > 0);
        square_free &= e < 2;
        p += 1;
    }
    if n > 1 {
        total += 1;
        distinct += 1;
    }
    (total, distinct, square_free)
=======
//! Independent algebraic checks for the factor predicates.
use math_prime_util::{gcd, is_carmichael, is_cyclic, is_perfect_number, is_quasi_carmichael};
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
>>>>>>> origin/dev-001
}

proptest! {
    #[test]
<<<<<<< HEAD
    fn predicates_match_independent_counts(n in 1u64..1_000_000, k in 0u32..25) {
        let (total, distinct, square_free) = counts(n);
        prop_assert_eq!(is_almost_prime(u64::from(k), n), total == u64::from(k));
        prop_assert_eq!(is_omega_prime(k, n), distinct == k);
        prop_assert_eq!(is_square_free(n), square_free);
        prop_assert_eq!(is_semiprime(n), total == 2);
        prop_assert!(is_almost_prime(total, n));
        prop_assert!(is_omega_prime(distinct, n));
    }

    #[test]
    fn multiplication_adds_multiplicity(a in 1u64..10000, b in 1u64..10000) {
        let (ka, _, _) = counts(a);
        let (kb, _, _) = counts(b);
        prop_assert!(is_almost_prime(ka + kb, a * b));
    }

    #[test]
    fn squares_are_not_square_free(n in 2u64..=u64::from(u32::MAX)) {
        prop_assert!(!is_square_free(n * n));
    }
}

#[test]
fn boundaries_and_large_counts() {
    for n in 0..=4096 {
        let (total, distinct, square_free) = counts(n);
        assert_eq!(is_square_free(n), square_free, "n={n}");
        assert_eq!(is_semiprime(n), n != 0 && total == 2, "n={n}");
        for k in 0..=15 {
            assert_eq!(
                is_almost_prime(u64::from(k), n),
                n != 0 && total == u64::from(k)
            );
            assert_eq!(is_omega_prime(k, n), n != 0 && distinct == k);
        }
    }
    assert!(is_almost_prime(63, 1 << 63));
    assert!(!is_almost_prime(64, u64::MAX));
    assert!(!is_almost_prime(u64::MAX, u64::MAX));
    assert!(!is_omega_prime(u32::MAX, u64::MAX));
    assert!(is_omega_prime(1, 13_u64.pow(16)));
    assert!(!is_omega_prime(1, 13 * 17));
}

#[test]
fn repeated_and_concurrent_calls() {
    let inputs = [0, 1, 4, 30, 49, 1_000_003 * 1_000_033, u64::MAX];
    let expected: Vec<_> = inputs.map(|n| (is_square_free(n), is_semiprime(n))).into();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for (n, expected) in inputs.into_iter().zip(&expected) {
                    assert_eq!((is_square_free(n), is_semiprime(n)), *expected);
                }
            });
        }
    });
=======
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
    };
    check();
    std::thread::scope(|scope| {
        let first = scope.spawn(check);
        let second = scope.spawn(check);
        first.join().unwrap();
        second.join().unwrap();
    });
    check();
>>>>>>> origin/dev-001
}
