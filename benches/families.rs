#![allow(missing_docs)]
//! Benchmarks for the lucky-number sieve, rank, and membership paths.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    goldbach_pair_count, goldbach_pairs, is_lucky, lucky_count, lucky_numbers,
    minimal_goldbach_pair, next_chen_prime, nth_lucky,
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
}
fn bench_goldbach_misc(c: &mut Criterion) {
    c.bench_function("minimal_goldbach_pair", |b| {
        b.iter(|| minimal_goldbach_pair(black_box(60_119_912)));
    });
    c.bench_function("goldbach_pair_count", |b| {
        b.iter(|| goldbach_pair_count(black_box(1_000_000)));
    });
    c.bench_function("goldbach_pairs", |b| {
        b.iter(|| goldbach_pairs(black_box(100_000)));
    });
    c.bench_function("next_chen_prime", |b| {
        b.iter(|| next_chen_prime(black_box(10_000_000_000_000_000)));
    });
}
criterion_group!(benches, bench_families, bench_goldbach_misc);
criterion_main!(benches);
