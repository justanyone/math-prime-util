<<<<<<< HEAD
//! Product, bound, and repeatability laws for bounded factor methods.
use math_prime_util::{pbrent_factor, pminus1_factor, pplus1_factor, prho_factor};
use proptest::prelude::*;

fn methods(n: u64, bound: u64) -> [Vec<u64>; 4] {
    [
        prho_factor(n, Some(bound)),
        pbrent_factor(n, Some(bound), None),
        pminus1_factor(n, Some(bound), Some(bound.saturating_mul(10))),
        pplus1_factor(n, Some(bound)),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn factors_preserve_product(n in any::<u64>(), bound in 0_u64..256) {
        for factors in methods(n, bound) {
            prop_assert_eq!(factors.iter().map(|&f| u128::from(f)).product::<u128>(), u128::from(n));
            prop_assert!(factors.windows(2).all(|w| w[0] <= w[1]));
            prop_assert!(n < 2 || factors.iter().all(|&f| f > 1 && n % f == 0));
        }
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
}

proptest! {
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
>>>>>>> origin/dev-001
    }
}

#[test]
<<<<<<< HEAD
fn edge_inputs_and_zero_work() {
    for n in [0, 1, 2, 3, 5, 7, 8, 25, 49, 77, 91, u64::MAX] {
        let first = methods(n, 0);
        assert_eq!(first, methods(n, 0));
        for factors in first {
            assert_eq!(
                factors.iter().map(|&f| u128::from(f)).product::<u128>(),
                u128::from(n)
            );
        }
    }
    assert_eq!(prho_factor(77, Some(0)), vec![77]);
    assert_eq!(pbrent_factor(77, Some(0), Some(0)), vec![77]);
}

#[test]
fn concurrent_calls_are_independent() {
    let expected = methods(1_000_036_000_099, 256);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let expected = &expected;
            scope.spawn(move || assert_eq!(&methods(1_000_036_000_099, 256), expected));
        }
    });
=======
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
