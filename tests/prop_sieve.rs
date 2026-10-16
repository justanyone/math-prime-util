//! Sieve laws, boundary cases, and concurrent cache use.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn partition_and_sum(lo in 0_u64..10000, width in 0_u64..1000) {
        let hi = lo + width;
        let mid = lo + width / 2;
        let all = primes(lo, hi);
        let mut parts = primes(lo, mid);
        parts.extend(primes(mid + 1, hi));
        prop_assert_eq!(&all, &parts);
        prop_assert_eq!(sum_primes(lo, hi), Some(all.iter().sum()));
        prop_assert_eq!(sieve_range(lo, width + 1, sqrtint(hi)), all.iter().map(|p| p-lo).collect::<Vec<_>>());
        for p in all {
            prop_assert_eq!(next_prime(p-1), Some(p));
            prop_assert_eq!(prev_prime(p+1), Some(p));
        }
    }
    #[test]
    fn clusters_are_prime(lo in 0_u64..10000, width in 0_u64..1000) {
        let expected: Vec<u64> = primes(lo, lo+width).into_iter().filter(|&p| next_prime(p) == Some(p+2)).collect();
        prop_assert_eq!(sieve_prime_cluster(lo, lo+width, &[2]), expected);
    }
}

#[test]
fn concurrent_cache_lifecycle() {
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                for _ in 0..10 {
                    prime_precalc(40000);
                    assert_eq!(primes(32749, 32771), vec![32749, 32771]);
                    prime_memfree();
                    assert_eq!(sum_primes(2, 10), Some(17));
                }
            });
        }
    });
}

#[test]
fn domain_boundaries() {
    assert!(primes(20, 10).is_empty());
    assert_eq!(prev_prime(0), None);
    assert_eq!(next_prime(u64::MAX - 58), None);
    assert_eq!(primes(u64::MAX - 58, u64::MAX), vec![u64::MAX - 58]);
    assert_eq!(sieve_range(u64::MAX, 1, 0), vec![0]);
    assert!(sieve_prime_cluster(u64::MAX - 58, u64::MAX, &[60]).is_empty());
    assert!(std::panic::catch_unwind(|| sieve_range(u64::MAX, 2, 0)).is_err());
    assert!(std::panic::catch_unwind(|| sieve_prime_cluster(0, 20, &[2, 2])).is_err());
    assert!(std::panic::catch_unwind(|| sieve_prime_cluster(0, 20, &[2, 0])).is_err());
    // A failed public call must not damage shared cache state.
    assert_eq!(primes(2, 10), vec![2, 3, 5, 7]);
}
