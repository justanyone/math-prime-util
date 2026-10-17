//! Stateful loops from MPU 0.75 `XS.xs` and `lib/Math/Prime/Util/PP.pm`.

use std::ops::{ControlFlow, RangeInclusive};

mod fordivisors;
pub use fordivisors::{DivisorsOf, divisors_of, fordivisors};
mod forpart;
pub use forpart::{PartitionOptions, PartitionsOf, forpart, partitions_of};
mod forcomp;
pub use forcomp::{CompositionsOf, compositions_of, forcomp};
mod forcomb;
pub use forcomb::{CombinationsOf, combinations_of, forcomb};

pub use crate::sieve::{PrimesIn, primes_in};

fn run<I: Iterator, B>(mut iter: I, f: impl FnMut(I::Item) -> ControlFlow<B>) -> ControlFlow<B> {
    iter.try_for_each(f)
}

/// Stop the current closure loop. See `XS.xs`, `lastfor`.
/// ```
/// use math_prime_util::{forprimes, lastfor};
/// assert!(forprimes(2..=10, |_| lastfor()).is_break());
/// ```
pub fn lastfor() -> ControlFlow<()> {
    ControlFlow::Break(())
}

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
    run(primes_in(range), f)
}

/// Create an iterator at the first prime at least `start`.
/// See `lib/Math/Prime/Util.pm`, `prime_iterator`. The native range ends at `u64::MAX`.
/// ```
/// assert_eq!(math_prime_util::prime_iterator(10).next(), Some(11));
/// ```
pub fn prime_iterator(start: u64) -> PrimeIterator {
    PrimeIterator {
        cursor: Some(start.saturating_sub(1)),
        buffer: Vec::new().into_iter(),
    }
}

/// Create an owned prime iterator. See `lib/Math/Prime/Util.pm`, `prime_iterator_object`.
/// ```
/// assert_eq!(math_prime_util::prime_iterator_object(3).take(3).collect::<Vec<_>>(), [3,5,7]);
/// ```
pub fn prime_iterator_object(start: u64) -> PrimeIterator {
    prime_iterator(start)
}

/// Buffered native prime state from `lib/Math/Prime/Util.pm`, `prime_iterator`.
#[derive(Debug, Clone)]
pub struct PrimeIterator {
    cursor: Option<u64>,
    buffer: std::vec::IntoIter<u64>,
}
impl Iterator for PrimeIterator {
    type Item = u64;
    // Match the reference's floating point estimate for a batch of about 10k primes.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn next(&mut self) -> Option<u64> {
        if let Some(p) = self.buffer.next() {
            self.cursor = Some(p);
            return Some(p);
        }
        let p = self.cursor.take()?;
        let lo = p.checked_add(1)?;
        let segment = if p <= 10_000 {
            10_000
        } else {
            (10_000.0 * (p as f64).ln() + 1.0) as u64
        };
        let hi = p.saturating_add(segment);
        self.buffer = crate::primes(lo, hi).into_iter();
        let next = self.buffer.next()?;
        self.cursor = Some(next);
        Some(next)
    }
}

/// Composite range state. See `XS.xs`, `foroddcomposites`.
#[derive(Debug, Clone)]
pub struct CompositesIn {
    range: RangeInclusive<u64>,
    odd: bool,
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
/// Iterate over odd composites. See `XS.xs`, `foroddcomposites`.
/// ```
/// assert_eq!(math_prime_util::odd_composites_in(0..=15).collect::<Vec<_>>(), [9,15]);
/// ```
pub fn odd_composites_in(range: RangeInclusive<u64>) -> CompositesIn {
    CompositesIn { range, odd: true }
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
    run(composites_in(range), f)
}

/// Visit each value. See `XS.xs`, `foroddcomposites`.
/// ```
/// use std::ops::ControlFlow;
/// let result = math_prime_util::foroddcomposites(0..=20, |_| ControlFlow::Break(()));
/// assert!(result.is_break());
/// ```
pub fn foroddcomposites<B>(
    range: RangeInclusive<u64>,
    f: impl FnMut(u64) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(odd_composites_in(range), f)
}

/// Factor `n` by trial division. Returns `(prime, exponent)` pairs in ascending order.
/// `0` and `1` have no factors. Trial division is the first stage of the factoring that
/// `XS.xs`, `forfactored`, calls for each value.
fn factor_pairs(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    if n < 2 {
        return out;
    }
    let mut p = 2u64;
    while p <= n / p {
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            out.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

/// Count the prime factors of `n` with multiplicity. See `XS.xs`, `foralmostprimes`.
fn big_omega(n: u64) -> u32 {
    factor_pairs(n).iter().map(|&(_, e)| e).sum()
}

/// Iterator over the integers in a range that have exactly `k` prime factors,
/// counted with multiplicity. See `XS.xs`, `foralmostprimes`.
#[derive(Debug, Clone)]
pub struct AlmostPrimesIn {
    k: u32,
    range: RangeInclusive<u64>,
}

impl Iterator for AlmostPrimesIn {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.k == 0 {
            return None;
        }
        let k = self.k;
        self.range.find(move |&n| n >= 2 && big_omega(n) == k)
    }
}

/// Iterate over the semiprimes in an inclusive range. See `XS.xs`, `forsemiprimes`.
/// ```
/// assert_eq!(math_prime_util::semiprimes_in(0..=12).collect::<Vec<_>>(), [4, 6, 9, 10]);
/// ```
#[must_use]
pub fn semiprimes_in(range: RangeInclusive<u64>) -> AlmostPrimesIn {
    AlmostPrimesIn { k: 2, range }
}

/// Iterate over the almost primes with `k` factors in an inclusive range.
/// See `XS.xs`, `foralmostprimes`.
/// ```
/// assert_eq!(math_prime_util::almost_primes_in(3, 0..=30).collect::<Vec<_>>(), [8, 12, 18, 20, 27, 28, 30]);
/// ```
#[must_use]
pub fn almost_primes_in(k: u32, range: RangeInclusive<u64>) -> AlmostPrimesIn {
    AlmostPrimesIn { k, range }
}

/// Iterator over `(n, factors)` for each `n` in a range, where `factors` are the
/// `(prime, exponent)` pairs of `n`. See `XS.xs`, `forfactored`.
#[derive(Debug, Clone)]
pub struct FactoredIn {
    range: RangeInclusive<u64>,
}

impl Iterator for FactoredIn {
    type Item = (u64, Vec<(u64, u32)>);

    fn next(&mut self) -> Option<Self::Item> {
        self.range.find(|&n| n > 0).map(|n| (n, factor_pairs(n)))
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
    FactoredIn { range }
}

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
            let factors = factor_pairs(n);
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
    run(semiprimes_in(range), f)
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
    mut f: impl FnMut(u64) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(squarefree_in(range), |(n, _)| f(n))
}

mod forperm;
pub use forperm::{PermutationsOf, forperm, permutations_of};

mod formultiperm;
pub use formultiperm::{MultipermutationsOf, formultiperm, multipermutations_of};

mod forderange;
pub use forderange::{DerangementsOf, derangements_of, forderange};

mod forsetproduct;
pub use forsetproduct::{SetProduct, forsetproduct, set_product};
