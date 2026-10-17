#![allow(missing_docs)]
//! Benchmarks for the factor predicates.
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    is_almost_prime, is_carmichael, is_cyclic, is_omega_prime, is_perfect_number,
    is_quasi_carmichael, is_semiprime, is_square_free,
};

fn bench_factor(c: &mut Criterion) {
    let mut group = c.benchmark_group("factor");
    for n in [30_u64, 1_000_003 * 1_000_033, u64::MAX] {
        group.bench_with_input(BenchmarkId::new("is_square_free", n), &n, |b, &n| {
            b.iter(|| is_square_free(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("is_semiprime", n), &n, |b, &n| {
            b.iter(|| is_semiprime(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("is_almost_prime", n), &n, |b, &n| {
            b.iter(|| is_almost_prime(black_box(3), black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("is_omega_prime", n), &n, |b, &n| {
            b.iter(|| is_omega_prime(black_box(3), black_box(n)));
        });
    }
    group.finish();
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
