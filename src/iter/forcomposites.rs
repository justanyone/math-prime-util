use std::ops::{ControlFlow, RangeInclusive};

/// Composite range state. See `XS.xs`, `foroddcomposites`.
#[derive(Debug, Clone)]
pub struct CompositesIn {
    pub(super) range: RangeInclusive<u64>,
    pub(super) odd: bool,
}
impl Iterator for CompositesIn {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.range
            .find(|&n| n >= 4 && (!self.odd || n % 2 == 1) && crate::is_prime(n) == 0)
    }
}

/// Iterate over composites. See `XS.xs`, `forcomposites`.
/// ```
/// assert_eq!(math_prime_util::composites_in(0..=9).collect::<Vec<_>>(), [4,6,8,9]);
/// ```
pub fn composites_in(range: RangeInclusive<u64>) -> CompositesIn {
    CompositesIn { range, odd: false }
}
/// Visit each value. See `XS.xs`, `forcomposites`.
/// ```
/// use std::ops::ControlFlow;
/// let result = math_prime_util::forcomposites(0..=20, |_| ControlFlow::Break(()));
/// assert!(result.is_break());
/// ```
pub fn forcomposites<B>(
    range: RangeInclusive<u64>,
    f: impl FnMut(u64) -> ControlFlow<B>,
) -> ControlFlow<B> {
    composites_in(range).try_for_each(f)
}

