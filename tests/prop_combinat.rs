<<<<<<< HEAD
//! Algebraic laws and native boundary checks for the factorial family.
use math_prime_util::{factorial, fubini, multifactorial, subfactorial};
=======
//! Algebraic laws and native overflow boundaries.
use math_prime_util::{bernfrac, harmfrac, pn_primorial, primorial};
>>>>>>> origin/dev-001
use proptest::prelude::*;

proptest! {
    #[test]
<<<<<<< HEAD
    fn factorial_recurrence(n in 1_u64..=20) {
        prop_assert_eq!(factorial(n), factorial(n - 1).and_then(|v| v.checked_mul(n)));
        prop_assert_eq!(multifactorial(n, 1), factorial(n));
    }

    #[test]
    fn multifactorial_recurrence(n in any::<u64>(), k in 1_u64..) {
        let expected = if n == 0 {
            Some(1)
        } else if n <= k {
            Some(n)
        } else {
            multifactorial(n - k, k).and_then(|value| value.checked_mul(n))
        };
        prop_assert_eq!(multifactorial(n, k), expected);
    }

    #[test]
    fn derangement_recurrence(n in 2_u64..=20) {
        let previous = subfactorial(n - 1).unwrap() + subfactorial(n - 2).unwrap();
        prop_assert_eq!(subfactorial(n), Some((n - 1) * previous));
    }

    #[test]
    fn ordered_partition_recurrence(n in 1_u64..=18) {
        let mut binomial = 1_u128;
        let mut sum = 0_u128;
        for k in 1..=n {
            binomial = binomial * u128::from(n - k + 1) / u128::from(k);
            sum += binomial * u128::from(fubini(n - k).unwrap());
        }
        prop_assert_eq!(fubini(n), u64::try_from(sum).ok());
=======
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
>>>>>>> origin/dev-001
    }
}

#[test]
<<<<<<< HEAD
fn native_boundaries() {
    assert_eq!(factorial(0), Some(1));
    assert_eq!(factorial(20), Some(2_432_902_008_176_640_000));
    assert_eq!(factorial(21), None);
    assert_eq!(factorial(u64::MAX), None);
    assert_eq!(subfactorial(0), Some(1));
    assert_eq!(subfactorial(1), Some(0));
    assert_eq!(subfactorial(2), Some(1));
    assert_eq!(subfactorial(3), Some(2));
    assert_eq!(subfactorial(20), Some(895_014_631_192_902_121));
    assert_eq!(subfactorial(21), None);
    assert_eq!(subfactorial(u64::MAX), None);
    assert_eq!(fubini(0), Some(1));
    assert_eq!(fubini(15), Some(230_283_190_977_853));
    assert_eq!(fubini(16), Some(5_315_654_681_981_355));
    assert_eq!(fubini(17), Some(130_370_767_029_135_901));
    assert_eq!(fubini(18), Some(3_385_534_663_256_845_323));
    assert_eq!(fubini(19), None);
    assert_eq!(fubini(u64::MAX), None);
    assert_eq!(multifactorial(0, 1), Some(1));
    assert_eq!(multifactorial(0, u64::MAX), Some(1));
    assert_eq!(multifactorial(u64::MAX, u64::MAX), Some(u64::MAX));
    assert_eq!(multifactorial(u64::MAX, u64::MAX - 1), Some(u64::MAX));
    assert_eq!(multifactorial(u64::MAX, 1), None);
    assert_eq!(multifactorial(33, 2), Some(6_332_659_870_762_850_625));
    assert_eq!(multifactorial(35, 2), None);
}

#[test]
#[should_panic(expected = "k must be positive")]
fn zero_step_rejected_for_empty_product() {
    let _ = multifactorial(0, 0);
}

#[test]
fn concurrent_repeated_calls() {
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..3 {
                    native_boundaries();
                }
            });
        }
    });
=======
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
>>>>>>> origin/dev-001
}
