//! Algebraic laws for factor counts, signatures, partial factoring, and predicates.
use math_prime_util::{
    cheb_factor, gcd, is_carmichael, is_cyclic, is_perfect_number, is_quasi_carmichael,
    prime_bigomega, prime_omega, prime_signature,
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
    fn signature_counts(n in 1_u64..1_000_000) {
        let signature = prime_signature(n);
        prop_assert_eq!(signature.iter().sum::<u32>(), prime_bigomega(n));
        prop_assert_eq!(signature.len(), usize::try_from(prime_omega(n)).unwrap());
        prop_assert!(signature.windows(2).all(|pair| pair[0] >= pair[1]));
    }

    #[test]
    fn multiplicity_is_additive(a in 1_u64..100_000, b in 1_u64..100_000) {
        prop_assert_eq!(prime_bigomega(a * b), prime_bigomega(a) + prime_bigomega(b));
        prop_assert!(prime_omega(a * b) <= prime_omega(a) + prime_omega(b));
        if gcd(a, b) == 1 {
            prop_assert_eq!(prime_omega(a * b), prime_omega(a) + prime_omega(b));
        }
    }

    #[test]
    fn chebyshev_preserves_product(n in 1_u64..10_000_000, bound in 0_u64..200, x in any::<u64>()) {
        let factors = cheb_factor(n, bound, x);
        prop_assert_eq!(factors.iter().product::<u64>(), n);
        prop_assert!(factors.windows(2).all(|pair| pair[0] <= pair[1]));
        prop_assert!(factors.iter().all(|&f| f > 1));
    }

    #[test]
    fn cyclic_iff_coprime_totient(n in 1_u64..100_000) {
        let fs = factors(n);
        let mut phi = n;
        let mut prev = 0;
        for p in fs {
            if p != prev {
                phi = phi / p * (p - 1);
            }
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
fn edges_and_repeated_calls() {
    for _ in 0..2 {
        assert_eq!(prime_signature(0), vec![1]);
        assert_eq!(prime_signature(1), Vec::<u32>::new());
        assert_eq!(prime_omega(0), 1);
        assert_eq!(prime_bigomega(1), 0);
        assert_eq!(prime_signature(1_u64 << 63), vec![63]);
        assert_eq!(cheb_factor(0, 0, 0), vec![0]);
        assert_eq!(cheb_factor(1, 0, 0), Vec::<u64>::new());
        assert_eq!(cheb_factor(143, 1, u64::MAX), vec![143]);
        assert_eq!(cheb_factor(143, u64::MAX, 1), vec![143]);
        assert_eq!(cheb_factor(143, 0, 143), vec![143]);
    }
}

#[test]
fn concurrent_calls_are_independent() {
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                assert_eq!(prime_signature(360), vec![3, 2, 1]);
                assert_eq!(prime_bigomega(360), 6);
                assert_eq!(prime_omega(360), 3);
                assert_eq!(cheb_factor(403, 0, 0), vec![13, 31]);
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
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
}
