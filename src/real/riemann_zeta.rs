//! Perl-named entry for `RiemannZeta` (real.c, `ld_riemann_zeta`).
use super::riemann_zeta as impl_zeta;

/// Return zeta(x) minus one under the MPU name.
/// Port of `real.c::ld_riemann_zeta` in MPU 0.75; see [`riemann_zeta`](super::riemann_zeta).
/// # Panics
/// Panics if `x` is negative.
/// ```
/// assert!((math_prime_util::RiemannZeta(2.0) - 0.6449340668482264).abs() < 1e-15);
/// ```
#[allow(non_snake_case)]
pub fn RiemannZeta(x: f64) -> f64 {
    impl_zeta(x)
}
