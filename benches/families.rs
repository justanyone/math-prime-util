#![allow(missing_docs)]
//! Benchmarks for the lucky-number, prime-power, twin prime and semiprime functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    is_lucky, lucky_count, lucky_numbers, nth_lucky, nth_twin_prime, nth_twin_prime_approx,
    semi_primes, semiprime_count, semiprime_count_approx, twin_prime_count,
    twin_prime_count_approx, twin_primes,
};
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
    c.bench_function("twin_primes", |b| {
        b.iter(|| twin_primes(0, black_box(100_000)));
    });
    c.bench_function("twin_prime_count", |b| {
        b.iter(|| twin_prime_count(black_box(100_000)));
    });
    c.bench_function("twin_prime_count_approx", |b| {
        b.iter(|| twin_prime_count_approx(black_box(1_000_000_000)));
    });
    c.bench_function("nth_twin_prime", |b| {
        b.iter(|| nth_twin_prime(black_box(1000)));
    });
    c.bench_function("nth_twin_prime_approx", |b| {
        b.iter(|| nth_twin_prime_approx(black_box(100_000)));
    });
    c.bench_function("semi_primes", |b| {
        b.iter(|| semi_primes(black_box(1_000_000), black_box(1_001_000)));
    });
    c.bench_function("semiprime_count", |b| {
        b.iter(|| semiprime_count(black_box(1_000_000)));
    });
    c.bench_function("semiprime_count_approx", |b| {
        b.iter(|| semiprime_count_approx(black_box(1_000_000)));
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
