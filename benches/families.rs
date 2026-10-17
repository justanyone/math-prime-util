#![allow(missing_docs)]
//! Benchmarks for native perfect powers.
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    is_perfect_power, next_perfect_power, perfect_power_count, prev_perfect_power,
};

fn bench_families(c: &mut Criterion) {
    let mut group = c.benchmark_group("perfect_powers");
    for n in [1_000_003, 4_294_967_296, 1 << 63, u64::MAX] {
        group.bench_with_input(BenchmarkId::new("is_perfect_power", n), &n, |b, &n| {
            b.iter(|| is_perfect_power(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("next_perfect_power", n), &n, |b, &n| {
            b.iter(|| next_perfect_power(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("prev_perfect_power", n), &n, |b, &n| {
            b.iter(|| prev_perfect_power(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("perfect_power_count", n), &n, |b, &n| {
            b.iter(|| perfect_power_count(black_box(n)));
        });
    }
    group.finish();
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
