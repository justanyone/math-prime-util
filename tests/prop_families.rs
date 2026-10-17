//! Algebraic laws for the Goldbach and Chen functions.
use math_prime_util::{
    goldbach_pair_count, goldbach_pairs, is_prime, minimal_goldbach_pair, next_chen_prime,
};
use proptest::prelude::*;

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
