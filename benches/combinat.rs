#![allow(missing_docs)]
//! Benchmarks for the assigned combinat functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    consecutive_integer_lcm, frobenius_number, is_congruent_number, is_polygonal,
};

fn bench_combinat(c: &mut Criterion) {
    c.bench_function("consecutive_integer_lcm", |b| {
        b.iter(|| consecutive_integer_lcm(black_box(43)));
    });
    c.bench_function("frobenius_number", |b| {
        b.iter(|| frobenius_number(black_box(&[47, 74, 97, 126, 157, 188])));
    });
    c.bench_function("is_congruent_number", |b| {
        b.iter(|| is_congruent_number(black_box(1001)));
    });
    c.bench_function("is_polygonal", |b| {
        b.iter(|| is_polygonal(black_box(15), black_box(3)));
    });
}
criterion_group!(benches, bench_combinat);
criterion_main!(benches);
