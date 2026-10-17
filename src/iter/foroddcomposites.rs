use super::CompositesIn;
use std::ops::{ControlFlow, RangeInclusive};

/// Iterate over odd composites. See `XS.xs`, `foroddcomposites`.
/// ```
/// assert_eq!(math_prime_util::odd_composites_in(0..=15).collect::<Vec<_>>(), [9,15]);
/// ```
pub fn odd_composites_in(range: RangeInclusive<u64>) -> CompositesIn {
    CompositesIn { range, odd: true }
}

/// Visit each value. See `XS.xs`, `foroddcomposites`.
/// ```
/// use std::ops::ControlFlow;
/// let result = math_prime_util::foroddcomposites(0..=20, |_| ControlFlow::Break(()));
/// assert!(result.is_break());
/// ```
pub fn foroddcomposites<B>(
    range: RangeInclusive<u64>,
    f: impl FnMut(u64) -> ControlFlow<B>,
) -> ControlFlow<B> {
    odd_composites_in(range).try_for_each(f)
}

