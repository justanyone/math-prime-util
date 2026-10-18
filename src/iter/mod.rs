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

mod forperm;
pub use forperm::{PermutationsOf, forperm, permutations_of};

mod formultiperm;
pub use formultiperm::{MultipermutationsOf, formultiperm, multipermutations_of};

mod forderange;
pub use forderange::{DerangementsOf, derangements_of, forderange};

mod forsetproduct;
pub use forsetproduct::{SetProduct, forsetproduct, set_product};
