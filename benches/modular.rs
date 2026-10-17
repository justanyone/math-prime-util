//! Benchmarks for the factorial, binomial, logarithm, and period ports.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{binomialmod, factorialmod, pisano_period, znlog};
fn modular(c: &mut Criterion) {
    c.bench_function("sqrtmod", |b| {
        b.iter(|| math_prime_util::sqrtmod(black_box(58), 101))
    });
    c.bench_function("allsqrtmod", |b| {
        b.iter(|| math_prime_util::allsqrtmod(black_box(1), 360))
    });
    c.bench_function("rootmod", |b| {
        b.iter(|| math_prime_util::rootmod(black_box(13), 6, 107))
    });
    c.bench_function("allrootmod", |b| {
        b.iter(|| math_prime_util::allrootmod(black_box(249), 50, 625))
    });
    c.bench_function("factorialmod", |b| {
        b.iter(|| factorialmod(black_box(10000), 1_000_003));
    });
    c.bench_function("binomialmod", |b| {
        b.iter(|| binomialmod(black_box(100_000), 50_000, 1000));
    });
    c.bench_function("znlog", |b| b.iter(|| znlog(black_box(5), 2, 1019)));
    c.bench_function("pisano_period", |b| {
        b.iter(|| pisano_period(black_box(1_000_000_007)));
    });
}
criterion_group!(benches, modular);
criterion_main!(benches);
