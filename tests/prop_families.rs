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
