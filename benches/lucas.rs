//! Benchmarks for the Lucas ladders.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{fibonacci, lucas_number, lucas_sequence, lucasuv, lucasuvmod, lucasvmod};

fn bench_lucas(c: &mut Criterion) {
    c.bench_function("lucas_sequence", |b| {
        b.iter(|| lucas_sequence(black_box(u64::MAX), 1, -1, black_box(u64::MAX)));
    });
    c.bench_function("lucasuvmod_even", |b| {
        b.iter(|| lucasuvmod(7, 3, black_box(u64::MAX), u64::MAX - 1));
    });
    c.bench_function("lucasvmod", |b| {
        b.iter(|| lucasvmod(7, 1, black_box(u64::MAX), u64::MAX));
    });
    c.bench_function("lucasuv", |b| b.iter(|| lucasuv(1, -1, black_box(90))));
    c.bench_function("fibonacci", |b| b.iter(|| fibonacci(black_box(93))));
    c.bench_function("lucas_number", |b| b.iter(|| lucas_number(black_box(92))));
}
criterion_group!(benches, bench_lucas);
criterion_main!(benches);
