<<<<<<< HEAD
#![allow(missing_docs)]
//! Benchmarks for the totient and Moebius functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{euler_phi, euler_phi_range, mertens, moebius, sumtotient};
fn bench_arith(c: &mut Criterion) {
    c.bench_function("euler_phi", |b| {
        b.iter(|| euler_phi(black_box(1_000_000_007)));
    });
    c.bench_function("moebius", |b| b.iter(|| moebius(black_box(1_000_000_007))));
    c.bench_function("totient_range", |b| {
        b.iter(|| euler_phi_range(0, black_box(10_000)));
    });
    c.bench_function("mertens", |b| b.iter(|| mertens(black_box(100_000))));
    c.bench_function("sumtotient", |b| b.iter(|| sumtotient(black_box(100_000))));
}
criterion_group!(benches, bench_arith);
=======
//! Arithmetic benchmarks.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{divisor_sum, znorder, znprimroot};
fn arithmetic(c: &mut Criterion) {
    c.bench_function("arith/divisor_sum", |b| {
        b.iter(|| divisor_sum(black_box(360_360), 2));
    });
    c.bench_function("arith/znorder", |b| {
        b.iter(|| znorder(black_box(2), 1_000_003));
    });
    c.bench_function("arith/znprimroot", |b| {
        b.iter(|| znprimroot(black_box(1_000_003)));
    });
}
criterion_group!(benches, arithmetic);
>>>>>>> origin/dev-001
criterion_main!(benches);
