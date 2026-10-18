#![allow(missing_docs)]
//! Benchmarks for the native primality tests.
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    Int, is_aks_prime, is_bpsw_prime, is_frobenius_khashin_pseudoprime,
    is_frobenius_underwood_pseudoprime, is_prime, is_provable_prime, is_provable_prime_with_cert,
    prime_certificate, verify_prime,
};

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
    c.bench_function("provable_with_certificate", |b| {
        b.iter(|| is_provable_prime_with_cert(black_box(u64::MAX - 58)));
    });
    let cert = prime_certificate(u64::MAX - 58);
    c.bench_function("verify", |b| b.iter(|| verify_prime(black_box(&cert))));
}

criterion_group!(benches, bench_primality, proving, bench_generic);
criterion_main!(benches);

fn bench_generic(c: &mut Criterion) {
    c.bench_function("generic/is_prime", |b| {
        b.iter(|| black_box((1_u128 << 127) - 1).is_prime());
    });
    c.bench_function("generic/factor", |b| {
        b.iter(|| black_box(600_851_475_143_u128).factor());
    });
    c.bench_function("generic/powmod", |b| {
        b.iter(|| black_box(u128::MAX - 2).powmod(&127, &u128::MAX));
    });
    c.bench_function("generic/gcd", |b| {
        b.iter(|| black_box(u128::MAX).gcd(&600_851_475_143));
    });
    c.bench_function("generic/next_prime", |b| {
        b.iter(|| black_box(1_u128 << 64).next_prime());
    });
}
