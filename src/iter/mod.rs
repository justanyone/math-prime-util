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

mod lastfor;
pub use lastfor::lastfor;
mod prime_iterator;
pub use prime_iterator::{PrimeIterator, prime_iterator};
mod prime_iterator_object;
pub use prime_iterator_object::prime_iterator_object;

mod forperm;
pub use forperm::{PermutationsOf, forperm, permutations_of};

mod formultiperm;
pub use formultiperm::{MultipermutationsOf, formultiperm, multipermutations_of};

mod forderange;
pub use forderange::{DerangementsOf, derangements_of, forderange};

mod forsetproduct;
pub use forsetproduct::{SetProduct, forsetproduct, set_product};
