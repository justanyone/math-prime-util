#![allow(missing_docs)]
//! Benchmarks for the assigned MPU combinat functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{hclassno, is_happy, is_pillai, numtoperm};

fn bench_combinat(c: &mut Criterion) {
    c.bench_function("is_pillai", |b| b.iter(|| is_pillai(black_box(1009))));
    c.bench_function("is_happy", |b| {
        b.iter(|| is_happy(black_box(u64::MAX), 10, 2));
    });
    c.bench_function("hclassno", |b| b.iter(|| hclassno(black_box(1_000_003))));
    c.bench_function("numtoperm", |b| {
        b.iter(|| numtoperm(24, black_box(987_654_321)));
    });
}

criterion_group!(benches, bench_combinat);
criterion_main!(benches);
