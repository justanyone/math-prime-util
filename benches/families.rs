#![allow(missing_docs)]
//! Benchmarks for the lucky-number sieve, rank, and membership paths.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    is_lucky, is_prime_power, lucky_count, lucky_numbers, next_prime_power, nth_lucky,
    prev_prime_power, prime_powers,
};
fn bench_families(c: &mut Criterion) {
    c.bench_function("prime_powers", |b| {
        b.iter(|| prime_powers(0, black_box(100_000)));
    });
    c.bench_function("is_prime_power", |b| {
        b.iter(|| is_prime_power(black_box(1_000_000_014_000_000_049)));
    });
    c.bench_function("next_prime_power", |b| {
        b.iter(|| next_prime_power(black_box(1_000_000)));
    });
    c.bench_function("prev_prime_power", |b| {
        b.iter(|| prev_prime_power(black_box(1_000_000)));
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
