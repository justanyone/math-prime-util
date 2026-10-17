#![allow(missing_docs)]
//! Benchmarks for the assigned MPU combinat functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    bernfrac, harmfrac, hclassno, is_happy, is_pillai, numtoperm, pn_primorial, primorial,
};

fn bench_combinat(c: &mut Criterion) {
    c.bench_function("is_pillai", |b| b.iter(|| is_pillai(black_box(1009))));
    c.bench_function("is_happy", |b| {
        b.iter(|| is_happy(black_box(u64::MAX), 10, 2));
    });
    c.bench_function("hclassno", |b| b.iter(|| hclassno(black_box(1_000_003))));
    c.bench_function("numtoperm", |b| {
        b.iter(|| numtoperm(24, black_box(987_654_321)));
    });
    c.bench_function("bernfrac_58", |b| b.iter(|| bernfrac(black_box(58))));
    c.bench_function("harmfrac_60", |b| b.iter(|| harmfrac(black_box(60))));
    c.bench_function("primorial_52", |b| b.iter(|| primorial(black_box(52))));
    c.bench_function("pn_primorial_15", |b| {
        b.iter(|| pn_primorial(black_box(15)));
    });
}

criterion_group!(benches, bench_combinat);
criterion_main!(benches);
