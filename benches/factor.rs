#![allow(missing_docs)]
//! Benchmarks for the factor predicates.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    is_carmichael, is_cyclic, is_perfect_number, is_practical, is_quasi_carmichael, is_rough,
    is_smooth, smooth_count,
};

fn bench_factor(c: &mut Criterion) {
    c.bench_function("is_practical", |b| {
        b.iter(|| is_practical(black_box(720_720)));
    });
    c.bench_function("is_smooth", |b| {
        b.iter(|| is_smooth(black_box(614_889_782_588_491_410), 100));
    });
    c.bench_function("is_rough", |b| {
        b.iter(|| is_rough(black_box(1_000_003 * 1_000_033), 10_000));
    });
    c.bench_function("smooth_count", |b| {
        b.iter(|| smooth_count(black_box(10_000_000), 400));
    });
    c.bench_function("is_carmichael", |b| {
        b.iter(|| is_carmichael(black_box(3_215_031_751)));
    });
    c.bench_function("is_quasi_carmichael", |b| {
        b.iter(|| is_quasi_carmichael(black_box(255_255)));
    });
    c.bench_function("is_cyclic", |b| {
        b.iter(|| is_cyclic(black_box(1009 * 1013)));
    });
    c.bench_function("is_perfect_number", |b| {
        b.iter(|| is_perfect_number(black_box(2_305_843_008_139_952_128)));
    });
}

criterion_group!(benches, bench_factor);
criterion_main!(benches);
