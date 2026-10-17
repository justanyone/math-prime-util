//! Algebraic and boundary checks for the assigned combinat functions.
//! Algebraic laws and native overflow boundaries.
use math_prime_util::{
    bernfrac, harmfrac, hclassno, is_happy, is_pillai, numtoperm, pn_primorial, primorial,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn pillai_witness_is_least(n in 0_u64..2000) {
        let actual = is_pillai(n);
        let mut fac = 1;
        let mut expected = 0;
        for v in 1..n.saturating_sub(1) {
            fac = fac * v % n;
            if fac == n - 1 && n % v != 1 {
                expected = v;
                break;
            }
        }
        prop_assert_eq!(actual, expected);
    }

    #[test]
    fn happy_height_follows_digit_sum(n in 2_u64..1_000_000, base in 2_u32..=36, exponent in 0_u32..=4) {
        let mut digits = n;
        let mut sum = 0;
        while digits > 0 {
            sum += (digits % u64::from(base)).pow(exponent);
            digits /= u64::from(base);
        }
        let next = is_happy(sum, base, exponent);
        prop_assert_eq!(is_happy(n, base, exponent), if next == 0 { 0 } else { next + 1 });
    }

    #[test]
    fn class_number_has_discriminant_support(n in 1_u64..500) {
        let h = hclassno(n).unwrap();
        prop_assert!(h >= 0);
        if matches!(n % 4, 1 | 2) { prop_assert_eq!(h, 0); }
        else { prop_assert!(h > 0); }
    }

    #[test]
    fn permutation_rank_round_trip(n in 0_usize..=20, rank in any::<u64>()) {
        let permutation = numtoperm(n, rank);
        let mut sorted = permutation.clone();
        sorted.sort_unstable();
        prop_assert_eq!(sorted, (0..n).collect::<Vec<_>>());
        let mut recovered = 0_u64;
        for (i, &value) in permutation.iter().enumerate() {
            let smaller = permutation[i+1..].iter().filter(|&&x| x < value).count();
            recovered = recovered * u64::try_from(n-i).unwrap() + u64::try_from(smaller).unwrap();
        }
        let period = (1..=n).map(|i| u64::try_from(i).unwrap()).product::<u64>();
        prop_assert_eq!(recovered, rank % period);
    }

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
fn native_boundaries_and_repeated_calls() {
    assert_eq!(hclassno(0), Some(-1));
    assert_eq!(hclassno(1_u64 << 63), None);
    assert_eq!(hclassno(u64::MAX), None);
    assert_eq!(numtoperm(0, u64::MAX), []);
    assert_eq!(numtoperm(1, u64::MAX), [0]);
    for n in [21, 24, 40] {
        let p = numtoperm(n, u64::MAX);
        let mut sorted = p.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..n).collect::<Vec<_>>());
        assert_eq!(p, numtoperm(n, u64::MAX));
    }
    assert_eq!(is_happy(0, 36, 10), 0);
    assert_eq!(is_happy(1, 36, 10), 1);
    assert_eq!(is_happy(u64::MAX, 2, 1), 3);
    assert_eq!(is_happy(u64::MAX, 36, 10), is_happy(u64::MAX, 36, 10));
    assert_eq!(is_pillai(29), is_pillai(29));
    assert_eq!(is_pillai((1_u64 << 32) + 1), 0);
}

#[test]
fn happy_rejects_invalid_arguments() {
    for base in [0, 1, 37, u32::MAX] {
        assert!(std::panic::catch_unwind(|| is_happy(1, base, 2)).is_err());
    }
    assert!(std::panic::catch_unwind(|| is_happy(1, 10, 11)).is_err());
}

#[test]
fn independent_calls_are_thread_safe() {
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (
                    is_pillai(23),
                    is_happy(19, 10, 2),
                    hclassno(1000),
                    numtoperm(24, 987_654_321),
                )
            })
        })
        .collect();
    let expected = (14, 5, hclassno(1000), numtoperm(24, 987_654_321));
    for thread in threads {
        assert_eq!(thread.join().unwrap(), expected);
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
