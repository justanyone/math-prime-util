//! Benchmarks for the real special functions.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::*;
fn bench_real(c: &mut Criterion) {
    c.bench_function("real/ei", |b| {
        b.iter(|| exponential_integral(black_box(100.0)));
    });
    c.bench_function("real/li", |b| {
        b.iter(|| logarithmic_integral(black_box(1e12)));
    });
    c.bench_function("real/zeta", |b| b.iter(|| riemann_zeta(black_box(3.5))));
    c.bench_function("real/riemann_r", |b| b.iter(|| riemann_r(black_box(1e15))));
    c.bench_function("real/lambert_w", |b| b.iter(|| lambert_w(black_box(1e10))));
    c.bench_function("real/inverse_li", |b| {
        b.iter(|| inverse_li(black_box(1_000_000)));
    });
    c.bench_function("real/pi", |b| b.iter(|| pi(black_box(100))));
}
criterion_group!(benches, bench_real);
criterion_main!(benches);
