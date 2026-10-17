#![allow(missing_docs)]
//! Benchmarks for the lucky-number, prime-power and Ramanujan prime families.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    is_lucky, lucky_count, lucky_numbers, nth_lucky, nth_ramanujan_prime,
    nth_ramanujan_prime_approx, nth_ramanujan_prime_lower, ramanujan_prime_count_upper,
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
    c.bench_function("nth_ramanujan_prime", |b| {
        b.iter(|| nth_ramanujan_prime(black_box(1000)));
    });
    c.bench_function("nth_ramanujan_prime_approx", |b| {
        b.iter(|| nth_ramanujan_prime_approx(black_box(1000)));
    });
    c.bench_function("nth_ramanujan_prime_lower", |b| {
        b.iter(|| nth_ramanujan_prime_lower(black_box(1000)));
    });
    c.bench_function("ramanujan_prime_count_upper", |b| {
        b.iter(|| ramanujan_prime_count_upper(black_box(10000)));
    });
}
criterion_group!(benches, bench_families);
criterion_main!(benches);
