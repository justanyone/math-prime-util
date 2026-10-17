#![allow(missing_docs)]
//! Benchmarks for the lucky-number sieve, rank, and membership paths.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    is_lucky, is_ramanujan_prime, lucky_count, lucky_numbers, nth_lucky, nth_ramanujan_prime_upper,
};
fn bench_families(c: &mut Criterion) {
    c.bench_function("lucky_numbers", |b| {
        b.iter(|| lucky_numbers(0, black_box(1_000_000)));
    });
    c.bench_function("lucky_count", |b| {
        b.iter(|| lucky_count(black_box(1_000_000)));
    });
    c.bench_function("nth_lucky", |b| b.iter(|| nth_lucky(black_box(100_000))));
    c.bench_function("is_lucky", |b| b.iter(|| is_lucky(black_box(971_011))));
    c.bench_function("nth_ramanujan_prime_upper", |b| {
        b.iter(|| nth_ramanujan_prime_upper(black_box(1_000_000)));
    });
    c.bench_function("is_ramanujan_prime", |b| {
        b.iter(|| is_ramanujan_prime(black_box(947)));
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
