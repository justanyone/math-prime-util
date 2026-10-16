#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
//! Property tests: algebraic laws that must hold for every input.

use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 4000,
        failure_persistence: Some(Box::new(
            proptest::test_runner::FileFailurePersistence::WithSource("proptest-regressions"),
        )),
        ..ProptestConfig::default()
    })]

    #[test]
    fn sqrtint_bounds(n in any::<u64>()) {
        let r = sqrtint(n);
        prop_assert!(r * r <= n);
        prop_assert!((r + 1).checked_mul(r + 1).is_none_or(|s| s > n));
    }

    #[test]
    fn rootint_bounds(n in any::<u64>(), k in 1u32..=80) {
        let r = rootint(n, k);
        prop_assert!(r.checked_pow(k).is_some_and(|p| p <= n), "r={r}");
        prop_assert!((r + 1).checked_pow(k).is_none_or(|p| p > n), "r={r}");
        let c = crootint(n, k);
        prop_assert!(c.checked_pow(k).is_none_or(|p| p >= n), "c={c}");
        prop_assert!(c == 0 || (c - 1).checked_pow(k).is_some_and(|p| p < n), "c={c}");
    }

    #[test]
    fn powint_is_checked_pow(n in any::<u64>(), k in 0u32..=130) {
        prop_assert_eq!(powint(n, k), n.checked_pow(k));
    }

    #[test]
    fn is_power_of_known_power(r in 2u64..=65535, k in 2u32..=16) {
        // Keep r^k inside u64 without rejecting the sample.
        let r = r.min(u64::from(f64::from(u64::MAX as f32).powf(1.0 / f64::from(k)) as u32).max(2));
        let Some(n) = r.checked_pow(k) else { return Ok(()); };
        prop_assert!(is_power(n, k));
        prop_assert!(int::is_power_exp(n) >= k);
        prop_assert!(int::is_power_exp(n) % k == 0 || int::is_power_exp(n) == k);
    }

    #[test]
    fn logint_bounds(n in 1u64.., b in 2u64..) {
        let e = logint(n, b);
        prop_assert!(b.checked_pow(e).is_some_and(|p| p <= n));
        prop_assert!(b.checked_pow(e + 1).is_none_or(|p| p > n));
    }

    #[test]
    fn gcd_laws(a in any::<u64>(), b in any::<u64>()) {
        let g = gcd(a, b);
        prop_assert_eq!(g, gcd(b, a));
        if a != 0 || b != 0 {
            prop_assert!(g > 0);
            prop_assert_eq!(a % g, 0);
            prop_assert_eq!(b % g, 0);
            prop_assert_eq!(gcd(a / g, b / g), 1);
        }
        if let Some(l) = lcm(a, b) {
            if a != 0 && b != 0 {
                prop_assert_eq!(u128::from(l) * u128::from(g), u128::from(a) * u128::from(b));
            }
        }
    }

    #[test]
    fn gcdext_bezout(a in any::<i64>(), b in any::<i64>()) {
        prop_assume!(a != i64::MIN && b != i64::MIN);
        let (g, u, v) = gcdext(a, b);
        prop_assert_eq!(g.unsigned_abs(), gcd(a.unsigned_abs(), b.unsigned_abs()));
        prop_assert!(g >= 0);
        prop_assert_eq!(i128::from(a) * i128::from(u) + i128::from(b) * i128::from(v), i128::from(g));
    }

    #[test]
    fn modops_laws(a in any::<u64>(), b in any::<u64>(), n in 1u64..) {
        let big = |x: u64| u128::from(x);
        let m = big(n);
        prop_assert_eq!(big(addmod(a, b, n).unwrap()), (big(a) + big(b)) % m);
        prop_assert_eq!(big(submod(a, b, n).unwrap()), (big(a) + m - big(b) % m) % m);
        prop_assert_eq!(big(mulmod(a, b, n).unwrap()), (big(a) * big(b)) % m);
        prop_assert_eq!(big(muladdmod(a, b, 5, n).unwrap()), (big(a) * big(b) + 5) % m);
        prop_assert_eq!(big(mulsubmod(a, b, 5, n).unwrap()), (big(a) * big(b) + 5 * m - 5) % m);
        prop_assert_eq!(addmod(a, negmod(a, n).unwrap(), n), Some(0));
        prop_assert_eq!(powmod(a, 1, n), Some(a % n));
        prop_assert_eq!(powmod(a, 2, n), mulmod(a, a, n));
        if let Some(x) = invmod(a, n) {
            prop_assert_eq!(mulmod(a, x, n), Some(1 % n));
        } else {
            prop_assert_ne!(gcd(a, n), 1);
        }
    }

    #[test]
    fn powmod_vs_square_and_multiply(a in any::<u64>(), k in 0u64..10_000, n in 1u64..) {
        let mut expect = 1 % n;
        for _ in 0..k {
            expect = mulmod(expect, a, n).unwrap();
        }
        prop_assert_eq!(powmod(a, k, n), Some(expect));
    }
}
