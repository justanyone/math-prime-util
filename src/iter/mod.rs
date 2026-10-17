//! Stateful loops from MPU 0.75 `XS.xs` and `lib/Math/Prime/Util/PP.pm`.

use std::ops::{ControlFlow, RangeInclusive};

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

/// Lexical permutation state from `XS.xs`, `_comb_iterate`.
#[derive(Debug, Clone)]
pub struct PermutationsOf<T> {
    current: Option<Vec<T>>,
}
fn advance<T: Ord>(a: &mut [T]) -> bool {
    let Some(i) = (1..a.len()).rev().find(|&i| a[i - 1] < a[i]) else {
        return false;
    };
    let j = (i..a.len())
        .rev()
        .find(|&j| a[i - 1] < a[j])
        .expect("greater suffix element");
    a.swap(i - 1, j);
    a[i..].reverse();
    true
}
impl<T: Ord + Clone> Iterator for PermutationsOf<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.current.take()?;
        let mut next = out.clone();
        if advance(&mut next) {
            self.current = Some(next);
        }
        Some(out)
    }
}
/// Iterate over permutations of indices. See `XS.xs`, `forperm`.
/// ```
/// assert_eq!(math_prime_util::permutations_of(3).count(), 6);
/// ```
pub fn permutations_of(n: usize) -> PermutationsOf<usize> {
    PermutationsOf {
        current: Some((0..n).collect()),
    }
}
/// Iterate over distinct multiset permutations in lexical order.
/// See `lib/Math/Prime/Util/PP.pm`, `formultiperm` and `_multiset_permutations`.
/// ```
/// assert_eq!(math_prime_util::multipermutations_of(&[1,1,2]).count(), 3);
/// ```
pub fn multipermutations_of<T: Ord + Clone>(values: &[T]) -> MultipermutationsOf<T> {
    let mut sorted = values.to_vec();
    sorted.sort();
    let mut counts: Vec<(T, usize)> = Vec::new();
    for value in sorted {
        if let Some(last) = counts.last_mut().filter(|last| last.0 == value) {
            last.1 += 1;
        } else {
            counts.push((value, 1));
        }
    }
    MultipermutationsOf {
        counts,
        prefix: Vec::new(),
        frames: vec![0],
        size: values.len(),
        done: false,
    }
}

/// Explicit recursion stack for `PP.pm`, `_multiset_permutations`.
#[derive(Debug, Clone)]
pub struct MultipermutationsOf<T> {
    counts: Vec<(T, usize)>,
    prefix: Vec<usize>,
    frames: Vec<usize>,
    size: usize,
    done: bool,
}
impl<T: Clone> Iterator for MultipermutationsOf<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.done {
            if self.prefix.len() == self.size {
                let out = self
                    .prefix
                    .iter()
                    .map(|&i| self.counts[i].0.clone())
                    .collect();
                self.frames.pop();
                if let Some(i) = self.prefix.pop() {
                    self.counts[i].1 += 1;
                } else {
                    self.done = true;
                }
                return Some(out);
            }
            let depth = self.prefix.len();
            let start = self.frames[depth];
            if let Some(i) = (start..self.counts.len()).find(|&i| self.counts[i].1 > 0) {
                self.frames[depth] = i + 1;
                self.counts[i].1 -= 1;
                self.prefix.push(i);
                self.frames.push(0);
            } else {
                self.frames.pop();
                if let Some(i) = self.prefix.pop() {
                    self.counts[i].1 += 1;
                } else {
                    self.done = true;
                }
            }
        }
        None
    }
}

/// Derangement state. See `XS.xs`, `_comb_init` and `_comb_iterate`.
#[derive(Debug, Clone)]
pub struct DerangementsOf {
    current: Option<Vec<usize>>,
}
impl Iterator for DerangementsOf {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut cm = self.current.take()?;
        let n = cm.len();
        let out = cm.iter().rev().map(|v| v - 1).collect();
        // Keep the C array reversed. This also preserves its early skip.
        while let Some(j) = (1..n).find(|&j| cm[j] <= cm[j - 1]) {
            let m = (0..n)
                .find(|&m| cm[j] <= cm[m])
                .expect("greater suffix element");
            cm.swap(j, m);
            if cm[j] == n - j {
                continue;
            }
            cm[..j].reverse();
            if cm.iter().rev().enumerate().all(|(i, &v)| v - 1 != i) {
                self.current = Some(cm);
                break;
            }
        }
        Some(out)
    }
}
/// Iterate over permutations with no fixed points. See `XS.xs`, `forderange`.
/// ```
/// assert_eq!(math_prime_util::derangements_of(4).count(), 9);
/// ```
pub fn derangements_of(n: usize) -> DerangementsOf {
    let current = if n == 1 {
        None
    } else {
        let mut cm = vec![0; n];
        for i in 0..n {
            cm[n - i - 1] = if i % 2 == 1 { i } else { i + 2 };
        }
        if n % 2 == 1 {
            cm[0] = n - 2;
            cm[1] = n;
        }
        Some(cm)
    };
    DerangementsOf { current }
}

/// Cartesian product state from `XS.xs`, `forsetproduct`.
#[derive(Debug, Clone)]
pub struct SetProduct<T> {
    sets: Vec<Vec<T>>,
    indices: Vec<usize>,
    done: bool,
}
impl<T: Clone> Iterator for SetProduct<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let out = self
            .sets
            .iter()
            .zip(&self.indices)
            .map(|(s, &i)| s[i].clone())
            .collect();
        self.done = true;
        for i in (0..self.indices.len()).rev() {
            self.indices[i] += 1;
            if self.indices[i] < self.sets[i].len() {
                self.done = false;
                break;
            }
            self.indices[i] = 0;
        }
        Some(out)
    }
}
/// Iterate over a snapshot of the sets. See `XS.xs`, `forsetproduct`.
/// ```
/// assert_eq!(math_prime_util::set_product(&[vec![1,2],vec![3]]).collect::<Vec<_>>(), [vec![1,3],vec![2,3]]);
/// ```
pub fn set_product<T: Clone>(sets: &[Vec<T>]) -> SetProduct<T> {
    SetProduct {
        sets: sets.to_vec(),
        indices: vec![0; sets.len()],
        done: sets.is_empty() || sets.iter().any(Vec::is_empty),
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

/// Visit each arrangement. See `XS.xs`, `forperm`.
/// ```
/// assert!(math_prime_util::forperm(3, |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forperm<B>(n: usize, f: impl FnMut(Vec<usize>) -> ControlFlow<B>) -> ControlFlow<B> {
    run(permutations_of(n), f)
}

/// Visit each arrangement. See `XS.xs`, `forderange`.
/// ```
/// assert!(math_prime_util::forderange(3, |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forderange<B>(n: usize, f: impl FnMut(Vec<usize>) -> ControlFlow<B>) -> ControlFlow<B> {
    run(derangements_of(n), f)
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
/// Visit distinct multiset permutations. See `lib/Math/Prime/Util/PP.pm`, `formultiperm`.
/// ```
/// assert!(math_prime_util::formultiperm(&[1,1,2], |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn formultiperm<T: Ord + Clone, B>(
    values: &[T],
    f: impl FnMut(Vec<T>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(multipermutations_of(values), f)
}
/// Visit a Cartesian product. See `XS.xs`, `forsetproduct`.
/// ```
/// assert!(math_prime_util::forsetproduct(&[vec![1,2]], |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forsetproduct<T: Clone, B>(
    sets: &[Vec<T>],
    f: impl FnMut(Vec<T>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(set_product(sets), f)
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
