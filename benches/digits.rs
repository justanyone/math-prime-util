#![allow(missing_docs)]
//! Hot digit and checked integer operations.
#![allow(clippy::wildcard_imports)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::digits::*;

fn bench_digits(c: &mut Criterion) {
    c.bench_function("todigits", |b| {
        b.iter(|| todigits(black_box(i128::from(u64::MAX)), 10, None));
    });
    c.bench_function("sumdigits", |b| {
        b.iter(|| sumdigits(black_box("18446744073709551615"), None));
    });
    c.bench_function("tozeckendorf", |b| {
        b.iter(|| tozeckendorf(black_box(u64::MAX)));
    });
    c.bench_function("mulint", |b| {
        b.iter(|| mulint(black_box(1_i128 << 63), black_box(97)));
    });
}
criterion_group!(benches, bench_digits);
criterion_main!(benches);
