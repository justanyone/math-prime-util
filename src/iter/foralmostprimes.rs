use std::ops::{ControlFlow, RangeInclusive};
use super::run;

/// Iterator over the integers in a range that have exactly `k` prime factors,
/// counted with multiplicity. See `XS.xs`, `foralmostprimes`.
#[derive(Debug, Clone)]
pub struct AlmostPrimesIn {
    k: u32,
    range: RangeInclusive<u64>,
    shift: u32,
}

impl Iterator for AlmostPrimesIn {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.k == 0 || self.k >= 64 {
            return None;
        }
        let k = self.k;
        self.range.find(move |&n| crate::is_almost_prime(u64::from(k), n))
            .map(|n| n << self.shift)
    }
}

/// Iterate over the almost primes with `k` factors in an inclusive range.
/// See `XS.xs`, `foralmostprimes`.
/// ```
/// assert_eq!(math_prime_util::almost_primes_in(3, 0..=30).collect::<Vec<_>>(), [8, 12, 18, 20, 27, 28, 30]);
/// ```
#[must_use]
pub fn almost_primes_in(k: u32, range: RangeInclusive<u64>) -> AlmostPrimesIn {
    // XS.xs::foralmostprimes reduces k when every result must be even.
    if range.is_empty() || k == 0 || k >= 64 {
        return AlmostPrimesIn { k: 0, range: 1..=0, shift: 0 };
    }
    let mut lo = (*range.start()).max(1_u64 << k);
    let mut hi = *range.end();
    let mut shift = k.saturating_sub(40);
    while k - shift > 1 && (hi >> shift) < 3_u64.pow(k - shift) {
        shift += 1;
    }
    lo = (lo >> shift) + u64::from(((lo >> shift) << shift) < lo);
    hi >>= shift;
    AlmostPrimesIn { k: k - shift, range: lo..=hi, shift }
}

/// Visit each almost prime with `k` factors in an inclusive range. See `XS.xs`, `foralmostprimes`.
/// ```
/// use std::ops::ControlFlow;
/// let result = math_prime_util::foralmostprimes(2, 0..=100, |n| {
///     if n > 10 { ControlFlow::Break(n) } else { ControlFlow::Continue(()) }
/// });
/// assert_eq!(result, ControlFlow::Break(14));
/// ```
pub fn foralmostprimes<B>(
    k: u32,
    range: RangeInclusive<u64>,
    f: impl FnMut(u64) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(almost_primes_in(k, range), f)
}

