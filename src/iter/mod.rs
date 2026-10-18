//! Stateful loops from MPU 0.75 `XS.xs` and `lib/Math/Prime/Util/PP.pm`.
mod forsquarefreeint;
pub use forsquarefreeint::{SquarefreeIntegersIn, forsquarefreeint, squarefree_integers_in};
mod forsquarefree;
pub use forsquarefree::{SquarefreeIn, forsquarefree, squarefree_in};
mod forfactored;
pub use forfactored::{FactoredIn, factored_in, forfactored};
mod foralmostprimes;
pub use foralmostprimes::{AlmostPrimesIn, almost_primes_in, foralmostprimes};

use std::ops::ControlFlow;

mod forprimes;
pub use forprimes::forprimes;
mod forcomposites;
pub use forcomposites::{CompositesIn, composites_in, forcomposites};
mod foroddcomposites;
pub use foroddcomposites::{foroddcomposites, odd_composites_in};
mod forsemiprimes;
pub use forsemiprimes::{SemiprimesIn, forsemiprimes, semiprimes_in};

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

/// Bounds for parts and lengths. See `XS.xs`, `forpart`.
#[derive(Debug, Clone, Copy, Default)]
pub struct PartitionOptions {
    /// Minimum part, clamped to one.
    pub amin: Option<u64>,
    /// Maximum part.
    pub amax: Option<u64>,
    /// Exact number of parts. Length bounds take precedence.
    pub n: Option<u64>,
    /// Minimum number of parts.
    pub nmin: Option<u64>,
    /// Maximum number of parts.
    pub nmax: Option<u64>,
    /// Require prime parts if true, nonprime parts if false.
    pub prime: Option<bool>,
}

/// `RuleAsc` state from `XS.xs`, `forpart` and `forcomp`.
#[derive(Debug, Clone)]
pub struct PartitionsOf {
    a: Vec<u64>,
    k: usize,
    n: u64,
    options: PartitionOptions,
    composition: bool,
    empty: bool,
    done: bool,
}
impl Iterator for PartitionsOf {
    type Item = Vec<u64>;
    fn next(&mut self) -> Option<Self::Item> {
        let amin = self.options.amin.unwrap_or(1).max(1);
        let amax = self.options.amax.unwrap_or(self.n).min(self.n);
        let nmin = self.options.nmin.or(self.options.n).unwrap_or(1).max(1);
        let nmax = self
            .options
            .nmax
            .or(self.options.n)
            .unwrap_or(self.n)
            .min(self.n);
        if self.empty {
            self.empty = false;
            return Some(Vec::new());
        }
        while !self.done && self.k != 0 {
            let mut x = self.a[self.k - 1] + 1;
            let mut y = self.a[self.k] - 1;
            self.k -= 1;
            let r = if self.composition { 1 } else { x };
            while r <= y {
                self.a[self.k] = x;
                self.k += 1;
                x = r;
                y -= x;
            }
            self.a[self.k] = x + y;
            while self.k as u64 + 1 > nmax && self.k > 0 {
                self.a[self.k - 1] += self.a[self.k];
                self.k -= 1;
            }
            if self.a[0] > amax {
                self.done = true;
                return None;
            }
            let out = &self.a[..=self.k];
            if (out.len() as u64) < nmin || (out.len() as u64) > nmax {
                continue;
            }
            if out.iter().any(|&v| {
                v < amin
                    || v > amax
                    || self
                        .options
                        .prime
                        .is_some_and(|p| (crate::is_prime(v) != 0) != p)
            }) {
                continue;
            }
            return Some(out.to_vec());
        }
        None
    }
}
fn parts(n: u64, options: PartitionOptions, composition: bool) -> PartitionsOf {
    assert!(n <= u64::MAX - 2, "partition argument overflow");
    let size = usize::try_from(n).expect("partition size must fit usize");
    let amin = options.amin.unwrap_or(1).max(1);
    let done = n == 0 || amin > n || options.amax == Some(0) || options.nmax == Some(0);
    let mut a = if done { vec![0, 0] } else { vec![0; size + 1] };
    if !done {
        a[0] = amin - 1;
        a[1] = n - amin + 1;
    }
    PartitionsOf {
        a,
        k: 1,
        n,
        options,
        composition,
        empty: n == 0 && options.nmin.or(options.n).unwrap_or(1) <= 1,
        done,
    }
}

