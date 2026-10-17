//! Hot iterator workloads.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::iter::{combinations_of, partitions_of, permutations_of, primes_in};

fn bench_iter(c: &mut Criterion) {
    c.bench_function("iter/primes", |b| {
        b.iter(|| black_box(primes_in(0..=black_box(100_000)).count()));
    });
    c.bench_function("iter/partitions", |b| {
        b.iter(|| black_box(partitions_of(black_box(30), Default::default()).count()));
    });
    c.bench_function("iter/combinations", |b| {
        b.iter(|| black_box(combinations_of(black_box(20), Some(5)).count()));
    });
    c.bench_function("iter/permutations", |b| {
        b.iter(|| black_box(permutations_of(black_box(8)).count()));
    });
}
criterion_group!(benches, bench_iter);
criterion_main!(benches);
