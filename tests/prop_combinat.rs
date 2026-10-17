//! Algebraic laws and native overflow boundaries.
use math_prime_util::{
    bell_number, bernfrac, binomial, catalan_number, factorial, falling_factorial, fubini,
    harmfrac, hclassno, integer_complexity, is_happy, is_pillai, multifactorial, numtoperm,
    pn_primorial, powersum, primorial, rising_factorial, stirling, subfactorial,
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

    #[test]
    fn binomial_symmetry_and_pascal(n in 1_u64..60, seed in any::<u64>()) {
        let k = seed % n;
        prop_assert_eq!(binomial(n, k), binomial(n, n - k));
        prop_assert_eq!(binomial(n + 1, k + 1),
            Some(binomial(n, k).unwrap() + binomial(n, k + 1).unwrap()));
    }

    #[test]
    fn catalan_recurrence(n in 0_u64..35) {
        let next = u128::from(catalan_number(n).unwrap()) * 2 * u128::from(2 * n + 1)
            / u128::from(n + 2);
        prop_assert_eq!(u128::from(catalan_number(n + 1).unwrap()), next);
    }

    #[test]
    fn stirling_recurrences(n in 1_u64..14, seed in any::<u64>()) {
        let m = seed % n + 1;
        let a = stirling(n, m - 1, 1).unwrap();
        let b = stirling(n, m, 1).unwrap();
        prop_assert_eq!(stirling(n + 1, m, 1), Some(a - i128::from(n) * b));
        let a = stirling(n, m - 1, 2).unwrap();
        let b = stirling(n, m, 2).unwrap();
        prop_assert_eq!(stirling(n + 1, m, 2), Some(a + i128::from(m) * b));
        let a = stirling(n, m - 1, 3).unwrap();
        let b = stirling(n, m, 3).unwrap();
        prop_assert_eq!(stirling(n + 1, m, 3), Some(a + i128::from(n + m) * b));
    }

    #[test]
    fn bell_binomial_transform(n in 0_u64..24) {
        let sum: u64 = (0..=n).map(|k| binomial(n, k).unwrap() * bell_number(k).unwrap()).sum();
        prop_assert_eq!(bell_number(n + 1), Some(sum));
    }

    #[test]
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
    }

    #[test]
    fn factorial_products(n in 0_u64..1000, m in 0_u64..30) {
        let falling = if m > n { Some(0) } else {
            (0..m).try_fold(1_u64, |r, i| r.checked_mul(n - i))
        };
        let rising = (0..m).try_fold(1_u64, |r, i| r.checked_mul(n + i));
        prop_assert_eq!(falling_factorial(n, m), falling);
        prop_assert_eq!(rising_factorial(n, m), rising);
    }

    #[test]
    fn power_sum_direct(n in 0_u64..200, k in 0_u32..20) {
        let expected = (1..=n).try_fold(0_u64, |sum, i| sum.checked_add(i.checked_pow(k)?));
        prop_assert_eq!(powersum(n, u64::from(k)), expected);
    }

    #[test]
    fn complexity_composition(a in 1_u64..100, b in 1_u64..100) {
        let bound = integer_complexity(a).unwrap() + integer_complexity(b).unwrap();
        prop_assert!(integer_complexity(a + b).unwrap() <= bound);
        prop_assert!(integer_complexity(a * b).unwrap() <= bound);
    }

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

#[test]
fn native_boundaries() {
    assert_eq!(binomial(u64::MAX, 0), Some(1));
    assert_eq!(binomial(u64::MAX, 1), Some(u64::MAX));
    assert_eq!(binomial(u64::MAX, u64::MAX - 1), Some(u64::MAX));
    assert_eq!(binomial(u64::MAX, 2), None);
    for (n, k) in [(73, 25), (92, 19), (131, 15)] {
        assert_eq!(binomial(n, k), None);
    }
    assert_eq!(catalan_number(36), Some(11_959_798_385_860_453_492));
    assert_eq!(catalan_number(37), None);
    assert_eq!(catalan_number(u64::MAX), None);
    assert_eq!(bell_number(25), Some(4_638_590_332_229_999_353));
    assert_eq!(bell_number(26), None);
    assert_eq!(bell_number(u64::MAX), None);
    for kind in 1..=3 {
        assert_eq!(stirling(u64::MAX, u64::MAX, kind), Some(1));
        assert_eq!(stirling(u64::MAX, u64::MAX - 1, kind), None);
        assert_eq!(stirling(0, 1, kind), Some(0));
        assert_eq!(
            stirling(40, 1, kind),
            if kind == 2 { Some(1) } else { None }
        );
        assert_eq!(stirling(100, 50, kind), None);
    }
}

#[test]
fn invalid_stirling_kind() {
    for kind in [0, 4, u64::MAX] {
        assert!(std::panic::catch_unwind(|| stirling(0, 0, kind)).is_err());
    }
}

#[test]
fn stirling_cancellation_regressions() {
    assert_eq!(stirling(18, 2, 1), Some(1_223_405_590_579_200));
    assert_eq!(stirling(64, 62, 2), Some(1_947_792));
    assert_eq!(stirling(65, 2, 2), Some(i128::from(u64::MAX)));
    assert_eq!(stirling(66, 2, 2), None);
    assert_eq!(stirling(100, 98, 1), Some(12_087_075));
    assert_eq!(stirling(100, 98, 2), Some(11_925_375));
    assert_eq!(stirling(1000, 2, 1), None);
    assert_eq!(stirling(1000, 2, 2), None);
}

#[test]
fn independent_calls_are_repeatable() {
    let calls: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (
                    binomial(67, 31),
                    catalan_number(36),
                    stirling(22, 12, 1),
                    bell_number(25),
                )
            })
        })
        .collect();
    let expected = (
        binomial(67, 31),
        catalan_number(36),
        stirling(22, 12, 1),
        bell_number(25),
    );
    for call in calls {
        assert_eq!(call.join().unwrap(), expected);
    }
}

#[test]
fn native_boundaries_factorial() {
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
}

#[test]
fn native_boundaries_rising_falling() {
    assert_eq!(falling_factorial(u64::MAX, 0), Some(1));
    assert_eq!(falling_factorial(u64::MAX, 1), Some(u64::MAX));
    assert_eq!(falling_factorial(u64::MAX, 2), None);
    assert_eq!(rising_factorial(u64::MAX, 2), None);
    assert_eq!(rising_factorial(0, u64::MAX), Some(0));
    assert_eq!(powersum(u64::MAX, 0), Some(u64::MAX));
    assert_eq!(powersum(0, u64::MAX), Some(0));
    assert_eq!(powersum(1, u64::MAX), Some(1));
    assert_eq!(powersum(2, 64), None);
    for (k, n) in [
        (1, 6_074_000_999),
        (2, 3_810_777),
        (3, 92_681),
        (4, 9839),
        (5, 2190),
        (6, 745),
        (7, 331),
        (8, 175),
    ] {
        assert!(powersum(n, k).is_some());
        assert_eq!(powersum(n + 1, k), None);
    }
}

#[test]
fn complexity_matches_exhaustive_recurrence() {
    let mut exact = vec![0_u64; 301];
    exact[1] = 1;
    for n in 2..exact.len() {
        exact[n] = (1..=n / 2).map(|a| exact[a] + exact[n - a]).min().unwrap();
        for a in 2..=n / 2 {
            if n % a == 0 {
                exact[n] = exact[n].min(exact[a] + exact[n / a]);
            }
        }
        assert_eq!(
            integer_complexity(u64::try_from(n).unwrap()),
            Some(exact[n])
        );
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
