//! Benchmarks for the smooth, rough, and practical predicates.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{is_practical, is_rough, is_smooth, smooth_count};

fn bench_factor(c: &mut Criterion) {
    c.bench_function("is_practical", |b| {
        b.iter(|| is_practical(black_box(720_720)))
    });
    c.bench_function("is_smooth", |b| {
        b.iter(|| is_smooth(black_box(614_889_782_588_491_410), 100))
    });
    c.bench_function("is_rough", |b| {
        b.iter(|| is_rough(black_box(1_000_003 * 1_000_033), 10_000))
    });
    c.bench_function("smooth_count", |b| {
        b.iter(|| smooth_count(black_box(10_000_000), 400))
    });
}

criterion_group!(benches, bench_factor);
criterion_main!(benches);
