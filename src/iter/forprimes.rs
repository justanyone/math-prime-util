use std::ops::{ControlFlow, RangeInclusive};

/// Visit primes in an inclusive range. See `XS.xs`, `forprimes`.
/// ```
/// use math_prime_util::forprimes;
/// use std::ops::ControlFlow;
/// let mut sum = 0;
/// let _: ControlFlow<()> = forprimes(0..=7, |p| { sum += p; ControlFlow::Continue(()) });
/// assert_eq!(sum, 17);
/// ```
pub fn forprimes<B>(
    range: RangeInclusive<u64>,
    f: impl FnMut(u64) -> ControlFlow<B>,
) -> ControlFlow<B> {
    crate::sieve::primes_in(range).try_for_each(f)
}
