#![allow(missing_docs)]
//! Benchmarks for the totient, Moebius, divisor-sum and residue-ring functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    divisor_sum, euler_phi, euler_phi_range, inverse_sigma0, inverse_sigma0_count, mertens,
    moebius, sumtotient, znorder, znprimroot,
};
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
    c.bench_function("arith/divisor_sum", |b| {
        b.iter(|| divisor_sum(black_box(360_360), 2));
    });
    c.bench_function("arith/znorder", |b| {
        b.iter(|| znorder(black_box(2), 1_000_003));
    });
    c.bench_function("arith/znprimroot", |b| {
        b.iter(|| znprimroot(black_box(1_000_003)));
    });
    c.bench_function("arith/inverse_sigma0", |b| {
        b.iter(|| inverse_sigma0(black_box(12), 1, 1_000_000));
    });
    c.bench_function("arith/inverse_sigma0_count", |b| {
        b.iter(|| inverse_sigma0_count(black_box(48), 1, 1_000_000));
    });
}
criterion_group!(benches, bench_arith);
criterion_main!(benches);
