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
            self.range = if hi == u64::MAX {
                RangeInclusive::new(1, 0)
            } else {
                hi + 1..=end
            };
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
        RangeInclusive::new(1, 0)
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
    const PRIME_BLOCK: u64 = 65_536;
    let len = usize::try_from(hi - lo + 1).expect("bounded sieve segment");
    if len < 4_096 && hi > (1_u64 << 40) {
        return (lo..=hi).filter(|&n| crate::is_square_free(n)).collect();
    }
    let mut is_free = vec![true; len];
    let limit = crate::sqrtint(hi);
    let base = crate::primes_in(2..=crate::sqrtint(limit)).collect::<Vec<_>>();
    let mut block_lo = 2_u64;
    while block_lo <= limit {
        let block_hi = limit.min(block_lo.saturating_add(PRIME_BLOCK - 1));
        let block_len = usize::try_from(block_hi - block_lo + 1).expect("prime block");
        let mut candidates = vec![true; block_len];
        for &p in &base {
            let square = p * p;
            let rem = block_lo % p;
            let first = if rem == 0 {
                block_lo
            } else {
                block_lo + (p - rem)
            };
            let mut multiple = first.max(square);
            while multiple <= block_hi {
                candidates[usize::try_from(multiple - block_lo).expect("prime index")] = false;
                if block_hi - multiple < p {
                    break;
                }
                multiple += p;
            }
        }
        for (offset, prime) in candidates.into_iter().enumerate() {
            if prime {
                let p = block_lo + u64::try_from(offset).expect("prime offset");
                if p < 2 {
                    continue;
                }
                let square = p * p;
                let rem = lo % square;
                let first = if rem == 0 { lo } else { lo + (square - rem) };
                let mut multiple = first.max(square);
                while multiple <= hi {
                    is_free[usize::try_from(multiple - lo).expect("bounded sieve index")] = false;
                    if hi - multiple < square {
                        break;
                    }
                    multiple += square;
                }
            }
        }
        if block_hi == limit {
            break;
        }
        block_lo = block_hi + 1;
    }
    is_free
        .into_iter()
        .zip(lo..=hi)
        .filter_map(|(free, n)| free.then_some(n))
        .collect()
}
