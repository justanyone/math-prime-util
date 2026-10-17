//! Algebraic laws for twin estimates and semiprimes.
use math_prime_util::{
    is_prime, nth_twin_prime_approx, semi_primes, semiprime_count, semiprime_count_approx,
};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn semiprime_list_count(lo in 0u64..2000, width in 0u64..200) {
        let hi = lo + width;
        let values = semi_primes(lo, hi);
        let before = lo.checked_sub(1).map_or(0, semiprime_count);
        prop_assert_eq!(values.len() as u64, semiprime_count(hi)-before);
        prop_assert!(values.windows(2).all(|w| w[0] < w[1]));
        for n in values {
            prop_assert!((2..=math_prime_util::sqrtint(n)).any(|p| n%p == 0 && is_prime(p) != 0 && is_prime(n/p) != 0));
        }
    }
    #[test]
    fn small_approx_is_exact(n in 0u64..=623) {
        prop_assert_eq!(semiprime_count_approx(n), semiprime_count(n));
    }
    #[test]
    fn twin_estimate_grows(n in 6u64..100_000) {
        prop_assert!(nth_twin_prime_approx(n).unwrap() <= nth_twin_prime_approx(n+1).unwrap());
    }
}

#[test]
fn empty_ranges_and_repeat_calls() {
    assert!(semi_primes(15, 4).is_empty());
    assert!(semi_primes(0, 3).is_empty());
    assert!(semi_primes(u64::MAX, u64::MAX).is_empty());
    let expected = semi_primes(0, 2000);
    let workers: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| semi_primes(0, 2000)))
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}
