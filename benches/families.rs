//! Benchmarks for twin estimates and semiprime operations.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    nth_twin_prime_approx, semi_primes, semiprime_count, semiprime_count_approx,
};

fn families(c: &mut Criterion) {
    c.bench_function("nth_twin_prime_approx", |b| {
        b.iter(|| nth_twin_prime_approx(black_box(100_000)))
    });
    c.bench_function("semi_primes", |b| {
        b.iter(|| semi_primes(black_box(1_000_000), black_box(1_001_000)))
    });
    c.bench_function("semiprime_count", |b| {
        b.iter(|| semiprime_count(black_box(1_000_000)))
    });
    c.bench_function("semiprime_count_approx", |b| {
        b.iter(|| semiprime_count_approx(black_box(1_000_000)))
    });
}
criterion_group!(benches, families);
criterion_main!(benches);
