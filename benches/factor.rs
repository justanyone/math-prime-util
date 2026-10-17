#![allow(missing_docs)]
//! Benchmarks for the factor family.
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{divisors, factor, factor_exp, trial_factor};

fn bench_factor(c: &mut Criterion) {
    let mut group = c.benchmark_group("factor");
    for n in [360, 600_851_475_143, 1_000_036_000_099, u64::MAX] {
        group.bench_with_input(BenchmarkId::new("factor", n), &n, |b, &n| {
            b.iter(|| factor(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("factor_exp", n), &n, |b, &n| {
            b.iter(|| factor_exp(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("divisors", n), &n, |b, &n| {
            b.iter(|| divisors(black_box(n), None));
        });
        group.bench_with_input(BenchmarkId::new("trial_factor", n), &n, |b, &n| {
            b.iter(|| trial_factor(black_box(n), 1000));
        });
    }
    group.finish();
}
criterion_group!(benches, bench_factor);
criterion_main!(benches);
