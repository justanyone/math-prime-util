#![allow(missing_docs)]
//! Benchmarks for the lucky-number, prime-power and twin prime functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    goldbach_pair_count, goldbach_pairs, is_lucky, is_prime_power, lucky_count, lucky_numbers,
    minimal_goldbach_pair, next_chen_prime, next_prime_power, nth_lucky, nth_ramanujan_prime,
    nth_ramanujan_prime_approx, nth_ramanujan_prime_lower, nth_twin_prime, powerfree_count,
    powerful_count, powerful_numbers, prev_prime_power, prime_powers, ramanujan_prime_count_upper,
    sumpowerful, twin_prime_count, twin_prime_count_approx, twin_primes,
};
fn bench_families(c: &mut Criterion) {
    c.bench_function("prime_powers", |b| {
        b.iter(|| prime_powers(0, black_box(100_000)));
    });
    c.bench_function("is_prime_power", |b| {
        b.iter(|| is_prime_power(black_box(1_000_000_014_000_000_049)));
    });
    c.bench_function("next_prime_power", |b| {
        b.iter(|| next_prime_power(black_box(1_000_000)));
    });
    c.bench_function("prev_prime_power", |b| {
        b.iter(|| prev_prime_power(black_box(1_000_000)));
    });
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
    c.bench_function("powerful_count", |b| {
        b.iter(|| powerful_count(black_box(1_000_000_000), 2));
    });
    c.bench_function("powerful_numbers", |b| {
        b.iter(|| powerful_numbers(1, black_box(1_000_000), 2));
    });
    c.bench_function("sumpowerful", |b| {
        b.iter(|| sumpowerful(black_box(1_000_000_000), 2));
    });
    c.bench_function("powerfree_count", |b| {
        b.iter(|| powerfree_count(black_box(1_000_000_000), 2));
    });
}
fn bench_goldbach_misc(c: &mut Criterion) {
    c.bench_function("minimal_goldbach_pair", |b| {
        b.iter(|| minimal_goldbach_pair(black_box(60_119_912)));
    });
    c.bench_function("goldbach_pair_count", |b| {
        b.iter(|| goldbach_pair_count(black_box(1_000_000)));
    });
    c.bench_function("goldbach_pairs", |b| {
        b.iter(|| goldbach_pairs(black_box(100_000)));
    });
    c.bench_function("next_chen_prime", |b| {
        b.iter(|| next_chen_prime(black_box(10_000_000_000_000_000)));
    });
}
criterion_group!(benches, bench_families, bench_goldbach_misc);
criterion_main!(benches);
