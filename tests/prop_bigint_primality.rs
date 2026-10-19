//! The arbitrary precision primality tests agree with the native path on the
//! `u64` range, and `Int::is_prime` on `BigUint` is `is_prob_prime`.
#![cfg(feature = "bigint")]
use math_prime_util::Int;
use math_prime_util::bigint::primality;
use num_bigint::{BigInt, BigUint};
use proptest::prelude::*;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

proptest! {
    #[test]
    fn base_tests_agree(n: u64, a in 2_u64..=u64::MAX, odd in 1_u64..=u64::MAX) {
        for n in [n, odd | 1] {
            let (bn, ba) = (big(n), [big(a)]);
            prop_assert_eq!(primality::is_strong_pseudoprime(&bn, &ba), math_prime_util::is_strong_pseudoprime(n, a));
            prop_assert_eq!(primality::is_pseudoprime(&bn, &ba), math_prime_util::is_pseudoprime(n, a));
            prop_assert_eq!(primality::is_euler_pseudoprime(&bn, &ba), math_prime_util::is_euler_pseudoprime(n, a));
            prop_assert_eq!(primality::is_euler_plumb_pseudoprime(&bn), math_prime_util::is_euler_plumb_pseudoprime(n));
            prop_assert_eq!(primality::is_strong_pseudoprime(&bn, &[]), math_prime_util::is_strong_pseudoprime(n, 2));
        }
    }

    #[test]
    fn lucas_tests_agree(n: u64, odd in 1_u64..u64::MAX, increment in 1_u64..=256) {
        for n in [n, odd | 1] {
            let bn = big(n);
            prop_assert_eq!(primality::is_lucas_pseudoprime(&bn), math_prime_util::is_lucas_pseudoprime(n));
            prop_assert_eq!(primality::is_strong_lucas_pseudoprime(&bn), math_prime_util::is_strong_lucas_pseudoprime(n));
            prop_assert_eq!(primality::is_extra_strong_lucas_pseudoprime(&bn), math_prime_util::is_extra_strong_lucas_pseudoprime(n));
            prop_assert_eq!(
                primality::is_almost_extra_strong_lucas_pseudoprime(&bn, increment),
                math_prime_util::is_almost_extra_strong_lucas_pseudoprime(n, increment)
            );
            prop_assert_eq!(primality::is_bpsw_prime(&bn), math_prime_util::is_bpsw_prime(n));
        }
    }

    #[test]
    fn primes_agree(n: u64, wide: u128, small in 0_u64..1_000_000) {
        for n in [n, small] {
            let bn = big(n);
            prop_assert_eq!(primality::is_prime(&bn), math_prime_util::is_prime(n));
            prop_assert_eq!(primality::is_prob_prime(&bn), math_prime_util::is_prob_prime(n));
            prop_assert_eq!(bn.is_prime(), primality::is_prob_prime(&bn));
            prop_assert_eq!(primality::next_prime(&bn), bn.next_prime().unwrap());
            let expected = math_prime_util::next_prime(n).map_or((BigUint::from(1_u8) << 64_usize) + 13_u8, big);
            prop_assert_eq!(primality::next_prime(&bn), expected);
            prop_assert_eq!(primality::prev_prime(&bn), math_prime_util::prev_prime(n).map(big));
            if let Some(known) = math_prime_util::is_mersenne_prime(n) {
                prop_assert_eq!(primality::is_mersenne_prime(n), known);
            }
        }
        let bw = BigUint::from(wide);
        prop_assert_eq!(primality::is_prob_prime(&bw), math_prime_util::is_prime_u128(wide));
        prop_assert_eq!(bw.is_prime(), primality::is_prob_prime(&bw));
        prop_assert!(primality::is_prob_prime(&bw) < 2 || wide <= u128::from(u64::MAX));
    }

    #[test]
    fn lucas_sequence_agrees(n in 1_u64..=u64::MAX, p: i64, q: i64, k: u64, small_k in 0_u64..64) {
        for k in [k, small_k] {
            let (u, v, qk) = math_prime_util::lucas_sequence(n, p, q, k);
            prop_assert_eq!(
                primality::lucas_sequence(&big(n), &BigInt::from(p), &BigInt::from(q), &big(k)),
                (big(u), big(v), big(qk))
            );
        }
    }

    #[test]
    fn probable_primes_above_the_native_range(a in 2_u64..=u64::MAX, b in 2_u64..=u64::MAX) {
        // A product of two primes above 2^32 is composite; a prime stays prime.
        let p = BigUint::from(math_prime_util::next_prime(a | (1 << 63)).unwrap_or(u64::MAX - 58));
        let q = BigUint::from(math_prime_util::next_prime(b | (1 << 63)).unwrap_or(u64::MAX - 58));
        let product = &p * &q;
        prop_assert_eq!(primality::is_prob_prime(&product), 0);
        prop_assert_eq!(product.is_prime(), 0);
        prop_assert!(!primality::is_strong_lucas_pseudoprime(&product));
        let r = primality::next_prime(&product);
        prop_assert_eq!(primality::is_prob_prime(&r), 1);
        prop_assert_eq!(r.is_prime(), 1);
        prop_assert_eq!(primality::prev_prime(&r), primality::prev_prime(&(&product + 1_u8)));
        prop_assert!(primality::is_strong_pseudoprime(&r, &[big(a), big(b)]));
        prop_assert!(primality::is_euler_pseudoprime(&r, &[big(a), big(b)]));
        prop_assert!(primality::is_lucas_pseudoprime(&r));
    }
}
