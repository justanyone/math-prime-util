//! Algebraic laws for bounded factor methods and the factor predicates.
use math_prime_util::{
    fermat_factor, gcd, holf_factor, is_carmichael, is_cyclic, is_perfect_number,
    is_quasi_carmichael, lehman_factor, squfof_factor,
};
use proptest::prelude::*;

type Method = fn(u64, Option<u64>) -> Vec<u64>;
const METHODS: [Method; 3] = [fermat_factor, holf_factor, squfof_factor];

fn validate(n: u64, factors: &[u64]) {
    assert_eq!(
        factors.iter().map(|&f| u128::from(f)).product::<u128>(),
        u128::from(n)
    );
    if n > 1 {
        assert!(factors.iter().all(|&f| f > 1 && n % f == 0));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]
    #[test]
    fn product_is_input(n in any::<u64>(), rounds in 0u64..100) {
        for method in METHODS { validate(n, &method(n, Some(rounds))); }
        validate(n, &lehman_factor(n, Some(false)));
        validate(n, &lehman_factor(n, Some(true)));
    }
    #[test]
    fn bounded_small_products(a in 1u64..1000, b in 1u64..1000) {
        let n = a*b;
        for method in METHODS { validate(n, &method(n, Some(1000))); }
        validate(n, &lehman_factor(n, None));
    }
}

#[test]
fn edge_inputs_and_zero_budget() {
    for method in METHODS {
        assert_eq!(method(0, Some(0)), vec![0]);
        assert_eq!(method(1, Some(0)), Vec::<u64>::new());
        assert_eq!(method(2 * 3 * 5 * 7, Some(0)), vec![2, 3, 5, 7]);
        assert_eq!(method(77, Some(0)), vec![77]);
        assert_eq!(method(49, None), vec![7, 7]);
        validate(u64::MAX, &method(u64::MAX, Some(1)));
    }
    assert_eq!(lehman_factor(0, None), vec![0]);
    assert_eq!(lehman_factor(1, None), Vec::<u64>::new());
    assert_eq!(lehman_factor(77, Some(true)), vec![7, 11]);
    assert_eq!(lehman_factor(89_u64.pow(3), None), vec![89, 89, 89]);
    assert_eq!(fermat_factor(49, Some(0)), vec![7, 7]);
    assert_eq!(holf_factor(49, Some(0)), vec![7, 7]);
    assert_eq!(squfof_factor(49, Some(0)), vec![49]);
    assert_eq!(
        squfof_factor(9_223_371_873_002_223_329, None),
        vec![9_223_371_873_002_223_329]
    );
}

#[test]
fn concurrent_repeated_calls_have_no_shared_state() {
    std::thread::scope(|scope| {
        for method in METHODS {
            scope.spawn(move || {
                for _ in 0..4 {
                    assert_eq!(method(8051, None), vec![83, 97]);
                }
            });
        }
        scope.spawn(|| {
            for _ in 0..4 {
                assert_eq!(lehman_factor(8051, None), vec![83, 97]);
            }
        });
    });
}

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
