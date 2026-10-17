#![allow(missing_docs)]
//! Benchmarks for the factor predicates.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{is_carmichael, is_cyclic, is_perfect_number, is_quasi_carmichael};

fn bench_factor(c: &mut Criterion) {
    c.bench_function("is_carmichael", |b| {
        b.iter(|| is_carmichael(black_box(321_503_1751)))
    });
    c.bench_function("is_quasi_carmichael", |b| {
        b.iter(|| is_quasi_carmichael(black_box(255_255)))
    });
    c.bench_function("is_cyclic", |b| {
        b.iter(|| is_cyclic(black_box(1009 * 1013)))
    });
    c.bench_function("is_perfect_number", |b| {
        b.iter(|| is_perfect_number(black_box(2_305_843_008_139_952_128)))
    });
}
criterion_group!(benches, bench_factor);
criterion_main!(benches);
