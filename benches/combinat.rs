//! Benchmarks for the assigned combinat functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{falling_factorial, integer_complexity, powersum, rising_factorial};

fn bench_combinat(c: &mut Criterion) {
    c.bench_function("falling_factorial", |b| {
        b.iter(|| falling_factorial(black_box(509), 7))
    });
    c.bench_function("rising_factorial", |b| {
        b.iter(|| rising_factorial(black_box(509), 7))
    });
    c.bench_function("powersum", |b| b.iter(|| powersum(black_box(116), 8)));
    c.bench_function("integer_complexity_cached", |b| {
        b.iter(|| integer_complexity(black_box(100_000)))
    });
}

criterion_group!(benches, bench_combinat);
criterion_main!(benches);
