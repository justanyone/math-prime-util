//! MPU real-number oracle cases. Printed doubles use relative tolerance 1e-12.
#![allow(clippy::float_cmp, clippy::cast_precision_loss)]
mod common;
use math_prime_util::*;

fn check(name: &str, f: impl Fn(&common::Case) -> f64 + std::panic::RefUnwindSafe) {
    common::each("real", name, |c| {
        let got = std::panic::catch_unwind(|| f(c));
        if c.error {
            assert!(got.is_err(), "{c:?}");
            return;
        }
        let actual = got.unwrap_or_else(|_| panic!("unexpected panic: {c:?}"));
        let expected: f64 = c.result.as_ref().unwrap().parse().unwrap();
        assert!(
            (actual.is_nan() && expected.is_nan())
                || actual == expected
                || (actual - expected).abs() <= 1e-12 * expected.abs(),
            "{c:?}: got {actual}, expected {expected}"
        );
    });
}
#[test]
fn exponential_integral_oracle() {
    check("ExponentialIntegral", |c| exponential_integral(c.arg(0)));
}
#[test]
fn logarithmic_integral_oracle() {
    check("LogarithmicIntegral", |c| logarithmic_integral(c.arg(0)));
}
#[test]
fn riemann_zeta_oracle() {
    check("RiemannZeta", |c| riemann_zeta(c.arg(0)));
}
#[test]
fn riemann_r_oracle() {
    check("RiemannR", |c| riemann_r(c.arg(0)));
}
#[test]
fn lambert_w_oracle() {
    check("LambertW", |c| lambert_w(c.arg(0)));
}
#[test]
fn inverse_li_oracle() {
    // inverse_li cases whose argument or result exceeds u64 use the Perl bigint
    // fallback. The native C inverse_li API cannot represent those cases.
    common::each("real", "inverse_li", |c| {
        let Some(x) = c.args[0].parse::<u64>().ok() else {
            return;
        };
        if c.result
            .as_ref()
            .and_then(|s| s.parse::<u64>().ok())
            .is_none()
        {
            return;
        }
        let actual = inverse_li(x) as f64;
        let expected: f64 = c.result.as_ref().unwrap().parse().unwrap();
        assert!(
            (actual - expected).abs() <= 1e-12 * expected.abs(),
            "{c:?}: {actual}"
        );
    });
}
#[test]
fn inverse_li_nv_oracle() {
    check("inverse_li_nv", |c| inverse_li_nv(c.arg(0)));
}
#[test]
fn bernreal_oracle() {
    check("bernreal", |c| bernreal(c.arg(0)));
}
#[test]
fn harmreal_oracle() {
    check("harmreal", |c| harmreal(c.arg(0)));
}
#[test]
fn pi_oracle() {
    common::each("real", "Pi", |c| {
        let digits = if c.args.is_empty() { 0 } else { c.arg(0) };
        let actual = pi(digits);
        if digits <= 15 {
            let a: f64 = actual.parse().unwrap();
            let e: f64 = c.result.as_ref().unwrap().parse().unwrap();
            assert!((a - e).abs() <= 1e-12 * e.abs(), "{c:?}: {actual}");
        } else {
            assert_eq!(
                actual.trim_end_matches('0'),
                c.result.as_ref().unwrap().trim_end_matches('0'),
                "{c:?}"
            );
        }
    });
}
