//! Benchmarks for native factorial products.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    factorial, falling_factorial, multifactorial, rising_factorial, subfactorial,
};

fn bench_combinat(c: &mut Criterion) {
    c.bench_function("factorial", |b| b.iter(|| factorial(black_box(20))));
    c.bench_function("multifactorial", |b| {
        b.iter(|| multifactorial(black_box(30), 2))
    });
    c.bench_function("subfactorial", |b| b.iter(|| subfactorial(black_box(20))));
    c.bench_function("falling_factorial", |b| {
        b.iter(|| falling_factorial(black_box(100), 8))
    });
    c.bench_function("rising_factorial", |b| {
        b.iter(|| rising_factorial(black_box(100), 8))
    });
}

criterion_group!(benches, bench_combinat);
criterion_main!(benches);
