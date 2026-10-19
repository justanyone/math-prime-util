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
    combinat::multifactorial(&big(0), &big(0));
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
