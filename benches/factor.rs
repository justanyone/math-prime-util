#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{fermat_factor, holf_factor, lehman_factor, squfof_factor};

fn bench_factor(c: &mut Criterion) {
    let n = 1_000_036_000_099;
    c.bench_function("fermat_factor", |b| {
        b.iter(|| fermat_factor(black_box(n), None));
    });
    c.bench_function("holf_factor", |b| {
        b.iter(|| holf_factor(black_box(n), None));
    });
    c.bench_function("lehman_factor", |b| {
        b.iter(|| lehman_factor(black_box(n), None));
    });
    c.bench_function("squfof_factor", |b| {
        b.iter(|| squfof_factor(black_box(n), None));
    });
}
criterion_group!(benches, bench_factor);
criterion_main!(benches);
