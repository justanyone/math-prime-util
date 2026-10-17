#![allow(missing_docs)]
//! Benchmarks for the factor family.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    fermat_factor, holf_factor, is_carmichael, is_cyclic, is_perfect_number, is_quasi_carmichael,
    lehman_factor, pbrent_factor, pminus1_factor, pplus1_factor, prho_factor, squfof_factor,
};

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
    c.bench_function("is_carmichael", |b| {
        b.iter(|| is_carmichael(black_box(3_215_031_751)));
    });
    c.bench_function("is_quasi_carmichael", |b| {
        b.iter(|| is_quasi_carmichael(black_box(255_255)));
    });
    c.bench_function("is_cyclic", |b| {
        b.iter(|| is_cyclic(black_box(1009 * 1013)));
    });
    c.bench_function("is_perfect_number", |b| {
        b.iter(|| is_perfect_number(black_box(2_305_843_008_139_952_128)));
    });
    {
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
}
criterion_group!(benches, bench_factor);
criterion_main!(benches);
