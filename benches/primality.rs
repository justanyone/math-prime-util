#![allow(missing_docs)]
//! Benchmarks for the native primality tests.
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::*;

fn bench_primality(c: &mut Criterion) {
    let mut group = c.benchmark_group("primality");
    for n in [1_000_003, 4_294_967_311, (1 << 61) - 1, u64::MAX - 58] {
        group.bench_with_input(BenchmarkId::new("is_prime", n), &n, |b, &n| {
            b.iter(|| is_prime(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("bpsw", n), &n, |b, &n| {
            b.iter(|| is_bpsw_prime(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("underwood", n), &n, |b, &n| {
            b.iter(|| is_frobenius_underwood_pseudoprime(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("khashin", n), &n, |b, &n| {
            b.iter(|| is_frobenius_khashin_pseudoprime(black_box(n)));
        });
    }
    group.finish();
}

fn proving(c: &mut Criterion) {
    c.bench_function("aks", |b| b.iter(|| is_aks_prime(black_box(1_000_003))));
    c.bench_function("provable", |b| {
        b.iter(|| is_provable_prime(black_box(u64::MAX - 58)));
    });
    c.bench_function("certificate", |b| {
        b.iter(|| prime_certificate(black_box(u64::MAX - 58)));
    });
    let cert = prime_certificate(u64::MAX - 58);
    c.bench_function("verify", |b| b.iter(|| verify_prime(black_box(&cert))));
}

criterion_group!(benches, bench_primality, proving);
criterion_main!(benches);
