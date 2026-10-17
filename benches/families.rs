//! Benchmarks for prime power counts.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    prime_power_count, prime_power_count_approx, prime_power_count_lower, prime_power_count_upper,
};

fn bench_families(c: &mut Criterion) {
    c.bench_function("prime_power_count", |b| {
        b.iter(|| prime_power_count(black_box(1_000_000)))
    });
    c.bench_function("prime_power_count_approx", |b| {
        b.iter(|| prime_power_count_approx(black_box(100_000_000)))
    });
    c.bench_function("prime_power_count_lower", |b| {
        b.iter(|| prime_power_count_lower(black_box(100_000_000)))
    });
    c.bench_function("prime_power_count_upper", |b| {
        b.iter(|| prime_power_count_upper(black_box(100_000_000)))
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
