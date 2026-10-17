#![allow(missing_docs)]
//! Benchmarks for the segmented sieve. Compare with `perl tools/bench_vs_perl.pl sieve`.

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{next_prime, prev_prime, primes, sieve_range, sum_primes};

fn bench_sieve(c: &mut Criterion) {
    let mut g = c.benchmark_group("sieve");
    for &n in &[10_000u64, 100_000, 1_000_000] {
        g.bench_with_input(BenchmarkId::new("primes", n), &n, |b, &n| {
            b.iter(|| primes(black_box(0), black_box(n)));
        });
        g.bench_with_input(BenchmarkId::new("next_prime", n), &n, |b, &n| {
            b.iter(|| next_prime(black_box(n)));
        });
        g.bench_with_input(BenchmarkId::new("prev_prime", n), &n, |b, &n| {
            b.iter(|| prev_prime(black_box(n)));
        });
        g.bench_with_input(BenchmarkId::new("sum_primes", n), &n, |b, &n| {
            b.iter(|| sum_primes(black_box(0), black_box(n)));
        });
    }
    g.bench_function("sieve_range_1e6_40", |b| {
        b.iter(|| sieve_range(black_box(1_000_000), black_box(2_000), black_box(40)));
    });
    g.bench_function("next_prime_near_2_63", |b| {
        b.iter(|| next_prime(black_box(9_223_372_036_854_775_807)));
    });
    g.finish();
}

criterion_group!(benches, bench_sieve);
criterion_main!(benches);
