//! Hot iterator workloads.
#![allow(missing_docs)] // Criterion generates a public benchmark entry point.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::iter::{
    PartitionOptions, combinations_of, composites_in, derangements_of, multipermutations_of,
    odd_composites_in, partitions_of, permutations_of, primes_in, semiprimes_in, set_product,
};

fn bench_iter(c: &mut Criterion) {
    c.bench_function("iter/almost_primes", |b| {
        b.iter(|| black_box(math_prime_util::almost_primes_in(3, 0..=black_box(10_000)).count()));
    });
    c.bench_function("iter/factored", |b| {
        b.iter(|| black_box(math_prime_util::factored_in(1..=black_box(10_000)).count()));
    });
    c.bench_function("iter/squarefree", |b| {
        b.iter(|| black_box(math_prime_util::squarefree_in(1..=black_box(10_000)).count()));
    });
    c.bench_function("iter/squarefree_int", |b| {
        b.iter(|| {
            black_box(math_prime_util::squarefree_integers_in(1..=black_box(10_000)).count())
        });
    });
    c.bench_function("iter/primes", |b| {
        b.iter(|| black_box(primes_in(0..=black_box(100_000)).count()));
    });
    c.bench_function("iter/composites", |b| {
        b.iter(|| black_box(composites_in(0..=black_box(100_000)).count()));
    });
    c.bench_function("iter/odd_composites", |b| {
        b.iter(|| black_box(odd_composites_in(0..=black_box(100_000)).count()));
    });
    c.bench_function("iter/semiprimes", |b| {
        b.iter(|| black_box(semiprimes_in(0..=black_box(100_000)).count()));
    });
    c.bench_function("iter/partitions", |b| {
        b.iter(|| black_box(partitions_of(black_box(30), PartitionOptions::default()).count()));
    });
    c.bench_function("iter/combinations", |b| {
        b.iter(|| black_box(combinations_of(black_box(20), Some(5)).count()));
    });
    c.bench_function("iter/permutations", |b| {
        b.iter(|| black_box(permutations_of(black_box(8)).count()));
    });
    c.bench_function("iter/derangements", |b| {
        b.iter(|| black_box(derangements_of(black_box(8)).count()));
    });
    c.bench_function("iter/multipermutations", |b| {
        b.iter(|| black_box(multipermutations_of(black_box(&[1, 1, 2, 2, 3, 3])).count()));
    });
    let sets = vec![vec![0, 1, 2, 3]; 5];
    c.bench_function("iter/set_product", |b| {
        b.iter(|| black_box(set_product(black_box(&sets)).count()));
    });
}
criterion_group!(benches, bench_iter);
criterion_main!(benches);
