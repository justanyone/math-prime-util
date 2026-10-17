//! Benchmarks for the generic integer surface.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::Int;

fn bench_generic(c: &mut Criterion) {
    c.bench_function("generic/is_prime", |b| {
        b.iter(|| black_box((1_u128 << 127) - 1).is_prime())
    });
    c.bench_function("generic/factor", |b| {
        b.iter(|| black_box(600_851_475_143_u128).factor())
    });
    c.bench_function("generic/powmod", |b| {
        b.iter(|| black_box(u128::MAX - 2).powmod(&127, &u128::MAX))
    });
    c.bench_function("generic/gcd", |b| {
        b.iter(|| black_box(u128::MAX).gcd(&600_851_475_143))
    });
    c.bench_function("generic/next_prime", |b| {
        b.iter(|| black_box(1_u128 << 64).next_prime())
    });
}
criterion_group!(benches, bench_generic);
criterion_main!(benches);
