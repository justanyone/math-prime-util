//! The arbitrary precision modular functions agree with the native path on
//! the `u64` range, and satisfy their defining identities on large inputs:
//! a square root squares back, an inverse times its argument is 1, `znlog`
//! inverts `powmod`.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::arith::{binomial, factorial, invmod, powmod};
use math_prime_util::bigint::modular::*;
use math_prime_util::bigint::primality::next_prime;
use num_bigint::{BigInt, BigUint};
use proptest::prelude::*;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

fn big_pair(a: u64, n: u64) -> (BigUint, BigUint) {
    (big(a), big(n))
}

proptest! {
    #[test]
    fn roots_agree_with_native(a: u64, n: u64, small in 0_u64..100_000, k in 0_u64..20) {
        for n in [n, small] {
            let (x, m) = (big(a), big(n));
            let root = sqrtmod(&x, &m);
            prop_assert_eq!(&root, &math_prime_util::sqrtmod(a, n).map(big));
            if let Some(r) = root {
                prop_assert_eq!(&r * &r % &m, &x % &m);
            }
            let root = rootmod(&x, &big(k), &m);
            prop_assert_eq!(&root, &math_prime_util::rootmod(a, k, n).map(big));
            if let Some(r) = root {
                prop_assert_eq!(powmod(&r, &big(k), &m), Some(&x % &m));
            }
        }
        let roots = allsqrtmod(&big(a), &big(small));
        prop_assert_eq!(roots, math_prime_util::allsqrtmod(a, small).into_iter().map(big).collect::<Vec<_>>());
    }

    #[test]
    fn groups_agree_with_native(a: u64, n: u64, small in 0_u64..1_000_000, g: u64) {
        for n in [n, small] {
            let (x, m) = (big(a), big(n));
            prop_assert_eq!(znorder(&x, &m), math_prime_util::znorder(a, n).map(big));
            prop_assert_eq!(znprimroot(&m), math_prime_util::znprimroot(n).map(big));
            prop_assert_eq!(is_primitive_root(&x, &m), math_prime_util::is_primitive_root(a, n));
        }
        let (x, y, m) = (big(a), big(g), big(small));
        let log = znlog(&x, &y, &m);
        prop_assert_eq!(&log, &math_prime_util::znlog(a, g, small).map(big));
        if let Some(k) = log {
            prop_assert_eq!(powmod(&y, &k, &m), Some(&x % &m));
        }
    }

    #[test]
    fn products_agree_with_native(n in 0_u64..3000, k in 0_u64..3000, m: u64, small in 0_u64..5000) {
        for m in [m, small] {
            prop_assert_eq!(factorialmod(&big(n), &big(m)), math_prime_util::factorialmod(n, m).map(big));
            prop_assert_eq!(binomialmod(&big(n), &big(k), &big(m)), math_prime_util::binomialmod(n, k, m).map(big));
        }
    }

    #[test]
    fn chinese_agrees_with_native(a: u64, n in 1_u64..=u64::from(u32::MAX), b: u64, m in 1_u64..=u64::from(u32::MAX), c: u64, l in 0_u64..1000) {
        let pairs = [(a, n), (b, m), (c, l)];
        let bigs: Vec<(BigUint, BigUint)> = pairs.iter().map(|&(x, y)| big_pair(x, y)).collect();
        // The native path answers in full only while the combined modulus stays below 2^63 (util.c's signed path).
        // Above it, util.c::_simple_chinese needs coprime moduli and refuses a modulus of 1, so native may return None.
        let fits = n
            .checked_mul(m)
            .and_then(|v| v.checked_mul(l.max(1)))
            .is_some_and(|v| v <= u64::MAX >> 1);
        for len in 0..=3 {
            let expected = math_prime_util::chinese(&pairs[..len]).map(big);
            if fits || len < 3 {
                prop_assert_eq!(chinese(&bigs[..len]), expected);
            } else if let Some(x) = chinese(&bigs[..len]) {
                for (a, n) in &bigs[..len] {
                    prop_assert_eq!(&x % n, a % n);
                }
            }
        }
    }

    #[test]
    fn lucas_agrees_with_native(p: i64, q: i64, k: u64, n in 1_u64..=u64::MAX, small_k in 0_u64..4) {
        for k in [k, small_k] {
            let (bp, bq, bk, bn) = (BigInt::from(p), BigInt::from(q), big(k), big(n));
            prop_assert_eq!(lucasumod(&bp, &bq, &bk, &bn), big(math_prime_util::lucasumod(p, q, k, n)));
            prop_assert_eq!(lucasvmod(&bp, &bq, &bk, &bn), big(math_prime_util::lucasvmod(p, q, k, n)));
        }
    }

    #[test]
    fn carmichael_agrees_with_native(n: u64, small in 0_u64..10_000_000) {
        for n in [n, small, n | 1] {
            prop_assert_eq!(is_carmichael(&big(n)), math_prime_util::is_carmichael(n));
            prop_assert_eq!(is_quasi_carmichael(&big(n)), math_prime_util::is_quasi_carmichael(n));
        }
        // Chernick's form (6k+1)(12k+1)(18k+1) is Carmichael when all three are prime.
        let k = small * 6;
        let (a, b, c) = (6 * k + 1, 12 * k + 1, 18 * k + 1);
        if [a, b, c].iter().all(|&p| math_prime_util::is_prime(p) != 0) {
            prop_assert!(is_carmichael(&(big(a) * big(b) * big(c))));
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    #[test]
    fn identities_on_large_primes(x in 2_u64..=u64::MAX, y in 2_u64..=u64::MAX, shift in 1_u64..200, k in 3_u64..12) {
        // Two primes above 2^64 and a value below their product.
        let p = next_prime(&(big(x) << shift));
        let q = next_prime(&((big(y) << shift) + 1_u8));
        let a = (big(x) << (shift + 60)) % &p + 2_u8;
        // sqrtmod squares back.
        let square = &a * &a % &p;
        let r = sqrtmod(&square, &p).unwrap();
        prop_assert_eq!(&r * &r % &p, square.clone());
        prop_assert!(&r * 2_u8 <= p);
        let mut both = vec![r.clone(), &p - &r];
        both.sort();
        prop_assert_eq!(allsqrtmod(&square, &p), both);
        prop_assert_eq!(rootmod(&square, &big(2), &p), Some(r));
        // rootmod gives a k-th root.
        let power = powmod(&a, &big(k), &p).unwrap();
        let root = rootmod(&power, &big(k), &p).unwrap();
        prop_assert_eq!(powmod(&root, &big(k), &p), Some(power));
        // invmod times a is 1.
        let inverse = invmod(&a, &p).unwrap();
        prop_assert_eq!(&a * &inverse % &p, big(1));
        // The Chinese remainder theorem recovers a value below p * q.
        let value = &a * &q + 1_u8;
        prop_assert_eq!(chinese(&[(&value % &p, p.clone()), (&value % &q, q.clone())]), Some(value));
        // Small factorials and binomials reduce the exact values.
        let n = k * 11;
        prop_assert_eq!(factorialmod(&big(n), &p), Some(factorial(n) % &p));
        prop_assert_eq!(binomialmod(&a, &big(n), &p), Some(binomial(&a, &big(n)).unwrap() % &p));
        // A composite modulus whose factors the recipe finds at once: a
        // product of two large primes would take as long as factoring it.
        let m = &p * 1_000_003_u32;
        prop_assert_eq!(binomialmod(&a, &big(n), &m), Some(binomial(&a, &big(n)).unwrap() % &m));
        // A prime is not Carmichael; two primes are not quasi-Carmichael
        // unless a base works, which the native rule decides.
        prop_assert!(!is_carmichael(&p));
        prop_assert!(!is_carmichael(&(&p * &q)));
    }

    #[test]
    fn znlog_inverts_powmod(x: u64, e in 66_u64..300, g in 2_u64..1000) {
        // Modulo 2^e, 3 has order 2^(e-2); modulo 3^e, 2 has order 2 * 3^(e-1).
        let n = big(1) << e;
        let a = powmod(&big(3), &big(x), &n).unwrap();
        prop_assert_eq!(znlog(&a, &big(3), &n), Some(big(x)));
        let m = big(3).pow(u32::try_from(e).unwrap() / 2);
        let order = &m / 3_u8 * 2_u8;
        let b = powmod(&big(2), &big(x), &m).unwrap();
        prop_assert_eq!(znlog(&b, &big(2), &m), Some(big(x) % &order));
        // A base that shares a factor with the modulus.
        let c = powmod(&big(g), &big(x % 60), &n).unwrap();
        let log = znlog(&c, &big(g), &n);
        prop_assert!(log.is_some());
        prop_assert_eq!(powmod(&big(g), &log.unwrap(), &n), Some(c));
        // Orders and primitive roots of the prime power moduli.
        prop_assert_eq!(znorder(&big(3), &n), Some(big(1) << (e - 2)));
        prop_assert_eq!(znorder(&big(2), &m), Some(order));
        prop_assert_eq!(znprimroot(&n), None);
        prop_assert_eq!(znprimroot(&m), Some(big(2)));
        prop_assert_eq!(is_primitive_root(&big(2), &m), Some(true));
        prop_assert_eq!(is_primitive_root(&big(4), &m), Some(false));
        prop_assert_eq!(is_primitive_root(&big(3), &n), Some(false));
    }
}
