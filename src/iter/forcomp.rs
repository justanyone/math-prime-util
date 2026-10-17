use super::forpart::parts;
use super::{PartitionOptions, PartitionsOf, run};
use std::ops::ControlFlow;

/// State for ordered compositions. See `XS.xs::forpart`, alias `forcomp`.
#[derive(Debug, Clone)]
pub struct CompositionsOf {
    inner: PartitionsOf,
}

impl Iterator for CompositionsOf {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Iterate over ordered compositions. See `XS.xs`, `forcomp`, `RuleAsc`.
/// # Panics
/// Panics if `n > u64::MAX - 2` or the state cannot fit in memory.
/// ```
/// assert_eq!(math_prime_util::compositions_of(3, Default::default()).count(), 4);
/// ```
pub fn compositions_of(n: u64, options: PartitionOptions) -> CompositionsOf {
    CompositionsOf {
        inner: parts(n, options, true),
    }
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
