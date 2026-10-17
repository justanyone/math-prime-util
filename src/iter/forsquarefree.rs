use std::ops::{ControlFlow, RangeInclusive};
use super::run;

/// Iterator over the squarefree `n` in a range, with their factors. See `XS.xs`, `forfactored`.
#[derive(Debug, Clone)]
pub struct SquarefreeIn {
    range: RangeInclusive<u64>,
}

impl Iterator for SquarefreeIn {
    type Item = (u64, Vec<(u64, u32)>);

    fn next(&mut self) -> Option<Self::Item> {
        self.range.find_map(|n| {
            if n == 0 {
                return None;
            }
            let factors = crate::factor_exp(n);
            factors.iter().all(|&(_, e)| e == 1).then_some((n, factors))
        })
    }
}

/// Iterate over the squarefree integers in a range. See `XS.xs`, `forsquarefree`.
/// ```
/// let v: Vec<_> = math_prime_util::squarefree_in(1..=10).map(|(n, _)| n).collect();
/// assert_eq!(v, [1, 2, 3, 5, 6, 7, 10]);
/// ```
#[must_use]
pub fn squarefree_in(range: RangeInclusive<u64>) -> SquarefreeIn {
    SquarefreeIn { range }
}

/// Visit `n` and its factors for each squarefree `n` in an inclusive range.
/// See `XS.xs`, `forsquarefree`.
/// ```
/// use std::ops::ControlFlow;
/// let mut count = 0;
/// let _: ControlFlow<()> = math_prime_util::forsquarefree(1..=10, |_, _| {
///     count += 1;
///     ControlFlow::Continue(())
/// });
/// assert_eq!(count, 7);
/// ```
pub fn forsquarefree<B>(
    range: RangeInclusive<u64>,
    mut f: impl FnMut(u64, Vec<(u64, u32)>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(squarefree_in(range), |(n, factors)| f(n, factors))
}

