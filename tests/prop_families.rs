//! Algebraic laws for the Ramanujan functions.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn ramanujan_bounds(n in 1_u64..1000) {
        let p = nth_ramanujan_prime(n).unwrap();
        prop_assert!(nth_ramanujan_prime_lower(n).unwrap() <= p);
        prop_assert!(is_prime(p) != 0);
        prop_assert!(ramanujan_prime_count_upper(p) >= n);
        prop_assert!(prime_count(p)-prime_count(p/2) >= n);
        prop_assert!(nth_ramanujan_prime_approx(n).unwrap() >= nth_ramanujan_prime_lower(n).unwrap()-1);
    }
}

#[test]
fn zero_and_repeat_calls() {
    assert_eq!(nth_ramanujan_prime(0), None);
    assert_eq!(nth_ramanujan_prime_lower(0), None);
    assert_eq!(nth_ramanujan_prime_approx(0), None);
    assert_eq!(ramanujan_prime_count_upper(0), 0);
    assert_eq!(nth_ramanujan_prime(u64::MAX), None);
    assert_eq!(nth_ramanujan_prime_lower(u64::MAX), None);
    assert_eq!(nth_ramanujan_prime_approx(u64::MAX), None);
    let handles: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| nth_ramanujan_prime(100)))
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), Some(1439));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]
    #[test]
    fn prime_power_exponent(n in any::<u64>()) {
        let k = is_prime_power(n);
        if k > 0 {
            let p = rootint(n, k);
            prop_assert_eq!(is_prob_prime(p), 2);
            prop_assert_eq!(p.checked_pow(k), Some(n));
        }
    }

    #[test]
    fn prime_power_ranges(lo in 0u64..20_000, width in 0u64..200) {
        let hi = lo + width;
        let expected: Vec<_> = (lo..=hi).filter(|&n| is_prime_power(n) > 0).collect();
        prop_assert_eq!(prime_powers(lo, hi), expected);
    }

    #[test]
    fn prime_power_neighbors(n in 2u64..1_000_000) {
        let next = next_prime_power(n).unwrap();
        prop_assert!(next > n);
        prop_assert!(is_prime_power(next) > 0);
        prop_assert!((n+1..next).all(|v| is_prime_power(v) == 0));
        prop_assert_eq!(prev_prime_power(next + 1), Some(next));
        if let Some(prev) = prev_prime_power(n) {
            prop_assert!(prev < n);
            prop_assert!(is_prime_power(prev) > 0);
            prop_assert!((prev+1..n).all(|v| is_prime_power(v) == 0));
            prop_assert_eq!(next_prime_power(prev - 1), Some(prev));
        }
    }

    #[test]
    fn known_prime_powers(index in 0usize..8, k in 1u32..64) {
        let p: u64 = [2, 3, 5, 7, 11, 13, 17, 101][index];
        if let Some(n) = p.checked_pow(k) {
            prop_assert_eq!(is_prime_power(n), k);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn prime_power_rank_and_bounds(k in 1u64..20_000) {
        let n = nth_prime_power(k).unwrap();
        let count = |x| prime_count(x) + (2..=x.ilog2()).map(|e| prime_count(rootint(x,e))).sum::<u64>();
        prop_assert_eq!(count(n), k);
        prop_assert_eq!(count(n-1), k-1);
        prop_assert!(nth_prime_power_lower(k).unwrap() <= n);
        prop_assert!(nth_prime_power_upper(k).unwrap() >= n);
        prop_assert!(nth_prime_power_approx(k).is_some());
    }
}

#[test]
fn prime_power_boundaries_and_repeated_calls() {
    const MAX_PRIME: u64 = 18_446_744_073_709_551_557;
    assert_eq!(next_prime_power(MAX_PRIME - 1), Some(MAX_PRIME));
    assert_eq!(next_prime_power(MAX_PRIME), None);
    assert_eq!(next_prime_power(u64::MAX), None);
    assert_eq!(prev_prime_power(u64::MAX), Some(MAX_PRIME));
    assert_eq!(prev_prime_power(0), None);
    assert_eq!(prev_prime_power(1), None);
    assert_eq!(prev_prime_power(2), None);
    assert_eq!(prime_powers(0, 1), []);
    assert_eq!(prime_powers(20, 10), []);
    assert_eq!(prime_powers(10, 20), [11, 13, 16, 17, 19]);
    assert_eq!(prime_powers(0, 10), [2, 3, 4, 5, 7, 8, 9]);
    assert_eq!(prime_powers(0, 2), [2]);
    assert_eq!(prime_powers(82, 96), [83, 89]);
    assert_eq!(prime_powers(MAX_PRIME, u64::MAX), [MAX_PRIME]);
    for k in 2..64 {
        let n = 1u64 << k;
        assert_eq!(next_prime_power(n - 1), Some(n));
        assert_eq!(prev_prime_power(n + 1), Some(n));
    }
    // A composite root must not be accepted as a prime root.
    assert_eq!(is_prime_power(121 * 169), 0);
    for p in [3u64, 5, 7] {
        assert_eq!(is_prime_power(p * 11), 0);
    }
    let expected = prime_powers(0, 10_000);
    let threads: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| prime_powers(0, 10_000)))
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), expected);
    }
    assert_eq!(prime_powers(0, 10_000), expected);
}

