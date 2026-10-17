//! Proving benchmarks.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{is_aks_prime, is_provable_prime, prime_certificate, verify_prime};
fn proving(c: &mut Criterion) {
    c.bench_function("aks", |b| b.iter(|| is_aks_prime(black_box(1_000_003))));
    c.bench_function("provable", |b| b.iter(|| is_provable_prime(black_box(u64::MAX-58))));
    c.bench_function("certificate", |b| b.iter(|| prime_certificate(black_box(u64::MAX-58))));
    let cert = prime_certificate(u64::MAX-58);
    c.bench_function("verify", |b| b.iter(|| verify_prime(black_box(&cert))));
}
criterion_group!(benches, proving);
criterion_main!(benches);
