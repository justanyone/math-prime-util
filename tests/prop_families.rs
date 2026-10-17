//! Algebraic laws for lucky-number ranks and membership.
use math_prime_util::*;
use proptest::prelude::*;

/// Semiprimes below `limit`, found by counting prime factors with multiplicity.
fn semiprimes_below(limit: u64) -> Vec<u64> {
    (4..limit)
        .filter(|&m| {
            let (mut rest, mut d, mut count) = (m, 2u64, 0u32);
            while d * d <= rest {
                while rest % d == 0 {
                    rest /= d;
                    count += 1;
                }
                d += 1;
            }
            if rest > 1 {
                count += 1;
            }
            count == 2
        })
        .collect()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]
    #[test]
    fn nth_semiprime_is_increasing(k in 1u64..300) {
        let a = nth_semiprime(k).unwrap();
        let b = nth_semiprime(k + 1).unwrap();
        prop_assert!(a < b);
        prop_assert!(nth_semiprime_approx(k).is_some());
    }
    #[test]
    fn nth_semiprime_matches_table(k in 1u64..191) {
        // Independent expected values: semiprimes are numbers with exactly two prime factors.
        let expected = semiprimes_below(700)[k as usize - 1];
        prop_assert_eq!(nth_semiprime(k).unwrap(), expected);
        prop_assert_eq!(nth_semiprime_approx(k).unwrap(), expected);
    }
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
