#![allow(missing_docs)]
//! Benchmarks for exact and modular Lucas sequences.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{fibonacci, lucas_sequence, lucasuv, lucasuvmod};

fn bench_lucas(c: &mut Criterion) {
    c.bench_function("fibonacci", |b| b.iter(|| fibonacci(black_box(93))));
    c.bench_function("lucasuv", |b| b.iter(|| lucasuv(1, -1, black_box(90))));
    c.bench_function("lucasuvmod", |b| {
        b.iter(|| lucasuvmod(1, -1, black_box(u64::MAX), u64::MAX - 58));
    });
    c.bench_function("lucas_sequence", |b| {
        b.iter(|| lucas_sequence(u64::MAX - 58, 1, -1, black_box(u64::MAX)));
    });
}
criterion_group!(benches, bench_lucas);
criterion_main!(benches);
