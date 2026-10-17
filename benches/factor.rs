#![allow(missing_docs)]
//! Benchmarks for the factor functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{cheb_factor, prime_bigomega, prime_omega, prime_signature};

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
}

criterion_group!(benches, bench_factor);
criterion_main!(benches);
