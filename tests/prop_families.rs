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
    #![proptest_config(ProptestConfig::with_cases(32))]
    #[test]
    fn almost_rank_and_range(k in 1u32..8, rank in 1u64..60) {
        let n = nth_almost_prime(k, rank).unwrap();
        prop_assert_eq!(almost_prime_count(k,n),rank);
        prop_assert_eq!(almost_prime_count(k,n-1),rank-1);
        prop_assert_eq!(almost_primes(k,n,n), vec![n]);
        prop_assert!(nth_almost_prime_lower(k,rank).unwrap() <= n);
        prop_assert!(nth_almost_prime_upper(k,rank).unwrap() >= n);
        prop_assert!(almost_prime_count_lower(k,n) <= rank);
        prop_assert!(almost_prime_count_upper(k,n) >= rank);
    }
    #[test]
    fn omega_rank_and_range(k in 1u32..5, rank in 1u64..60) {
        let n = nth_omega_prime(k,rank).unwrap();
        prop_assert_eq!(omega_prime_count(k,n),rank);
        prop_assert_eq!(omega_prime_count(k,n-1),rank-1);
        prop_assert_eq!(omega_primes(k,0,n).len() as u64,rank);
    }
    #[test]
    fn factor_multiplicities(k in 0u32..7, hi in 0u64..2000) {
        let a=almost_primes(k,0,hi);
        let w=omega_primes(k,0,hi);
        prop_assert_eq!(a.len() as u64,almost_prime_count(k,hi));
        prop_assert_eq!(w.len() as u64,omega_prime_count(k,hi));
        prop_assert!(a.windows(2).all(|v| v[0]<v[1]));
        prop_assert!(w.windows(2).all(|v| v[0]<v[1]));
        if k==1 { prop_assert!(a.iter().all(|v| w.contains(v))); }
    }
}

#[test]
fn almost_omega_boundaries_and_concurrency() {
    for k in [0, 1, 2, 63, 64, u32::MAX] {
        assert_eq!(almost_primes(k, 10, 1), []);
        assert_eq!(omega_primes(k, 10, 1), []);
        assert_eq!(almost_primes(k, 0, 0), []);
        assert_eq!(omega_primes(k, 0, 0), []);
        assert_eq!(nth_almost_prime(k, 0), None);
        assert_eq!(nth_almost_prime_lower(k, 0), None);
        assert_eq!(nth_almost_prime_upper(k, 0), None);
        assert_eq!(nth_almost_prime_approx(k, 0), None);
        assert_eq!(nth_omega_prime(k, 0), None);
        assert_eq!(almost_prime_count(k, 0), 0);
        assert_eq!(omega_prime_count(k, 0), 0);
    }
    assert_eq!(nth_almost_prime(0, 2), None);
    assert_eq!(nth_omega_prime(0, 2), None);
    assert_eq!(nth_omega_prime(16, 1), None);
    assert_eq!(nth_omega_prime(15, 299_179), None);
    assert_eq!(nth_omega_prime(1, u64::MAX), None);
    assert_eq!(almost_prime_count(64, u64::MAX), 0);
    assert_eq!(almost_prime_count_approx(64, u64::MAX), 0);
    assert_eq!(almost_prime_count_lower(64, u64::MAX), 0);
    assert_eq!(almost_prime_count_upper(64, u64::MAX), 0);
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (
                    almost_primes(3, 0, 2000),
                    omega_primes(3, 0, 2000),
                    nth_almost_prime(3, 100),
                    nth_omega_prime(3, 100),
                )
            })
        })
        .collect();
    let expected = (
        almost_primes(3, 0, 2000),
        omega_primes(3, 0, 2000),
        nth_almost_prime(3, 100),
        nth_omega_prime(3, 100),
    );
    for thread in threads {
        assert_eq!(thread.join().unwrap(), expected);
    }
    assert_eq!(almost_primes(3, 0, 2000), expected.0);
    assert_eq!(omega_primes(3, 0, 2000), expected.1);
}

#[test]
fn large_prime_estimates_stay_in_their_bounds() {
    for n in [1u64 << 53, 1u64 << 60, 10_000_000_000_000_000_000, u64::MAX] {
        let estimate = almost_prime_count_approx(1, n);
        assert!(almost_prime_count_lower(1, n) <= estimate);
        assert!(estimate <= almost_prime_count_upper(1, n));
        assert!(almost_prime_count_approx(1, n / 2) < estimate);
    }
}
