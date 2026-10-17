#![allow(missing_docs)]
//! Benchmarks for the combinatorics group.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::*;

fn bench_combinat(c: &mut Criterion) {
    let mut g = c.benchmark_group("combinat");
    g.bench_function("binomial_66_33", |b| {
        b.iter(|| binomial(black_box(66), black_box(33)));
    });
    g.bench_function("factorial_20", |b| {
        b.iter(|| factorial(black_box(20)));
    });
    g.bench_function("bell_25", |b| {
        b.iter(|| bell_number(black_box(25)));
    });
    g.bench_function("partitions_200", |b| {
        b.iter(|| partitions(black_box(200)));
    });
    g.bench_function("primorial_40", |b| {
        b.iter(|| primorial(black_box(40)));
    });
    g.bench_function("is_pillai_10007", |b| {
        b.iter(|| is_pillai(black_box(10_007)));
    });
    g.bench_function("integer_complexity_1000", |b| {
        b.iter(|| integer_complexity(black_box(1_000)));
    });
    g.finish();
}

criterion_group!(benches, bench_combinat);
criterion_main!(benches);
