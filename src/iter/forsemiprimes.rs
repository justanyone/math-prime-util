use std::ops::{ControlFlow, RangeInclusive};

/// Semiprime range state from MPU 0.75 `XS.xs::forsemiprimes`.
#[derive(Debug, Clone)]
pub struct SemiprimesIn {
    range: RangeInclusive<u64>,
}

impl Iterator for SemiprimesIn {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.range.find(|&n| crate::is_semiprime(n))
    }
}

/// Iterate over semiprimes in an inclusive range.
///
/// Port of MPU 0.75 `XS.xs::forsemiprimes`, the predicate loop.
/// It uses `util.c::is_semiprime` and counts repeated prime factors.
/// ```
/// assert_eq!(math_prime_util::semiprimes_in(0..=12).collect::<Vec<_>>(), [4, 6, 9, 10]);
/// ```
#[must_use]
pub fn semiprimes_in(range: RangeInclusive<u64>) -> SemiprimesIn {
    if range.is_empty() {
        return SemiprimesIn { range };
    }
    let (lo, hi) = range.into_inner();
    SemiprimesIn {
        range: lo.max(4)..=hi.min(18_446_744_073_709_551_601),
    }
}

/// Visit each semiprime in an inclusive range. See `XS.xs`, `forsemiprimes`.
/// ```
/// use std::ops::ControlFlow;
/// let mut count = 0;
/// let _: ControlFlow<()> = math_prime_util::forsemiprimes(0..=100, |_| {
///     count += 1;
///     ControlFlow::Continue(())
/// });
/// assert_eq!(count, 34);
/// ```
pub fn forsemiprimes<B>(
    range: RangeInclusive<u64>,
    f: impl FnMut(u64) -> ControlFlow<B>,
) -> ControlFlow<B> {
    semiprimes_in(range).try_for_each(f)
}
