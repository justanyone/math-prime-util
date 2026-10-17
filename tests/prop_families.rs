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

const SMALL_RAM: [u64; 23] = [
    2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227, 229, 233,
    239, 241,
];

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn ramanujan_upper_is_at_least_the_prime_on_the_small_range(k in 1u64..24) {
        let prime = SMALL_RAM[k as usize - 1];
        prop_assert_eq!(nth_ramanujan_prime_upper(k), Some(prime));
        prop_assert!(is_ramanujan_prime(prime));
        prop_assert_eq!(is_prime(prime), 2);
    }

    #[test]
    fn ramanujan_upper_grows_past_the_table(k in 24u64..80) {
        let lo = nth_ramanujan_prime_upper(k).unwrap();
        let hi = nth_ramanujan_prime_upper(k + 1).unwrap();
        prop_assert!(lo <= hi);
        prop_assert!(lo > k);
    }
}

#[test]
fn ramanujan_validation_repeat_and_threads() {
    assert_eq!(nth_ramanujan_prime_upper(0), None);
    assert!(!is_ramanujan_prime(0));
    assert!(!is_ramanujan_prime(1));
    assert!(!is_ramanujan_prime(u64::MAX));
    let bound = nth_ramanujan_prime_upper(100);
    assert_eq!(bound, nth_ramanujan_prime_upper(100));
    assert_eq!(is_ramanujan_prime(97), is_ramanujan_prime(97));
    let threads: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| (nth_ramanujan_prime_upper(80), is_ramanujan_prime(241))))
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap(), (Some(1123), true));
    }
    for (index, &prime) in SMALL_RAM.iter().enumerate() {
        assert!(is_ramanujan_prime(prime));
        if index + 1 < SMALL_RAM.len() {
            for n in prime + 1..SMALL_RAM[index + 1] {
                assert!(!is_ramanujan_prime(n), "{n}");
            }
        }
    }
}
