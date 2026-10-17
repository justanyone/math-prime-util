//! Benchmarks for the assigned perfect-power functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    nth_perfect_power, perfect_power_count_approx, perfect_power_count_lower,
    perfect_power_count_upper,
};

fn bench_families(c: &mut Criterion) {
    c.bench_function("perfect_power_count_approx", |b| {
        b.iter(|| perfect_power_count_approx(black_box(u64::MAX)));
    });
    c.bench_function("perfect_power_count_lower", |b| {
        b.iter(|| perfect_power_count_lower(black_box(u64::MAX)));
    });
    c.bench_function("perfect_power_count_upper", |b| {
        b.iter(|| perfect_power_count_upper(black_box(u64::MAX)));
    });
    c.bench_function("nth_perfect_power", |b| {
        b.iter(|| nth_perfect_power(black_box(1_000_000)));
    });
}

criterion_group!(benches, bench_families);
criterion_main!(benches);
