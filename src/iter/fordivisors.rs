use super::run;
use std::ops::ControlFlow;

/// Iterator over the divisors of `n` in ascending order. See `XS.xs`, `fordivisors`.
#[derive(Debug, Clone)]
pub struct DivisorsOf {
    divisors: std::vec::IntoIter<u64>,
}

impl Iterator for DivisorsOf {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.divisors.next()
    }
}

/// Iterate over the divisors of `n` in ascending order. Zero has no divisors.
/// Port of `XS.xs::fordivisors` and `factor.c::divisor_list`.
/// See `XS.xs`, `fordivisors`.
/// ```
/// assert_eq!(math_prime_util::divisors_of(12).collect::<Vec<_>>(), [1, 2, 3, 4, 6, 12]);
/// ```
#[must_use]
pub fn divisors_of(n: u64) -> DivisorsOf {
    let divisors = crate::divisors(n, None);
    DivisorsOf {
        divisors: divisors.into_iter(),
    }
}

/// Visit each divisor of `n` in ascending order. See `XS.xs`, `fordivisors`.
/// ```
/// use std::ops::ControlFlow;
/// let result = math_prime_util::fordivisors(12, |d| {
///     if d > 3 { ControlFlow::Break(d) } else { ControlFlow::Continue(()) }
/// });
/// assert_eq!(result, ControlFlow::Break(4));
/// ```
pub fn fordivisors<B>(n: u64, f: impl FnMut(u64) -> ControlFlow<B>) -> ControlFlow<B> {
    run(divisors_of(n), f)
}
