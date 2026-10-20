//! Native parity and exact product and permutation identities.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::combinat;
use num_bigint::BigUint;
use proptest::prelude::*;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

proptest! {
    #[test]
    fn products_match_native(n: u64, m in 0_u64..8, small in 0_u64..25, step in 1_u64..30) {
        let falling = combinat::falling_factorial(&big(n), &big(m));
        let rising = combinat::rising_factorial(&big(n), &big(m));
        prop_assert_eq!(u64::try_from(falling).ok(), math_prime_util::falling_factorial(n, m));
        prop_assert_eq!(u64::try_from(rising).ok(), math_prime_util::rising_factorial(n, m));
        let multi = combinat::multifactorial(&big(small), &big(step));
        prop_assert_eq!(u64::try_from(multi).ok(), math_prime_util::multifactorial(small, step));
        // Full u64 input range with a step that needs at most one factor.
        prop_assert_eq!(combinat::multifactorial(&big(n), &big(u64::MAX)),
            big(math_prime_util::multifactorial(n, u64::MAX).unwrap()));
    }

    #[test]
    fn permutations_match_native(n in 0_usize..22, rank: u64) {
        let p = combinat::numtoperm(n, &big(rank));
        prop_assert_eq!(&p, &math_prime_util::numtoperm(n, rank));
        let native: Vec<u64> = p.iter().map(|&v| u64::try_from(v).unwrap()).collect();
        let values: Vec<BigUint> = native.iter().copied().map(big).collect();
        let got = combinat::permtonum(&values).unwrap();
        prop_assert_eq!(u64::try_from(got).ok(), math_prime_util::permtonum(&native).unwrap());
    }

    #[test]
    fn large_permutation_roundtrip(rank in any::<u128>(), n in 22_usize..35) {
        let rank = BigUint::from(rank);
        let p = combinat::numtoperm(n, &rank);
        let p: Vec<BigUint> = p.into_iter().map(BigUint::from).collect();
        let period = (2..=n).fold(big(1), |a, b| a * b);
        prop_assert_eq!(combinat::permtonum(&p).unwrap(), rank % period);
    }

    #[test]
    fn products_above_native(n in any::<u128>(), m in 0_u64..8) {
        let n = BigUint::from(n) + (big(1) << 64_usize);
        let f = combinat::falling_factorial(&n, &big(m));
        prop_assert_eq!(combinat::falling_factorial(&n, &big(m+1)), f * (&n - m));
        let r = combinat::rising_factorial(&n, &big(m));
        prop_assert_eq!(combinat::rising_factorial(&n, &big(m+1)), r * (&n + m));
    }
}

#[test]
fn validation_and_repeat_calls() {
    use math_prime_util::PermutationError::Invalid;
    assert_eq!(combinat::permtonum(&[]), Ok(big(0)));
    assert_eq!(combinat::permtonum(&[big(0), big(0)]), Err(Invalid));
    assert_eq!(combinat::permtonum(&[big(2)]), Err(Invalid));
    assert_eq!(combinat::permtonum(&[big(1) << 200_usize]), Err(Invalid));
    let rank = big(1) << 100_usize;
    let saved = rank.clone();
    let a = combinat::numtoperm(30, &rank);
    assert_eq!(a, combinat::numtoperm(30, &rank));
    assert_eq!(rank, saved);
    assert_eq!(combinat::falling_factorial(&big(0), &big(0)), big(1));
    assert_eq!(combinat::falling_factorial(&big(0), &big(1)), big(0));
    assert_eq!(combinat::rising_factorial(&big(0), &big(2)), big(0));
    assert_eq!(combinat::multifactorial(&big(0), &big(1)), big(1));
}

#[test]
#[should_panic(expected = "k must be positive")]
fn multifactorial_rejects_zero_step() {
    let _ = combinat::multifactorial(&big(0), &big(0));
}

