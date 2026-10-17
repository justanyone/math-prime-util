#![allow(missing_docs)]
//! Benchmarks for the assigned combinat functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    bernfrac, falling_factorial, harmfrac, integer_complexity, pn_primorial, powersum, primorial,
    rising_factorial,
};

fn bench_combinat(c: &mut Criterion) {
    c.bench_function("falling_factorial", |b| {
        b.iter(|| falling_factorial(black_box(509), 7));
    });
    c.bench_function("rising_factorial", |b| {
        b.iter(|| rising_factorial(black_box(509), 7));
    });
    c.bench_function("powersum", |b| b.iter(|| powersum(black_box(116), 8)));
    c.bench_function("integer_complexity_cached", |b| {
        b.iter(|| integer_complexity(black_box(100_000)));
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
