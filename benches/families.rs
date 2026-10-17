#![allow(missing_docs)]
//! Hot powerful and powerfree operations.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{powerfree_count, powerful_count, powerful_numbers, sumpowerful};

fn bench_families(c: &mut Criterion) {
    c.bench_function("powerful_count", |b| {
        b.iter(|| powerful_count(black_box(1_000_000_000), 2));
    });
    c.bench_function("powerful_numbers", |b| {
        b.iter(|| powerful_numbers(1, black_box(1_000_000), 2));
    });
    c.bench_function("sumpowerful", |b| {
        b.iter(|| sumpowerful(black_box(1_000_000_000), 2));
    });
    c.bench_function("powerfree_count", |b| {
        b.iter(|| powerfree_count(black_box(1_000_000_000), 2));
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
