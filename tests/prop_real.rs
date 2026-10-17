//! Algebraic laws and validation for stateless real functions.
#![allow(clippy::float_cmp, clippy::cast_precision_loss)]
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
    fn harmonic_recurrence(n in 1_u32..100_000) {
        let d = harmreal(u64::from(n))-harmreal(u64::from(n-1));
        prop_assert!((d-1.0/f64::from(n)).abs() < 1e-14);
    }
    #[test]
    fn odd_bernoulli(n in 1_u32..1000) { prop_assert_eq!(bernreal(2*n+1), 0.0); }
    #[test]
    fn integer_inverse_bracket(n in 1_u32..1_000_000) {
        let r = inverse_li(u64::from(n));
        prop_assert!(logarithmic_integral(r as f64) >= f64::from(n));
        prop_assert!(logarithmic_integral((r-1) as f64) < f64::from(n));
    }
}
#[test]
fn invalid_inputs_panic() {
    assert_eq!(exponential_integral(0.0), f64::NEG_INFINITY);
    for f in [
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
    let expected = (pi(100), bernreal(100), harmreal(10_000), riemann_r(1e30));
    std::thread::scope(|s| {
        for _ in 0..4 {
            let expected = &expected;
            s.spawn(move || {
                for _ in 0..5 {
                    assert_eq!(
                        &(pi(100), bernreal(100), harmreal(10_000), riemann_r(1e30)),
                        expected
                    );
                }
            });
        }
    });
}

/// Sample the three real functions. Same inputs must give the same bits.
fn ei_li_zeta_sample() -> Vec<u64> {
    let mut out = Vec::new();
    for x in [0.25_f64, 0.5, 2.0, 3.5, 10.0, 40.0, 100.0] {
        out.push(ExponentialIntegral(x).to_bits());
        out.push(LogarithmicIntegral(x).to_bits());
        out.push(RiemannZeta(x).to_bits());
    }
    out
}

#[test]
fn ei_li_zeta_match_definitions() {
    // li(x) = Ei(ln x) away from the pole at 1. zeta(2) - 1 = pi^2/6 - 1.
    let pi2_6 = std::f64::consts::PI * std::f64::consts::PI / 6.0;
    let z2 = RiemannZeta(2.0);
    assert!((z2 - (pi2_6 - 1.0)).abs() < 1e-15, "{z2}");
    for x in [1e-6, 1e-3, 0.1, 0.5, 0.9, 1.2, 2.0, 10.0, 1e3, 1e6, 1e12] {
        let li = LogarithmicIntegral(x);
        let ei = ExponentialIntegral(x.ln());
        let scale = li.abs().max(1.0);
        assert!((li - ei).abs() <= 1e-9 * scale, "x={x} li={li} ei={ei}");
    }
    // zeta(s) - 1 is the tail sum n^(-s) from 2, so it sits above 2^(-s).
    for s in [1.5, 2.0, 3.5, 8.0, 20.0] {
        let z = RiemannZeta(s);
        let two = 2.0_f64.powf(-s);
        let upper = two + 2.0_f64.powf(1.0 - s) / (s - 1.0);
        assert!(z > two && z < upper, "s={s} z={z} two={two} upper={upper}");
    }
}

#[test]
fn ei_li_zeta_are_monotone_where_defined() {
    let mut prev_ei = ExponentialIntegral(0.2);
    let mut prev_li = LogarithmicIntegral(1.2);
    let mut prev_z = RiemannZeta(1.5);
    for i in 1..20 {
        let t = 0.2 + f64::from(i) * 0.4;
        let ei = ExponentialIntegral(t);
        assert!(ei > prev_ei, "{t}");
        prev_ei = ei;
        let u = 1.2 + f64::from(i);
        let li = LogarithmicIntegral(u);
        assert!(li > prev_li, "{u}");
        prev_li = li;
        let s = 1.5 + f64::from(i) * 0.5;
        let z = RiemannZeta(s);
        assert!(z < prev_z, "{s}");
        prev_z = z;
    }
}

#[test]
fn ei_derivative_matches_exp_over_x() {
    // d/dx Ei(x) = exp(x) / x. A central difference must follow that.
    for x in [0.5, 1.0, 2.0, 5.0, 12.0] {
        let h = 1e-6;
        let slope = (ExponentialIntegral(x + h) - ExponentialIntegral(x - h)) / (2.0 * h);
        let exact = x.exp() / x;
        assert!(
            (slope - exact).abs() <= 1e-6 * exact.abs(),
            "x={x} slope={slope} exact={exact}"
        );
    }
}

#[test]
fn ei_li_zeta_input_validation() {
    // Perl Ei(0) is -infinity. C and Perl both reject a negative li or zeta.
    assert_eq!(ExponentialIntegral(0.0), f64::NEG_INFINITY);
    assert_eq!(ExponentialIntegral(-0.0), f64::NEG_INFINITY);
    assert!(ExponentialIntegral(f64::NAN).is_nan());
    assert_eq!(ExponentialIntegral(f64::INFINITY), f64::INFINITY);
    assert_eq!(ExponentialIntegral(f64::NEG_INFINITY), 0.0);
    assert_eq!(ExponentialIntegral(12_000.0), f64::INFINITY);
    assert_eq!(ExponentialIntegral(-12_000.0), 0.0);
    assert_eq!(LogarithmicIntegral(0.0), 0.0);
    assert_eq!(LogarithmicIntegral(1.0), f64::NEG_INFINITY);
    assert!(LogarithmicIntegral(f64::NAN).is_nan());
    assert_eq!(LogarithmicIntegral(f64::INFINITY), f64::INFINITY);
    assert!(RiemannZeta(f64::NAN).is_nan());
    assert_eq!(RiemannZeta(0.0), -1.5);
    assert_eq!(RiemannZeta(1.0), f64::INFINITY);
    assert_eq!(RiemannZeta(f64::INFINITY), 0.0);
    assert_eq!(RiemannZeta(17_001.0), 0.0);
    for f in [
        || LogarithmicIntegral(-1.0),
        || LogarithmicIntegral(-0.5),
        || LogarithmicIntegral(f64::NEG_INFINITY),
        || RiemannZeta(-1.0),
        || RiemannZeta(-0.5),
        || RiemannZeta(f64::NEG_INFINITY),
    ] {
        assert!(std::panic::catch_unwind(f).is_err());
    }
}

#[test]
fn ei_li_zeta_repeat_and_concurrent_calls() {
    // The functions keep no state. A second launch and other threads match.
    let once = ei_li_zeta_sample();
    assert_eq!(ei_li_zeta_sample(), once);
    std::thread::scope(|s| {
        for _ in 0..4 {
            let once = &once;
            s.spawn(move || {
                for _ in 0..5 {
                    assert_eq!(ei_li_zeta_sample(), *once);
                }
            });
        }
    });
}
