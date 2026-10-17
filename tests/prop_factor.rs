//! Algebraic laws for the factor family. Factors from the bounded methods need not be prime.
use math_prime_util::{
    cheb_factor, divisors, factor, factor_exp, factor_u128, fermat_factor, gcd, holf_factor,
    is_almost_prime, is_carmichael, is_cyclic, is_omega_prime, is_perfect_number, is_practical,
    is_prime, is_prime_u128, is_quasi_carmichael, is_rough, is_semiprime, is_smooth,
    is_square_free, lehman_factor, pbrent_factor, pminus1_factor, pplus1_factor, prho_factor,
    prime_bigomega, prime_omega, prime_signature, rough_count, smooth_count, squarefree_kernel,
    squfof_factor, trial_factor,
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
    }
}

#[test]
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
        if math_prime_util::gcd(a, b) == 1 {
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
fn concurrent_omega_calls_are_independent() {
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
}

proptest! {
    #[test]
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
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn factor_product(n in any::<u64>()) {
        let factors = factor(n);
        prop_assert!(factors.windows(2).all(|w| w[0] <= w[1]));
        prop_assert_eq!(factors.iter().map(|&p| u128::from(p)).product::<u128>(), u128::from(n));
        if n > 1 {
            prop_assert!(factors.iter().all(|&p| is_prime(p) != 0));
        }
        let expanded: Vec<_> = factor_exp(n).into_iter()
            .flat_map(|(p, e)| (0..e).map(move |_| p)).collect();
        prop_assert_eq!(expanded, factors);
    }

    #[test]
    fn divisor_laws(n in 1u64..=1_000_000, max in 0u64..=1_000_000) {
        let all = divisors(n, None);
        prop_assert_eq!(all.first(), Some(&1));
        prop_assert_eq!(all.last(), Some(&n));
        prop_assert!(all.windows(2).all(|w| w[0] < w[1]));
        prop_assert!(all.iter().all(|d| n % d == 0));
        for (a, b) in all.iter().zip(all.iter().rev()) {
            prop_assert_eq!(a * b, n);
        }
        let expected: Vec<_> = all.into_iter().filter(|&d| d <= max).collect();
        prop_assert_eq!(divisors(n, Some(max)), expected);
    }

    #[test]
    fn bounded_trial_product(n in any::<u64>(), limit in 1u64..=10_000) {
        let factors = trial_factor(n, limit);
        prop_assert_eq!(factors.iter().map(|&p| u128::from(p)).product::<u128>(), u128::from(n));
        if factors.len() > 1 {
            prop_assert!(factors[..factors.len()-1].iter().all(|&p| p <= limit && is_prime(p) != 0));
        }
    }
}

#[test]
fn boundaries_and_repeated_calls() {
    assert_eq!(factor(0), [0]);
    assert_eq!(factor(1), Vec::<u64>::new());
    assert_eq!(factor_exp(0), [(0, 1)]);
    assert_eq!(factor_exp(1), Vec::<(u64, u32)>::new());
    assert_eq!(divisors(0, None), Vec::<u64>::new());
    assert_eq!(divisors(360, Some(0)), Vec::<u64>::new());
    assert_eq!(divisors(360, Some(1)), [1]);
    assert_eq!(divisors(1, None), [1]);
    assert_eq!(trial_factor(49, 6), [49]);
    assert_eq!(trial_factor(49, 7), [7, 7]);
    assert_eq!(trial_factor(4, 1), [4]);
    assert_eq!(trial_factor(2017 * 2017, 0), [2017, 2017]);
    for n in [1, 0, 360, 4_294_967_291, u64::MAX, 360] {
        let expected = factor(n);
        std::thread::scope(|scope| {
            for _ in 0..4 {
                let expected = &expected;
                scope.spawn(move || assert_eq!(&factor(n), expected));
            }
        });
    }
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
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    #[test]
    fn u128_paths_agree_with_u64(n in any::<u64>()) {
        let wide: Vec<u64> = factor_u128(u128::from(n)).unwrap()
            .into_iter().map(|p| u64::try_from(p).unwrap()).collect();
        prop_assert_eq!(wide, factor(n));
        prop_assert_eq!(is_prime_u128(u128::from(n)), is_prime(n));
    }

    #[test]
    fn u128_factors_multiply_back(a in 2u64..(1 << 40), b in 2u64..(1 << 40), c in 1u64..1000) {
        let n = u128::from(a) * u128::from(b) * u128::from(c);
        let factors = factor_u128(n).unwrap();
        prop_assert!(factors.windows(2).all(|w| w[0] <= w[1]));
        prop_assert_eq!(factors.iter().product::<u128>(), n);
        prop_assert!(factors.iter().all(|&p| is_prime_u128(p) != 0));
    }

    #[test]
    fn u128_primality_matches_factoring(n in any::<u128>()) {
        // A probable prime must not factor; a composite must.
        let prime = is_prime_u128(n) != 0;
        if n >> 100 == 0 || prime {
            let factors = factor_u128(n).unwrap();
            prop_assert_eq!(prime, factors.len() == 1 && n > 1);
        }
    }
}

#[test]
fn rough_count_is_bounded_by_n() {
    for n in 0..200 {
        for k in 0..12 {
            assert!(rough_count(n, k) <= n, "n={n} k={k}");
        }
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
fn rough_count_is_monotone_in_n() {
    for k in 0..10 {
        for n in 0..200 {
            assert!(rough_count(n, k) <= rough_count(n + 1, k), "n={n} k={k}");
        }
    }
}

#[test]
fn rough_count_grows_as_k_shrinks() {
    // Fewer excluded primes never lowers the count.
    for n in 0..150 {
        for k in 3..12 {
            assert!(rough_count(n, k) <= rough_count(n, k - 1), "n={n} k={k}");
        }
    }
}

#[test]
fn squarefree_kernel_divides_n() {
    for n in 1..2000 {
        assert_eq!(n % squarefree_kernel(n), 0, "n={n}");
    }
}

#[test]
fn squarefree_kernel_is_idempotent() {
    for n in 0..2000 {
        let s = squarefree_kernel(n);
        assert_eq!(squarefree_kernel(s), s, "n={n}");
    }
}

#[test]
fn squarefree_kernel_is_multiplicative_on_coprimes() {
    // gcd(a, b) = 1 gives kernel(a * b) = kernel(a) * kernel(b).
    for a in 1..60u64 {
        for b in 1..60u64 {
            if gcd(a, b) == 1 {
                assert_eq!(
                    squarefree_kernel(a * b),
                    squarefree_kernel(a) * squarefree_kernel(b),
                    "a={a} b={b}"
                );
            }
        }
    }
}
