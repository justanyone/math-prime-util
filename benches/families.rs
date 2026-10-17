#![allow(missing_docs)]
//! Benchmarks for Ramanujan prime generation and counts.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    ramanujan_prime_count, ramanujan_prime_count_approx, ramanujan_prime_count_lower,
    ramanujan_primes,
};
fn bench_families(c: &mut Criterion) {
    c.bench_function("ramanujan_primes", |b| {
        b.iter(|| ramanujan_primes(black_box(10_000), black_box(20_000)));
    });
    c.bench_function("ramanujan_prime_count", |b| {
        b.iter(|| ramanujan_prime_count(black_box(100_000)));
    });
    c.bench_function("ramanujan_prime_count_approx", |b| {
        b.iter(|| ramanujan_prime_count_approx(black_box(100_000_000)));
    });
    c.bench_function("ramanujan_prime_count_lower", |b| {
        b.iter(|| ramanujan_prime_count_lower(black_box(100_000_000)));
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
