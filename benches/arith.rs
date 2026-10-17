//! Arithmetic benchmarks.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{divisor_sum, znorder, znprimroot};
fn arithmetic(c: &mut Criterion) {
    c.bench_function("arith/divisor_sum", |b| {
        b.iter(|| divisor_sum(black_box(360_360), 2))
    });
    c.bench_function("arith/znorder", |b| {
        b.iter(|| znorder(black_box(2), 1_000_003))
    });
    c.bench_function("arith/znprimroot", |b| {
        b.iter(|| znprimroot(black_box(1_000_003)))
    });
}
criterion_group!(benches, arithmetic);
criterion_main!(benches);