/// Iterate over ascending partitions. See `XS.xs`, `forpart`, `RuleAsc`.
/// # Panics
/// Panics if `n > u64::MAX - 2` or the state cannot fit in memory.
/// ```
/// use math_prime_util::{partitions_of, PartitionOptions};
/// assert_eq!(partitions_of(3, PartitionOptions::default()).collect::<Vec<_>>(), [vec![1,1,1],vec![1,2],vec![3]]);
/// ```
pub fn partitions_of(n: u64, options: PartitionOptions) -> PartitionsOf {
    parts(n, options, false)
}
/// Iterate over ordered compositions. See `XS.xs`, `forcomp`, `RuleAsc`.
/// # Panics
/// Panics if `n > u64::MAX - 2` or the state cannot fit in memory.
/// ```
/// assert_eq!(math_prime_util::compositions_of(3, Default::default()).count(), 4);
/// ```
pub fn compositions_of(n: u64, options: PartitionOptions) -> PartitionsOf {
    parts(n, options, true)
}

/// Combination state from `XS.xs`, `_comb_init` and `_comb_iterate`.
#[derive(Debug, Clone)]
pub struct CombinationsOf {
    n: usize,
    k: usize,
    end: usize,
    current: Option<Vec<usize>>,
}
impl Iterator for CombinationsOf {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.k > self.end || self.k > self.n {
            return None;
        }
        let out = self.current.take().unwrap_or_else(|| (0..self.k).collect());
        let mut next = out.clone();
        if let Some(i) = (0..self.k).rev().find(|&i| next[i] < self.n - self.k + i) {
            next[i] += 1;
            for j in i + 1..self.k {
                next[j] = next[j - 1] + 1;
            }
            self.current = Some(next);
        } else {
            self.k += 1;
        }
        Some(out)
    }
}
/// Iterate over combinations of indices. `None` visits all lengths.
/// See `XS.xs`, `forcomb` and `_comb_iterate`.
/// ```
/// assert_eq!(math_prime_util::combinations_of(3, Some(2)).collect::<Vec<_>>(), [vec![0,1],vec![0,2],vec![1,2]]);
/// ```
pub fn combinations_of(n: usize, k: Option<usize>) -> CombinationsOf {
    CombinationsOf {
        n,
        k: k.unwrap_or(0),
        end: k.unwrap_or(n),
        current: None,
    }
}

/// Visit each sum. See `XS.xs`, `forpart`.
/// # Panics
/// Panics for a size that cannot be represented or allocated.
/// ```
/// assert!(math_prime_util::forpart(3, Default::default(), |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forpart<B>(
    n: u64,
    options: PartitionOptions,
    f: impl FnMut(Vec<u64>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(partitions_of(n, options), f)
}

/// Visit each sum. See `XS.xs`, `forcomp`.
/// # Panics
/// Panics for a size that cannot be represented or allocated.
/// ```
/// assert!(math_prime_util::forcomp(3, Default::default(), |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forcomp<B>(
    n: u64,
    options: PartitionOptions,
    f: impl FnMut(Vec<u64>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(compositions_of(n, options), f)
}

/// Visit combinations. See `XS.xs`, `forcomb`.
/// ```
/// assert!(math_prime_util::forcomb(3, Some(2), |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forcomb<B>(
    n: usize,
    k: Option<usize>,
    f: impl FnMut(Vec<usize>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(combinations_of(n, k), f)
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

mod forperm;
pub use forperm::{PermutationsOf, forperm, permutations_of};

mod formultiperm;
pub use formultiperm::{MultipermutationsOf, formultiperm, multipermutations_of};

mod forderange;
pub use forderange::{DerangementsOf, derangements_of, forderange};

mod forsetproduct;
pub use forsetproduct::{SetProduct, forsetproduct, set_product};
