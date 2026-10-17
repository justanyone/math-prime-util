use std::ops::ControlFlow;
use super::{factor_pairs, run};

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

/// Iterate over the divisors of `n` in ascending order. `0` has no divisors here.
/// See `XS.xs`, `fordivisors`.
/// ```
/// assert_eq!(math_prime_util::divisors_of(12).collect::<Vec<_>>(), [1, 2, 3, 4, 6, 12]);
/// ```
#[must_use]
pub fn divisors_of(n: u64) -> DivisorsOf {
    let mut divisors = vec![1u64];
    if n == 0 {
        divisors.clear();
    }
    for (p, e) in factor_pairs(n) {
        let len = divisors.len();
        let mut pk = 1u64;
        for _ in 0..e {
            pk *= p;
            for i in 0..len {
                divisors.push(divisors[i] * pk);
            }
        }
    }
    divisors.sort_unstable();
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
