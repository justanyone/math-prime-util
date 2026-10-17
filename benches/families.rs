#![allow(missing_docs)]
//! Benchmarks for the lucky-number sieve, rank, and membership paths.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    almost_prime_count, almost_primes, nth_almost_prime, nth_omega_prime, omega_prime_count,
    omega_primes,
};
use math_prime_util::{is_lucky, lucky_count, lucky_numbers, nth_lucky};
fn bench_families(c: &mut Criterion) {
    for (name, f) in [
        (
            "nth_prime_power",
            math_prime_util::nth_prime_power as fn(u64) -> Option<u64>,
        ),
        (
            "nth_prime_power_approx",
            math_prime_util::nth_prime_power_approx,
        ),
        (
            "nth_prime_power_lower",
            math_prime_util::nth_prime_power_lower,
        ),
        (
            "nth_prime_power_upper",
            math_prime_util::nth_prime_power_upper,
        ),
    ] {
        c.bench_function(name, |b| b.iter(|| f(black_box(100_000))));
    }
    c.bench_function("lucky_numbers", |b| {
        b.iter(|| lucky_numbers(0, black_box(1_000_000)));
    });
    c.bench_function("lucky_count", |b| {
        b.iter(|| lucky_count(black_box(1_000_000)));
    });
    c.bench_function("nth_lucky", |b| b.iter(|| nth_lucky(black_box(100_000))));
    c.bench_function("is_lucky", |b| b.iter(|| is_lucky(black_box(971_011))));
}
fn bench_almost_omega(c: &mut Criterion) {
    c.bench_function("almost_prime_count", |b| {
        b.iter(|| almost_prime_count(3, black_box(100_000)));
    });
    c.bench_function("omega_prime_count", |b| {
        b.iter(|| omega_prime_count(3, black_box(100_000)));
    });
    c.bench_function("almost_primes", |b| {
        b.iter(|| almost_primes(3, 0, black_box(100_000)));
    });
    c.bench_function("omega_primes", |b| {
        b.iter(|| omega_primes(3, 0, black_box(100_000)));
    });
    c.bench_function("nth_almost_prime", |b| {
        b.iter(|| nth_almost_prime(3, black_box(1000)));
    });
    c.bench_function("nth_omega_prime", |b| {
        b.iter(|| nth_omega_prime(3, black_box(1000)));
    });
}
criterion_group!(benches, bench_families, bench_almost_omega);
criterion_main!(benches);