#[test]
fn prime_power_repeated_and_concurrent_calls() {
    for f in [
        nth_prime_power,
        nth_prime_power_approx,
        nth_prime_power_lower,
        nth_prime_power_upper,
    ] {
        assert_eq!(f(0), None);
        let expected = f(4096);
        let threads: Vec<_> = (0..4)
            .map(|_| std::thread::spawn(move || f(4096)))
            .collect();
        for thread in threads {
            assert_eq!(thread.join().unwrap(), expected);
        }
        assert_eq!(f(4096), expected);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn count_matches_list(n in 0u64..30_000) {
        let pairs = twin_primes(0, n);
        prop_assert_eq!(pairs.len() as u64, twin_prime_count(n));
        prop_assert!(pairs.iter().all(|&p| is_prime(p) != 0 && is_prime(p + 2) != 0));
        prop_assert!(pairs.windows(2).all(|p| p[0] < p[1]));
    }
    #[test]
    fn selection_inverts_count(n in 1u64..1000) {
        let p = nth_twin_prime(n).unwrap();
        prop_assert_eq!(twin_prime_count(p), n);
        prop_assert_eq!(twin_prime_count(p - 1), n - 1);
    }
    #[test]
    fn partition_preserves_pairs(lo in 0u64..5000, width in 0u64..1000) {
        let mid = lo + width / 2;
        let hi = lo + width;
        let mut pairs = twin_primes(lo, mid);
        pairs.extend(twin_primes(mid + 1, hi));
        prop_assert_eq!(pairs, twin_primes(lo, hi));
    }
}

#[test]
fn native_and_segment_boundaries() {
    assert_eq!(twin_primes(11, 10), []);
    assert_eq!(twin_primes(11, 11), vec![11]);
    assert_eq!(twin_primes(29, 29), vec![29]);
    assert_eq!(twin_primes(u64::MAX - 844, u64::MAX), vec![u64::MAX - 844]);
    assert_eq!(twin_primes(u64::MAX, u64::MAX), []);
    assert_eq!(nth_twin_prime(u64::MAX), None);
    let pairs = twin_primes(0, 1_000_100);
    assert_eq!(pairs.len() as u64, twin_prime_count(1_000_100));
    assert_eq!(twin_prime_count(10_000_000), 58_980);
    assert_eq!(
        twin_prime_count(10_000_019),
        twin_prime_count(10_000_000) + twin_primes(10_000_001, 10_000_019).len() as u64
    );
}

#[test]
fn correction_intervals_and_repeat_calls() {
    for n in [
        1999, 2000, 3999, 4000, 7999, 8000, 15_999, 16_000, 31_999, 32_000, 63_999, 64_000,
        127_999, 128_000, 255_999, 256_000, 599_999, 600_000, 999_999, 1_000_000, 3_999_999,
        4_000_000, 15_999_999, 16_000_000, 31_999_999, 32_000_000,
    ] {
        assert!(twin_prime_count_approx(n) > 0);
    }
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                assert_eq!(nth_twin_prime(100), Some(3821));
                assert_eq!(twin_prime_count(1000), 35);
                assert_eq!(twin_primes(29, 31), vec![29]);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn counts_and_sums(n in 1_u64..2000, k in 2_u32..8) {
        let powerful=powerful_numbers(1,n,k);
        prop_assert_eq!(powerful_count(n,k),powerful.len() as u64);
        prop_assert_eq!(sumpowerful(n,k),powerful.iter().map(|&v| u128::from(v)).sum::<u128>());
        let free: Vec<_>=(1..=n).filter(|&i| is_powerfree(i,k)).collect();
        prop_assert_eq!(powerfree_count(n,k),free.len() as u64);
        prop_assert_eq!(powerfree_sum(n,k),free.iter().map(|&v| u128::from(v)).sum::<u128>());
        prop_assert_eq!(powerfree_part_sum(n,k),(1..=n).map(|i| u128::from(powerfree_part(i,k))).sum::<u128>());
    }
    #[test]
    fn parts_and_inverse(n in 1_u64..1000, k in 2_u32..7) {
        let part=powerfree_part(n,k);
        prop_assert!(is_powerfree(part,k));
        prop_assert_eq!(n%part,0);
        prop_assert!(is_power(n/part,k));
        let p=nth_powerful(n,k).unwrap();
        prop_assert_eq!(powerful_count(p,k),n);
        prop_assert!(is_powerful(p,k));
        let q=nth_powerfree(n,k).unwrap();
        prop_assert_eq!(powerfree_count(q,k),n);
        prop_assert!(is_powerfree(q,k));
    }
}

#[test]
fn boundaries_and_repeat_calls() {
    assert!(!is_powerful(0, 0));
    assert!(is_powerful(1, 100));
    assert!(!is_powerful(2, 64));
    assert_eq!(powerful_count(0, 100), 0);
    assert_eq!(powerful_count(10, 100), 1);
    assert_eq!(powerful_numbers(10, 1, 2), Vec::<u64>::new());
    assert_eq!(powerful_numbers(2, 100, 64), Vec::<u64>::new());
    assert_eq!(powerful_numbers(0, 100, 64), vec![1]);
    assert_eq!(sumpowerful(10, 0), 55);
    assert_eq!(nth_powerful(0, 2), None);
    assert_eq!(nth_powerful(2, 64), None);
    assert_eq!(nth_powerful(10, 1), Some(10));
    assert_eq!(nth_powerfree(0, 2), None);
    assert_eq!(nth_powerfree(5, 1), None);
    assert_eq!(nth_powerfree(u64::MAX, 2), None);
    assert_eq!(nth_powerfree(u64::MAX, 64), Some(u64::MAX));
    for k in [0, 1, 2, 3, 64, 100] {
        assert_eq!(powerfree_count(0, k), 0);
        assert_eq!(powerfree_sum(0, k), 0);
        assert_eq!(powerfree_part(0, k), 0);
        assert_eq!(powerfree_part_sum(0, k), 0);
    }
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..3 {
                    assert_eq!(powerful_numbers(1, 10, 2), vec![1, 4, 8, 9]);
                    assert_eq!(powerfree_count(10, 2), 7);
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn root_sum_and_bounds(n in 0_u64..1_000_000) {
        let exact = prime_power_count(n);
        let expected: u64 = (1..=n.checked_ilog2().unwrap_or(0))
            .map(|k| prime_count(rootint(n, k))).sum();
        prop_assert_eq!(exact, expected);
        prop_assert!(prime_power_count_lower(n) <= exact);
        prop_assert!(prime_power_count_upper(n) >= exact);
        prop_assert_eq!(prime_power_count_approx(n), exact);
    }

    #[test]
    fn increments_are_prime_powers(n in 2_u64..10_000) {
        let is_power = (1..=n.ilog2()).any(|k| {
            let r = rootint(n, k);
            r.checked_pow(k) == Some(n) && is_prime(r) != 0
        });
        prop_assert_eq!(prime_power_count(n) - prime_power_count(n - 1), u64::from(is_power));
    }
}

#[test]
fn small_counts_and_repeat_calls() {
    for (n, expected) in [0, 0, 1, 2, 3, 4, 4, 5, 6, 7].into_iter().enumerate() {
        let n = n as u64;
        for _ in 0..2 {
            assert_eq!(prime_power_count(n), expected);
            assert_eq!(prime_power_count_lower(n), expected);
            assert_eq!(prime_power_count_upper(n), expected);
            assert_eq!(prime_power_count_approx(n), expected);
        }
    }
}

#[test]
fn large_estimates_and_concurrent_calls() {
    let inputs = [
        10_000_000_000_000_001,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
        u64::MAX,
    ];
    std::thread::scope(|scope| {
        for n in inputs {
            scope.spawn(move || {
                let estimate = prime_power_count_approx(n);
                assert!(prime_power_count_lower(n) <= estimate);
                assert!(estimate <= prime_power_count_upper(n));
                assert_eq!(prime_power_count_approx(n), estimate);
            });
        }
    });
}
