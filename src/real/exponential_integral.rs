//! Perl-named entry for `ExponentialIntegral` (real.c, `Ei`).
use super::exponential_integral as impl_ei;

/// Return the real exponential integral under the MPU name.
/// Port of `real.c::Ei` in MPU 0.75; see [`exponential_integral`](super::exponential_integral).
/// Zero gives negative infinity, as in the Perl front end.
/// ```
/// assert!((math_prime_util::ExponentialIntegral(1.0) - 1.89511781635594).abs() < 1e-12);
/// ```
#[allow(non_snake_case)]
pub fn ExponentialIntegral(x: f64) -> f64 {
    impl_ei(x)
}
