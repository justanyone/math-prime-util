#![allow(missing_docs)]
//! Benchmarks for the lucky-number sieve, rank, and membership paths.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    is_lucky, lucky_count, lucky_numbers, nth_lucky, prime_power_count, prime_power_count_approx,
    prime_power_count_lower, prime_power_count_upper,
};
fn bench_families(c: &mut Criterion) {
    c.bench_function("prime_power_count", |b| {
        b.iter(|| prime_power_count(black_box(1_000_000)));
    });
    c.bench_function("prime_power_count_approx", |b| {
        b.iter(|| prime_power_count_approx(black_box(100_000_000)));
    });
    c.bench_function("prime_power_count_lower", |b| {
        b.iter(|| prime_power_count_lower(black_box(100_000_000)));
    });
    c.bench_function("prime_power_count_upper", |b| {
        b.iter(|| prime_power_count_upper(black_box(100_000_000)));
    });
    c.bench_function("lucky_numbers", |b| {
        b.iter(|| lucky_numbers(0, black_box(1_000_000)));
    });
    c.bench_function("lucky_count", |b| {
        b.iter(|| lucky_count(black_box(1_000_000)));
    });
    c.bench_function("nth_lucky", |b| b.iter(|| nth_lucky(black_box(100_000))));
    c.bench_function("is_lucky", |b| b.iter(|| is_lucky(black_box(971_011))));
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
