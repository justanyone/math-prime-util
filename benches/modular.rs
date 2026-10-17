//! Benchmarks for the factorial, binomial, logarithm, and period ports.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{binomialmod, factorialmod, pisano_period, znlog};
fn modular(c: &mut Criterion) {
    c.bench_function("factorialmod", |b| {
        b.iter(|| factorialmod(black_box(10000), 1_000_003))
    });
    c.bench_function("binomialmod", |b| {
        b.iter(|| binomialmod(black_box(100000), 50000, 1000))
    });
    c.bench_function("znlog", |b| b.iter(|| znlog(black_box(5), 2, 1019)));
    c.bench_function("pisano_period", |b| {
        b.iter(|| pisano_period(black_box(1_000_000_007)))
    });
}
criterion_group!(benches, modular);
criterion_main!(benches);
