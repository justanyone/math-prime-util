use super::run;
use std::ops::ControlFlow;

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
        let amin = self.options.amin.unwrap_or(1);
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
            if (out.len() as u64) < nmin {
                // XS.xs::forpart skips the remaining lexical range here.
                if self.a[0] >= self.n - nmin + 1 && self.a[self.k] > 1 {
                    self.done = true;
                    return None;
                }
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
pub(super) fn parts(n: u64, options: PartitionOptions, composition: bool) -> PartitionsOf {
    assert!(n <= u64::MAX - 2, "partition argument overflow");
    let mut options = options;
    let mut amin = options.amin.unwrap_or(1).max(1);
    let mut amax = options.amax.unwrap_or(n).min(n);
    let nmin = options.nmin.or(options.n).unwrap_or(1).max(1);
    let nmax = options.nmax.or(options.n).unwrap_or(n).min(n);
    if options.prime == Some(true) {
        let prev = crate::prev_prime(amax + 1);
        let next = if amin <= 2 {
            2
        } else {
            crate::next_prime(amin - 1)
        };
        amin = amin.max(next);
        amax = amax.min(prev);
    }
    options.amin = Some(amin);
    options.amax = Some(amax);
    options.nmin = Some(nmin);
    options.nmax = Some(nmax);
    let done = n < nmin || nmin > nmax || amin > amax || nmax == 0 || amax == 0;
    let size = if done {
        1
    } else {
        usize::try_from(n).expect("partition size must fit usize")
    };
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
        empty: n == 0 && nmin <= 1,
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
