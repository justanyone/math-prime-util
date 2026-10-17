#![allow(missing_docs)]
//! Benchmarks for the factor functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    cheb_factor, is_carmichael, is_cyclic, is_perfect_number, is_quasi_carmichael, prime_bigomega,
    prime_omega, prime_signature,
};

fn bench_factor(c: &mut Criterion) {
    c.bench_function("cheb_factor", |b| {
        b.iter(|| cheb_factor(black_box(1_000_036_000_099), 0, 0));
    });
    c.bench_function("prime_bigomega", |b| {
        b.iter(|| prime_bigomega(black_box(600_851_475_143)));
    });
    c.bench_function("prime_omega", |b| {
        b.iter(|| prime_omega(black_box(600_851_475_143)));
    });
    c.bench_function("prime_signature", |b| {
        b.iter(|| prime_signature(black_box(600_851_475_143)));
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
}

criterion_group!(benches, bench_factor);
criterion_main!(benches);
