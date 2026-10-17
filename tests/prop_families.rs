//! Algebraic laws for lucky-number ranks and membership.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn lucky_rank_inverse(k in 1u64..5000) {
        let n = nth_lucky(k).unwrap();
        prop_assert!(is_lucky(n));
        prop_assert_eq!(lucky_count(n), k);
        prop_assert_eq!(lucky_count(n - 1), k - 1);
        prop_assert!(nth_lucky_lower(k).unwrap() <= n);
        prop_assert!(n <= nth_lucky_upper(k).unwrap());
    }
    #[test]
    fn lucky_sieve_membership(n in 0u64..10000) {
        let values = lucky_numbers(0, n);
        prop_assert_eq!(values.len() as u64, lucky_count(n));
        prop_assert_eq!(values.last() == Some(&n), is_lucky(n));
        prop_assert!(values.windows(2).all(|w| w[0] < w[1]));
        prop_assert!(lucky_count_lower(n) <= lucky_count(n));
        prop_assert!(lucky_count(n) <= lucky_count_upper(n));
    }
}

#[test]
fn independent_and_concurrent_calls() {
    let expected = lucky_numbers(0, 300_000);
    let threads: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| lucky_numbers(0, 300_000)))
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), expected);
    }
    assert_eq!(lucky_numbers(10, 1), []);
    assert_eq!(lucky_numbers(0, 0), []);
    assert_eq!(nth_lucky(0), None);
    assert_eq!(nth_lucky_approx(0), None);
    assert_eq!(nth_lucky_lower(0), None);
    assert_eq!(nth_lucky_upper(0), None);
    assert!(!is_lucky(u64::MAX));
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
