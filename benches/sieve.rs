//! Hot sieve operations.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{next_prime, primes, sieve_prime_cluster, sieve_range, sum_primes};
fn bench_sieve(c: &mut Criterion) {
    c.bench_function("primes", |b| b.iter(|| primes(0, black_box(1_000_000))));
    c.bench_function("next_prime", |b| {
        b.iter(|| next_prime(black_box(1_u64 << 63)));
    });
    c.bench_function("sieve_range", |b| {
        b.iter(|| sieve_range(black_box(1_000_000), 10000, 97));
    });
    c.bench_function("cluster", |b| {
        b.iter(|| sieve_prime_cluster(0, black_box(100_000), &[2, 6]));
    });
    c.bench_function("sum_primes", |b| {
        b.iter(|| sum_primes(0, black_box(1_000_000)));
    });
}
criterion_group!(benches, bench_sieve);
criterion_main!(benches);
