//! Benchmarks for almost-prime and omega-prime families.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    almost_prime_count, almost_primes, nth_almost_prime, nth_omega_prime, omega_prime_count,
    omega_primes,
};

fn bench_families(c: &mut Criterion) {
    c.bench_function("almost_prime_count", |b| {
        b.iter(|| almost_prime_count(3, black_box(100_000)));
    });
    c.bench_function("omega_prime_count", |b| {
        b.iter(|| omega_prime_count(3, black_box(100_000)));
    });
    c.bench_function("almost_primes", |b| {
        b.iter(|| almost_primes(3, 0, black_box(100_000)));
    });
    c.bench_function("omega_primes", |b| {
        b.iter(|| omega_primes(3, 0, black_box(100_000)));
    });
    c.bench_function("nth_almost_prime", |b| {
        b.iter(|| nth_almost_prime(3, black_box(1000)));
    });
    c.bench_function("nth_omega_prime", |b| {
        b.iter(|| nth_omega_prime(3, black_box(1000)));
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
