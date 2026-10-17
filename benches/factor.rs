#![allow(missing_docs)]
//! Benchmarks for the bounded factor methods.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{pbrent_factor, pminus1_factor, pplus1_factor, prho_factor};

fn bench_factor(c: &mut Criterion) {
    let n = 1_000_036_000_099;
    c.bench_function("prho_factor", |b| {
        b.iter(|| prho_factor(black_box(n), None));
    });
    c.bench_function("pbrent_factor", |b| {
        b.iter(|| pbrent_factor(black_box(n), None, None));
    });
    c.bench_function("pminus1_factor", |b| {
        b.iter(|| pminus1_factor(black_box(n), None, None));
    });
    c.bench_function("pplus1_factor", |b| {
        b.iter(|| pplus1_factor(black_box(n), None));
    });
}
criterion_group!(benches, bench_factor);
criterion_main!(benches);
