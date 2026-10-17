#![allow(missing_docs)]
//! Benchmarks for native random sampling.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::*;

fn bench_random(c: &mut Criterion) {
    srand(42);
    c.bench_function("random_unrestricted_semiprime/32", |b| {
        b.iter(|| random_unrestricted_semiprime(black_box(32)));
    });
    c.bench_function("random_factored_integer/32", |b| {
        b.iter(|| random_factored_integer(black_box(1 << 32)));
    });
    c.bench_function("randperm/sparse", |b| {
        b.iter(|| randperm(black_box(1_000_000), black_box(20)));
    });
    let mut items: Vec<_> = (0..1000).collect();
    c.bench_function("shuffle/1000", |b| {
        b.iter(|| shuffle(black_box(&mut items)));
    });
}
criterion_group!(benches, bench_random);
criterion_main!(benches);
