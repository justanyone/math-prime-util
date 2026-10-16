#![allow(missing_docs)]
//! Benchmarks for the integer and modular core. Compare with
//! `perl tools/bench_vs_perl.pl int` on the same machine.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::*;

fn bench_int(c: &mut Criterion) {
    let inputs: [u64; 4] = [1_000_003, 4_294_967_311, (1 << 61) - 1, u64::MAX - 58];
    let mut g = c.benchmark_group("int");
    for &n in &inputs {
        g.bench_with_input(BenchmarkId::new("sqrtint", n), &n, |b, &n| {
            b.iter(|| sqrtint(black_box(n)));
        });
        g.bench_with_input(BenchmarkId::new("rootint3", n), &n, |b, &n| {
            b.iter(|| rootint(black_box(n), 3));
        });
        g.bench_with_input(BenchmarkId::new("rootint7", n), &n, |b, &n| {
            b.iter(|| rootint(black_box(n), 7));
        });
        g.bench_with_input(BenchmarkId::new("is_power_exp", n), &n, |b, &n| {
            b.iter(|| int::is_power_exp(black_box(n)));
        });
        g.bench_with_input(BenchmarkId::new("logint10", n), &n, |b, &n| {
            b.iter(|| logint(black_box(n), 10));
        });
        g.bench_with_input(BenchmarkId::new("gcd", n), &n, |b, &n| {
            b.iter(|| gcd(black_box(n), black_box(600_851_475_143)));
        });
    }
    g.finish();

    let mut g = c.benchmark_group("modular");
    for &n in &inputs {
        let a = n - 12_345;
        g.bench_with_input(BenchmarkId::new("mulmod", n), &n, |b, &n| {
            b.iter(|| mulmod(black_box(a), black_box(a - 1), n));
        });
        g.bench_with_input(BenchmarkId::new("powmod", n), &n, |b, &n| {
            b.iter(|| powmod(black_box(a), black_box(n - 1), n));
        });
        g.bench_with_input(BenchmarkId::new("invmod", n), &n, |b, &n| {
            b.iter(|| invmod(black_box(a), n));
        });
    }
    g.finish();
}

criterion_group!(benches, bench_int);
criterion_main!(benches);
