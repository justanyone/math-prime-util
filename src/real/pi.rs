//! Perl-named entry for `Pi` (util.c, `pidigits`, exported as `Pi`).
use super::pi as impl_pi;

/// Return pi rounded to the given number of significant digits under the MPU name.
/// Port of `util.c::pidigits` in MPU 0.75; see [`pi`](super::pi).
/// # Panics
/// Panics if `digits` exceeds the C limit of 306783370.
/// ```
/// assert_eq!(math_prime_util::Pi(5), "3.1416");
/// ```
#[allow(non_snake_case)]
pub fn Pi(digits: u32) -> String {
    impl_pi(digits)
}
