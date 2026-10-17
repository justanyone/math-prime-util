#![allow(missing_docs)]
//! Benchmarks for the twin prime functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{nth_twin_prime, twin_prime_count, twin_prime_count_approx, twin_primes};

fn bench_families(c: &mut Criterion) {
    c.bench_function("twin_primes", |b| {
        b.iter(|| twin_primes(0, black_box(100_000)))
    });
    c.bench_function("twin_prime_count", |b| {
        b.iter(|| twin_prime_count(black_box(100_000)))
    });
    c.bench_function("twin_prime_count_approx", |b| {
        b.iter(|| twin_prime_count_approx(black_box(1_000_000_000)))
    });
    c.bench_function("nth_twin_prime", |b| {
        b.iter(|| nth_twin_prime(black_box(1000)))
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
