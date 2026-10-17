#![allow(missing_docs)]
//! Benchmarks for the factorial family and ordered set partitions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{factorial, fubini, multifactorial, subfactorial};

fn bench_combinat(c: &mut Criterion) {
    c.bench_function("factorial/20", |b| b.iter(|| factorial(black_box(20))));
    c.bench_function("subfactorial/20", |b| {
        b.iter(|| subfactorial(black_box(20)))
    });
    c.bench_function("multifactorial/33/2", |b| {
        b.iter(|| multifactorial(black_box(33), black_box(2)));
    });
    c.bench_function("fubini/15", |b| b.iter(|| fubini(black_box(15))));
    c.bench_function("fubini/18", |b| b.iter(|| fubini(black_box(18))));
}

criterion_group!(benches, bench_combinat);
criterion_main!(benches);
