//! Perl-named entry for `RiemannR` (real.c, `RiemannR`).
use super::riemann_r as impl_riemann_r;

/// Return Riemann's prime-count estimate under the MPU name.
/// Port of `real.c::RiemannR` in MPU 0.75; see [`riemann_r`](super::riemann_r).
/// # Panics
/// Panics if `x` is not positive.
/// ```
/// assert!((math_prime_util::RiemannR(100.0) - 25.6616332669242).abs() < 1e-12);
/// ```
#[allow(non_snake_case)]
pub fn RiemannR(x: f64) -> f64 {
    impl_riemann_r(x)
}