#[test]
fn concurrent_calls_have_no_shared_state() {
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                let rank = big(1) << 100_usize;
                let p: Vec<BigUint> = combinat::numtoperm(30, &rank)
                    .into_iter()
                    .map(BigUint::from)
                    .collect();
                assert_eq!(combinat::permtonum(&p), Ok(rank));
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

fn big_i(v: i128) -> num_bigint::BigInt {
    num_bigint::BigInt::from(v)
}

/// Native counting numbers return `None` once the value leaves `u64`.
fn same_as_native(got: Option<BigUint>, native: Option<u64>) {
    let got = got.expect("small n fits a usize");
    assert_eq!(u64::try_from(&got).ok(), native, "value {got}");
}

proptest! {
    #[test]
    fn counting_numbers_match_native(n in 0_u64..90) {
        same_as_native(combinat::bell_number(&big(n)), math_prime_util::bell_number(n));
        same_as_native(combinat::catalan_number(&big(n)), math_prime_util::catalan_number(n));
        same_as_native(combinat::subfactorial(&big(n)), math_prime_util::subfactorial(n));
        same_as_native(combinat::fubini(&big(n)), math_prime_util::fubini(n));
        same_as_native(combinat::partitions(&big(n * 6)), math_prime_util::partitions(n * 6));
        same_as_native(combinat::partitionsq(&big(n * 6)), math_prime_util::partitionsq(n * 6));
    }

    #[test]
    fn stirling_matches_native(n in 0_u64..45, m in 0_u64..48, kind in 1_u64..4) {
        let got = combinat::stirling(&big(n), &big(m), kind).unwrap();
        match math_prime_util::stirling(n, m, kind) {
            Some(v) => prop_assert_eq!(got, BigUint::from(v.unsigned_abs())),
            None => prop_assert!(got > big(u64::MAX)),
        }
    }

    #[test]
    fn fractions_match_native(n in 0_u64..100) {
        let (num, den) = combinat::bernfrac(&big(n)).unwrap();
        if let Some((a, b)) = math_prime_util::bernfrac(n) {
            prop_assert_eq!((num, den), (big_i(a), BigUint::from(b)));
        }
        let (num, den) = combinat::harmfrac(&big(n)).unwrap();
        if let Some((a, b)) = math_prime_util::harmfrac(n) {
            prop_assert_eq!((num, den), (BigUint::from(a.unsigned_abs()), BigUint::from(b)));
        }
    }

    #[test]
    fn tau_matches_native(n in 0_u64..3000) {
        let got = combinat::ramanujan_tau(&big(n)).unwrap();
        if let Some(v) = math_prime_util::ramanujan_tau(n) {
            prop_assert_eq!(got, big_i(i128::from(v)));
        }
    }

    #[test]
    fn tau_is_multiplicative(a in 1_u64..400, b in 1_u64..400) {
        prop_assume!(math_prime_util::gcd(a, b) == 1);
        let t = |n: u64| combinat::ramanujan_tau(&big(n)).unwrap();
        prop_assert_eq!(t(a * b), t(a) * t(b));
    }
}

#[test]
fn identities_above_native() {
    // Recurrences and closed forms, with values far above u64.
    let b = |n: u64| combinat::bell_number(&big(n)).unwrap();
    let c = |n: u64| combinat::catalan_number(&big(n)).unwrap();
    let d = |n: u64| combinat::subfactorial(&big(n)).unwrap();
    for n in [30_u64, 64, 100] {
        // Bell(n+1) = sum C(n,k) Bell(k): check via Stirling 2 row sum.
        let row: BigUint = (0..=n + 1)
            .map(|k| combinat::stirling(&big(n + 1), &big(k), 2).unwrap())
            .sum();
        assert_eq!(row, b(n + 1));
        // (n+2) C(n+1) = 2 (2n+1) C(n).
        assert_eq!(c(n + 1) * (n + 2), c(n) * (4 * n + 2));
        // D(n) = n D(n-1) + (-1)^n.
        let lhs = d(n) + u8::from(n % 2 == 1);
        assert_eq!(lhs, d(n - 1) * n + u8::from(n % 2 == 0));
    }
    // Unsigned Stirling 1 row sums to n!.
    let fact: BigUint = (1..=30_u64).map(BigUint::from).product();
    let sum: BigUint = (0..=30)
        .map(|k| combinat::stirling(&big(30), &big(k), 1).unwrap())
        .sum();
    assert_eq!(sum, fact);
    // Lah numbers: L(n,1) = n!.
    assert_eq!(combinat::stirling(&big(30), &big(1), 3), Some(fact));
    // Big arguments give None, or zero for a huge m.
    let huge = big(1) << 80_usize;
    assert_eq!(combinat::bell_number(&huge), None);
    assert_eq!(combinat::partitionsq(&huge), None);
    assert_eq!(combinat::stirling(&huge, &big(1), 2), None);
    assert_eq!(combinat::stirling(&big(5), &huge, 2), Some(big(0)));
    assert_eq!(combinat::bernfrac(&huge), None);
    assert_eq!(combinat::harmfrac(&huge), None);
    assert_eq!(combinat::fubini(&huge), None);
    assert_eq!(combinat::catalan_number(&huge), None);
    assert_eq!(combinat::subfactorial(&huge), None);
    assert_eq!(combinat::partitions(&huge), None);
}

#[test]
fn fractions_above_native() {
    // B_n has the von Staudt-Clausen denominator: the sum of 1/p over p-1 | n is an integer shift.
    let (num, den) = combinat::bernfrac(&big(100)).unwrap();
    assert_eq!(den, BigUint::from(33_330_u32));
    assert_eq!(num.sign(), num_bigint::Sign::Minus);
    // Odd index is zero except B_1.
    assert_eq!(combinat::bernfrac(&big(101)).unwrap(), (big_i(0), big(1)));
    assert_eq!(combinat::bernfrac(&big(1)).unwrap(), (big_i(1), big(2)));
    // H_n - H_(n-1) = 1/n, so the difference is a unit fraction.
    let (p1, q1) = combinat::harmfrac(&big(120)).unwrap();
    let (p0, q0) = combinat::harmfrac(&big(119)).unwrap();
    assert_eq!(p1 * &q0 * 120_u32 - p0 * &q1 * 120_u32, q1 * q0);
}

#[test]
fn tau_unsupported_inputs() {
    // A prime factor above 2^32 is impractical.
    let p = (big(1) << 61_usize) - 1_u8;
    assert_eq!(combinat::ramanujan_tau(&p), None);
    // A large smooth input, and a prime power with a repeated factor.
    let n = (big(1) << 100_usize) * 9_u8;
    let t = combinat::ramanujan_tau(&n).unwrap();
    let t2 = combinat::ramanujan_tau(&(big(1) << 100_usize)).unwrap();
    assert_eq!(t, t2 * combinat::ramanujan_tau(&big(9)).unwrap());
    // A prime in the formula range (above the 47-entry table).
    assert_eq!(
        combinat::ramanujan_tau(&big(10007)).unwrap().to_string(),
        "-5758585642481476962744"
    );
}

#[test]
#[should_panic(expected = "type must be")]
fn stirling_rejects_bad_kind() {
    let _ = combinat::stirling(&big(3), &big(2), 4);
}
