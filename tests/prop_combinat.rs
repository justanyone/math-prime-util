//! Algebraic laws and native overflow boundaries.
use math_prime_util::{
    bernfrac, harmfrac, partitions, partitionsq, permtonum, pn_primorial, primorial,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn odd_bernoulli_vanishes(k in 1..u64::MAX / 2) {
        prop_assert_eq!(bernfrac(2 * k + 1), Some((0, 1)));
    }

    #[test]
    fn harmonic_recurrence(n in 1_u64..=30) {
        let (a, b) = harmfrac(n - 1).unwrap();
        let (c, d) = harmfrac(n).unwrap();
        let a = u128::try_from(a).unwrap();
        let c = u128::try_from(c).unwrap();
        prop_assert_eq!(c * b * u128::from(n), d * (a * u128::from(n) + b));
    }

    #[test]
    fn primorial_step(n in 2_u64..=52) {
        let prime = (2..n).all(|d| n % d != 0);
        let previous = primorial(n - 1).unwrap();
        prop_assert_eq!(primorial(n), Some(previous * if prime { n } else { 1 }));
    }
}

#[test]
fn prime_index_products() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut product = 1;
    assert_eq!(pn_primorial(0), Some(product));
    for (i, prime) in primes.into_iter().enumerate() {
        product *= prime;
        assert_eq!(pn_primorial(u64::try_from(i + 1).unwrap()), Some(product));
        assert_eq!(primorial(prime), Some(product));
    }
}

#[test]
fn native_boundaries_and_repeat_calls() {
    for _ in 0..2 {
        assert_eq!(harmfrac(0), Some((0, 1)));
        assert!(harmfrac(88).is_some());
        assert_eq!(harmfrac(u64::MAX), None);
        assert!(bernfrac(58).is_some());
        assert_eq!(bernfrac(60), None);
        assert_eq!(bernfrac(u64::MAX - 1), None);
        assert_eq!(bernfrac(u64::MAX), Some((0, 1)));
        assert_eq!(primorial(52), pn_primorial(15));
        assert_eq!(primorial(53), None);
        assert_eq!(primorial(u64::MAX), None);
        assert_eq!(pn_primorial(16), None);
        assert_eq!(pn_primorial(u64::MAX), None);
    }
}

#[test]
fn partition_counts_match_enumeration() {
    for n in 0_u64..=24 {
        assert_eq!(partitions(n), Some(count_partitions(n)));
        assert_eq!(partitionsq(n), Some(count_distinct_partitions(n)));
        if n > 0 {
            assert!(partitions(n).unwrap() >= partitions(n - 1).unwrap());
            assert!(partitionsq(n).unwrap() <= partitions(n).unwrap());
        }
    }
}

#[test]
fn permutation_ranks_are_complete() {
    for length in 0_usize..=7 {
        let mut perm: Vec<u64> = (0..u64::try_from(length).unwrap()).collect();
        let mut ranks = Vec::new();
        collect_ranks(&mut perm, 0, &mut ranks);
        ranks.sort_unstable();
        let factorial = (1..=length)
            .map(|factor| u64::try_from(factor).unwrap())
            .product();
        let expected: Vec<u64> = (0..factorial).collect();
        assert_eq!(ranks, expected);
    }
    for length in 1_u64..=20 {
        let reverse: Vec<u64> = (0..length).rev().collect();
        let mut factorial = 1_u64;
        for factor in 1..=length {
            factorial *= factor;
        }
        assert_eq!(permtonum(&reverse), Some(factorial - 1));
        let identity: Vec<u64> = (0..length).collect();
        assert_eq!(permtonum(&identity), Some(0));
    }
}

#[test]
fn native_overflow_and_invalid_permutation() {
    assert!(partitions(416).is_some());
    assert_eq!(partitions(417), None);
    assert_eq!(partitions(u64::MAX), None);
    assert!(partitionsq(791).is_some());
    assert_eq!(partitionsq(792), None);
    assert_eq!(partitionsq(u64::MAX), None);
    // `21!` does not fit in `u64`. The reverse of `0..21` overflows the rank.
    let reverse_21: Vec<u64> = (0..21).rev().collect();
    assert_eq!(permtonum(&reverse_21), None);
    // `factorial(21)` is 0 in util.c, so length 22 returns `None` even for the identity.
    let identity_22: Vec<u64> = (0..22).collect();
    assert_eq!(permtonum(&identity_22), None);
    let identity_21: Vec<u64> = (0..21).collect();
    assert_eq!(permtonum(&identity_21), Some(0));
    assert!(std::panic::catch_unwind(|| permtonum(&[1, 1])).is_err());
    assert!(std::panic::catch_unwind(|| permtonum(&[0, 2])).is_err());
    assert!(std::panic::catch_unwind(|| permtonum(&[3, 0, 1])).is_err());
}

#[test]
fn repeat_and_concurrent_partition_calls() {
    let sample = [6_u64, 3, 4, 2, 5, 0, 1];
    let once = (permtonum(&sample), partitions(120), partitionsq(90));
    let twice = (permtonum(&sample), partitions(120), partitionsq(90));
    assert_eq!(once, twice);
    let workers: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(move || (permtonum(&sample), partitions(120), partitionsq(90))))
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), once);
    }
}

fn count_partitions(n: u64) -> u64 {
    fn rec(remain: u64, max_part: u64) -> u64 {
        if remain == 0 {
            return 1;
        }
        let mut count = 0;
        for part in 1..=max_part.min(remain) {
            count += rec(remain - part, part);
        }
        count
    }
    rec(n, n)
}

fn count_distinct_partitions(n: u64) -> u64 {
    fn rec(remain: u64, min_part: u64) -> u64 {
        if remain == 0 {
            return 1;
        }
        let mut count = 0;
        for part in min_part..=remain {
            count += rec(remain - part, part + 1);
        }
        count
    }
    rec(n, 1)
}

fn collect_ranks(perm: &mut [u64], start: usize, ranks: &mut Vec<u64>) {
    if start == perm.len() {
        ranks.push(permtonum(perm).unwrap());
        return;
    }
    for index in start..perm.len() {
        perm.swap(start, index);
        collect_ranks(perm, start + 1, ranks);
        perm.swap(start, index);
    }
}

#[test]
fn concurrent_calls() {
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| (bernfrac(58), harmfrac(60), primorial(52), pn_primorial(15)))
        })
        .collect();
    let expected = (bernfrac(58), harmfrac(60), primorial(52), pn_primorial(15));
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}
