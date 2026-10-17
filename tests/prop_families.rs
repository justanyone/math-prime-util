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

fn semiprime(mut n: u64) -> bool {
    let mut factors = 0;
    let mut p = 2;
    while p <= n / p {
        while n % p == 0 {
            n /= p;
            factors += 1;
        }
        p += 1;
    }
    factors + u32::from(n > 1) == 2
}

proptest! {
    #[test]
    fn goldbach_agrees_with_exhaustive_search(n in 0_u64..5000) {
        let expected: Vec<_> = (2..=n / 2)
            .filter(|&p| is_prime(p) != 0 && is_prime(n - p) != 0)
            .collect();
        let pairs = goldbach_pairs(n);
        prop_assert_eq!(&pairs, &expected);
        prop_assert_eq!(goldbach_pair_count(n), pairs.len() as u64);
        prop_assert_eq!(minimal_goldbach_pair(n), pairs.first().copied());
    }

    #[test]
    fn chen_is_the_next_valid_prime(n in 0_u64..10_000) {
        let p = next_chen_prime(n).unwrap();
        prop_assert!(p > n);
        prop_assert!(is_prime(p) != 0);
        prop_assert!(is_prime(p + 2) != 0 || semiprime(p + 2));
        for q in n + 1..p {
            prop_assert!(is_prime(q) == 0 || (is_prime(q + 2) == 0 && !semiprime(q + 2)));
        }
    }
}

#[test]
fn repeat_and_concurrent_calls_agree() {
    let expected = goldbach_pairs(100_000);
    for handle in (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (
                    goldbach_pairs(100_000),
                    goldbach_pair_count(100_000),
                    next_chen_prime(167),
                )
            })
        })
        .collect::<Vec<_>>()
    {
        let (pairs, count, chen) = handle.join().unwrap();
        assert_eq!(pairs, expected);
        assert_eq!(count, expected.len() as u64);
        assert_eq!(chen, Some(179));
    }
}
