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
