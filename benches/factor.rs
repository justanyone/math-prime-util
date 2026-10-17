#![allow(missing_docs)]
//! Benchmarks for the u128 factor paths.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{factor_u128, is_prime_u128};

fn bench_factor(c: &mut Criterion) {
    c.bench_function("factor_u128", |b| {
        b.iter(|| factor_u128(black_box(2017 * 18_446_744_073_709_551_629u128)));
    });
    c.bench_function("is_prime_u128", |b| {
        b.iter(|| is_prime_u128(black_box((1u128 << 127) - 1)));
    });
}
criterion_group!(benches, bench_factor);
criterion_main!(benches);
