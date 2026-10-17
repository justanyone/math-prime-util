use std::ops::{ControlFlow, RangeInclusive};
use super::run;

/// Iterator over `(n, factors)` for each `n` in a range, where `factors` are the
/// `(prime, exponent)` pairs of `n`. See `XS.xs`, `forfactored`.
#[derive(Debug, Clone)]
pub struct FactoredIn {
    range: RangeInclusive<u64>,
    segment: std::vec::IntoIter<(u64, Vec<(u64, u32)>)>,
    sieve: bool,
}

impl Iterator for FactoredIn {
    type Item = (u64, Vec<(u64, u32)>);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.sieve {
            return self.range.find(|&n| n > 0).map(|n| (n, crate::factor_exp(n)));
        }
        if let Some(value) = self.segment.next() {
            return Some(value);
        }
        let lo = self.range.next()?;
        let end = *self.range.end();
        let hi = lo.saturating_add(256 * 1024 - 1).min(end);
        self.range = if hi == u64::MAX { 1..=0 } else { hi + 1..=end };
        self.segment = factor_segment(lo, hi).into_iter();
        self.segment.next()
    }
}

/// Iterate over `(n, factors)` for each `n` in a range. `0` is not visited.
/// See `XS.xs`, `forfactored`.
/// ```
/// let v: Vec<_> = math_prime_util::factored_in(1..=4).collect();
/// assert_eq!(v[3], (4, vec![(2, 2)]));
/// ```
#[must_use]
pub fn factored_in(range: RangeInclusive<u64>) -> FactoredIn {
    let range = if range.is_empty() { 1..=0 } else { (*range.start()).max(1)..=*range.end() };
    let sieve = !range.is_empty() && range.end() - range.start() >= 100;
    FactoredIn { range, sieve, segment: Vec::new().into_iter() }
}

/// Visit `n` and its factors for each `n` in an inclusive range. See `XS.xs`, `forfactored`.
/// ```
/// use std::ops::ControlFlow;
/// let mut seen = Vec::new();
/// let _: ControlFlow<()> = math_prime_util::forfactored(12..=12, |n, f| {
///     seen.push((n, f));
///     ControlFlow::Continue(())
/// });
/// assert_eq!(seen, [(12, vec![(2, 2), (3, 1)])]);
/// ```
pub fn forfactored<B>(
    range: RangeInclusive<u64>,
    mut f: impl FnMut(u64, Vec<(u64, u32)>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(factored_in(range), |(n, factors)| f(n, factors))
}

