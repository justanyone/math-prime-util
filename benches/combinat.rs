//! Benchmarks for the four native combinatorics functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{bell_number, binomial, catalan_number, stirling};

fn bench_combinat(c: &mut Criterion) {
    c.bench_function("binomial", |b| {
        b.iter(|| binomial(black_box(67), black_box(31)))
    });
    c.bench_function("catalan_number", |b| {
        b.iter(|| catalan_number(black_box(36)))
    });
    c.bench_function("stirling", |b| {
        b.iter(|| stirling(black_box(15), black_box(7), black_box(1)))
    });
    c.bench_function("bell_number", |b| b.iter(|| bell_number(black_box(25))));
}

criterion_group!(benches, bench_combinat);
criterion_main!(benches);
