//! Laws for Ramanujan ranges and counts.
use math_prime_util::{
    is_prime, prime_count, ramanujan_prime_count, ramanujan_prime_count_lower, ramanujan_primes,
};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn ranges_match_counts(a in 0_u64..5000, b in 0_u64..5000) {
        let lo=a.min(b); let hi=a.max(b);
        let values=ramanujan_primes(lo,hi);
        prop_assert_eq!(values.len() as u64, ramanujan_prime_count(hi)-ramanujan_prime_count(lo.saturating_sub(1)));
        prop_assert!(values.windows(2).all(|w| w[0]<w[1]));
        for p in values {prop_assert_ne!(is_prime(p),0);}
        prop_assert!(ramanujan_prime_count_lower(hi)<=ramanujan_prime_count(hi));
    }
    #[test]
    fn defining_inequality(n in 2_u64..3000, extra in 0_u64..1000) {
        let count=ramanujan_prime_count(n);
        let x=n+extra;
        prop_assert!(prime_count(x)-prime_count(x/2)>=count);
    }
}

#[test]
fn empty_and_singleton_ranges() {
    assert!(ramanujan_primes(10, 2).is_empty());
    assert!(ramanujan_primes(0, 1).is_empty());
    assert!(ramanujan_primes(242, 250).is_empty());
    assert_eq!(ramanujan_primes(263, 263), vec![263]);
}

#[test]
fn repeat_and_concurrent_calls() {
    let expected = ramanujan_primes(250, 1000);
    let threads: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| ramanujan_primes(250, 1000)))
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), expected);
    }
    assert_eq!(ramanujan_primes(250, 1000), expected);
    for k in 0_u32..=56 {
        assert!(ramanujan_prime_count(1_u64 << k) <= 1_u64 << k);
    }
}
