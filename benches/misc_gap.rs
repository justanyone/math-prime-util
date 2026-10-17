//! Benchmarks for the misc gap helpers.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{floor_sum, print_primes, remove_factors, remove_factors_exp};

fn bench_misc_gap(c: &mut Criterion) {
    c.bench_function("floor_sum", |b| {
        b.iter(|| floor_sum(black_box(1_000_000), 997, 613, 37));
    });
    c.bench_function("remove_factors", |b| {
        b.iter(|| remove_factors(black_box(1 << 63), 2));
    });
    c.bench_function("remove_factors_exp", |b| {
        b.iter(|| remove_factors_exp(black_box(3u64.pow(40)), 3));
    });
    c.bench_function("print_primes", |b| {
        b.iter(|| print_primes(2, black_box(10_000), std::io::sink()).unwrap());
    });
}

criterion_group!(benches, bench_misc_gap);
criterion_main!(benches);
