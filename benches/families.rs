#![allow(missing_docs)]
//! Benchmarks for Goldbach and Chen functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    goldbach_pair_count, goldbach_pairs, minimal_goldbach_pair, next_chen_prime,
};

fn bench_families(c: &mut Criterion) {
    c.bench_function("minimal_goldbach_pair", |b| {
        b.iter(|| minimal_goldbach_pair(black_box(60_119_912)));
    });
    c.bench_function("goldbach_pair_count", |b| {
        b.iter(|| goldbach_pair_count(black_box(1_000_000)));
    });
    c.bench_function("goldbach_pairs", |b| {
        b.iter(|| goldbach_pairs(black_box(100_000)));
    });
    c.bench_function("next_chen_prime", |b| {
        b.iter(|| next_chen_prime(black_box(10_000_000_000_000_000)));
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
