//! Perl-named entry for `LogarithmicIntegral` (real.c, `Li`).
use super::logarithmic_integral as impl_li;

/// Return the logarithmic integral under the MPU name.
/// Port of `real.c::Li` in MPU 0.75; see [`logarithmic_integral`](super::logarithmic_integral).
/// # Panics
/// Panics if `x` is negative.
/// ```
/// assert!((math_prime_util::LogarithmicIntegral(2.0) - 1.04516378011749).abs() < 1e-12);
/// ```
#[allow(non_snake_case)]
pub fn LogarithmicIntegral(x: f64) -> f64 {
    impl_li(x)
}
