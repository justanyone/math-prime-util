//! Algebraic laws and boundary cases for twin primes.
use math_prime_util::{is_prime, nth_twin_prime, twin_prime_count, twin_prime_count_approx, twin_primes};
use proptest::prelude::*;

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
    assert!(twin_primes(11, 10).is_empty());
    assert_eq!(twin_primes(11, 11), vec![11]);
    assert_eq!(twin_primes(29, 29), vec![29]);
    assert_eq!(twin_primes(u64::MAX - 844, u64::MAX), vec![u64::MAX - 844]);
    assert!(twin_primes(u64::MAX, u64::MAX).is_empty());
    assert_eq!(nth_twin_prime(u64::MAX), None);
    let pairs = twin_primes(0, 1_000_100);
    assert_eq!(pairs.len() as u64, twin_prime_count(1_000_100));
    assert_eq!(twin_prime_count(10_000_000), 58_980);
    assert_eq!(twin_prime_count(10_000_019), twin_prime_count(10_000_000) + twin_primes(10_000_001, 10_000_019).len() as u64);
}

#[test]
fn correction_intervals_and_repeat_calls() {
    for n in [1999, 2000, 3999, 4000, 7999, 8000, 15999, 16000, 31999, 32000,
        63999, 64000, 127999, 128000, 255999, 256000, 599999, 600000,
        999999, 1_000_000, 3_999_999, 4_000_000, 15_999_999, 16_000_000,
        31_999_999, 32_000_000] {
        assert!(twin_prime_count_approx(n) > 0);
    }
    let handles: Vec<_> = (0..4).map(|_| std::thread::spawn(|| {
        assert_eq!(nth_twin_prime(100), Some(3821));
        assert_eq!(twin_prime_count(1000), 35);
        assert_eq!(twin_primes(29, 31), vec![29]);
    })).collect();
    for handle in handles { handle.join().unwrap(); }
}
