//! Algebraic laws and validation for stateless real functions.
use math_prime_util::*;
use proptest::prelude::*;
proptest! {
    #[test]
    fn lambert_inverse(x in -0.36_f64..1e100) {
        let w = lambert_w(x);
        prop_assert!((w*w.exp()-x).abs() <= 1e-12*x.abs().max(1.0));
    }
    #[test]
    fn li_inverse(x in 0.0_f64..1e12) {
        let t = inverse_li_nv(x);
        prop_assert!((logarithmic_integral(t)-x).abs() <= 1e-12*x.max(1.0));
    }
    #[test]
    fn harmonic_recurrence(n in 1_u32..100000) {
        let d = harmreal(u64::from(n))-harmreal(u64::from(n-1));
        prop_assert!((d-1.0/f64::from(n)).abs() < 1e-14);
    }
    #[test]
    fn odd_bernoulli(n in 1_u32..1000) { prop_assert_eq!(bernreal(2*n+1), 0.0); }
    #[test]
    fn integer_inverse_bracket(n in 1_u32..1000000) {
        let r = inverse_li(u64::from(n));
        prop_assert!(logarithmic_integral(r as f64) >= f64::from(n));
        prop_assert!(logarithmic_integral((r-1) as f64) < f64::from(n));
    }
}
#[test]
fn invalid_inputs_panic() {
    for f in [
        || exponential_integral(0.0),
        || logarithmic_integral(-1.0),
        || riemann_zeta(-1.0),
        || riemann_r(0.0),
        || lambert_w(-1.0),
        || inverse_li_nv(-1.0),
        || inverse_li_nv(f64::INFINITY),
        || inverse_li_nv(f64::NAN),
    ] {
        assert!(std::panic::catch_unwind(f).is_err());
    }
    assert!(std::panic::catch_unwind(|| pi(u32::MAX)).is_err());
}
#[test]
fn repeated_and_concurrent_calls() {
    let expected = (pi(100), bernreal(100), harmreal(10000), riemann_r(1e30));
    std::thread::scope(|s| {
        for _ in 0..4 {
            let expected = &expected;
            s.spawn(move || {
                for _ in 0..5 {
                    assert_eq!(
                        &(pi(100), bernreal(100), harmreal(10000), riemann_r(1e30)),
                        expected
                    );
                }
            });
        }
    });
}
