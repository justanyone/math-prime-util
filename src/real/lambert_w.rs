//! Perl-named entry for `LambertW` (real.c, `LambertW`).
use super::lambert_w as impl_lambert_w;

/// Return the principal real Lambert W value under the MPU name.
/// Port of `real.c::LambertW` in MPU 0.75; see [`lambert_w`](super::lambert_w).
/// # Panics
/// Panics below MPU's branch limit, -0.36787944117145.
/// ```
/// assert!((math_prime_util::LambertW(1.0) - 0.567143290409784).abs() < 1e-14);
/// ```
#[allow(non_snake_case)]
pub fn LambertW(x: f64) -> f64 {
    impl_lambert_w(x)
}
