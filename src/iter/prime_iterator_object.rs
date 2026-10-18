use super::prime_iterator::{PrimeIterator, prime_iterator};

/// Create an owned prime iterator.
///
/// See `reference/mpu/lib/Math/Prime/Util.pm`, `prime_iterator_object`.
/// MPU returns a `Math::Prime::Util::PrimeIterator` object. This function
/// returns the same native stream as [`prime_iterator`].
///
/// ```
/// assert_eq!(
///     math_prime_util::prime_iterator_object(3)
///         .take(3)
///         .collect::<Vec<_>>(),
///     [3, 5, 7]
/// );
/// ```
#[must_use]
pub fn prime_iterator_object(start: u64) -> PrimeIterator {
    prime_iterator(start)
}
