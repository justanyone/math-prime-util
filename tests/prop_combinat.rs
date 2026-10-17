//! Algebraic laws and native overflow boundaries.
use math_prime_util::{bernfrac, harmfrac, pn_primorial, primorial};
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
