#![allow(missing_docs)]
//! Benchmarks for Ramanujan prime queries.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    nth_ramanujan_prime, nth_ramanujan_prime_approx, nth_ramanujan_prime_lower,
    ramanujan_prime_count_upper,
};
fn bench_families(c: &mut Criterion) {
    c.bench_function("nth_ramanujan_prime", |b| {
        b.iter(|| nth_ramanujan_prime(black_box(1000)));
    });
    c.bench_function("nth_ramanujan_prime_approx", |b| {
        b.iter(|| nth_ramanujan_prime_approx(black_box(1000)));
    });
    c.bench_function("nth_ramanujan_prime_lower", |b| {
        b.iter(|| nth_ramanujan_prime_lower(black_box(1000)));
    });
    c.bench_function("ramanujan_prime_count_upper", |b| {
        b.iter(|| ramanujan_prime_count_upper(black_box(10000)));
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
