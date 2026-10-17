use super::run;
use std::ops::{ControlFlow, RangeInclusive};

/// Visit each squarefree integer in an inclusive range. See `XS.xs`, `forsquarefreeint`.
/// ```
/// use std::ops::ControlFlow;
/// let mut sum = 0;
/// let _: ControlFlow<()> = math_prime_util::forsquarefreeint(1..=10, |n| {
///     sum += n;
///     ControlFlow::Continue(())
/// });
/// assert_eq!(sum, 34);
/// ```
pub fn forsquarefreeint<B>(
    range: RangeInclusive<u64>,
    f: impl FnMut(u64) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(squarefree_integers_in(range), f)
}

/// State for the squarefree sieve in `XS.xs::forsquarefreeint`.
#[derive(Clone, Debug)]
pub struct SquarefreeIntegersIn {
    range: RangeInclusive<u64>,
    segment: std::vec::IntoIter<u64>,
}

impl Iterator for SquarefreeIntegersIn {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            if let Some(n) = self.segment.next() {
                return Some(n);
            }
            let lo = self.range.next()?;
            let end = *self.range.end();
            // XS.xs uses this bound and chunk size, including its strict > test.
            let hi = if end - lo > 65_536 * 256 {
                lo + 65_536 * 256 - 1
            } else {
                end
            };
            self.range = if hi == u64::MAX { 1..=0 } else { hi + 1..=end };
            self.segment = squarefree_segment(lo, hi).into_iter();
        }
    }
}

/// Iterate over squarefree integers with the MPU square sieve.
///
/// Port of MPU 0.75 `XS.xs::forsquarefreeint` and `util.c::range_issquarefree`.
/// ```
/// assert_eq!(math_prime_util::squarefree_integers_in(0..=10).collect::<Vec<_>>(),
///            [1, 2, 3, 5, 6, 7, 10]);
/// ```
#[must_use]
pub fn squarefree_integers_in(range: RangeInclusive<u64>) -> SquarefreeIntegersIn {
    let range = if range.is_empty() {
        1..=0
    } else {
        (*range.start()).max(1)..=*range.end()
    };
    SquarefreeIntegersIn {
        range,
        segment: Vec::new().into_iter(),
    }
}

// util.c::range_issquarefree marks multiples of each prime square.
fn squarefree_segment(lo: u64, hi: u64) -> Vec<u64> {
    let len = usize::try_from(hi - lo + 1).expect("bounded sieve segment");
    let mut is_free = vec![true; len];
    for p in crate::primes_in(2..=crate::sqrtint(hi)) {
        let square = p * p;
        let offset = (square - lo % square) % square;
        let mut i = offset;
        while i <= hi - lo {
            is_free[usize::try_from(i).expect("bounded sieve index")] = false;
            i += square;
        }
    }
    is_free
        .into_iter()
        .zip(lo..=hi)
        .filter_map(|(free, n)| free.then_some(n))
        .collect()
}
