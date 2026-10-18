#![allow(missing_docs)]
//! Benchmarks for the lucky-number, prime-power, twin prime and semiprime functions.
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    goldbach_pair_count, goldbach_pairs, is_gaussian_prime, is_lucky, is_perfect_power,
    is_prime_power, is_sum_of_squares, lucky_count, lucky_numbers, minimal_goldbach_pair,
    next_chen_prime, next_perfect_power, next_prime_power, nth_lucky, nth_perfect_power,
    nth_ramanujan_prime, nth_ramanujan_prime_approx, nth_ramanujan_prime_lower, nth_semiprime,
    nth_semiprime_approx, nth_twin_prime, nth_twin_prime_approx, perfect_power_count,
    perfect_power_count_approx, perfect_power_count_lower, perfect_power_count_upper,
    powerfree_count, powerful_count, powerful_numbers, prev_perfect_power, prev_prime_power,
    prime_power_count, prime_power_count_approx, prime_power_count_lower, prime_power_count_upper,
    prime_powers, ramanujan_prime_count_upper, semi_primes, semiprime_count,
    semiprime_count_approx, sumpowerful, twin_prime_count, twin_prime_count_approx, twin_primes,
};
use math_prime_util::{is_ramanujan_prime, nth_ramanujan_prime_upper};
use math_prime_util::{
    ramanujan_prime_count, ramanujan_prime_count_approx, ramanujan_prime_count_lower,
    ramanujan_primes,
};
fn bench_almost_omega(c: &mut Criterion) {
    c.bench_function("nth_almost_prime_upper", |b| {
        b.iter(|| math_prime_util::nth_almost_prime_upper(4, black_box(5678)));
    });
    c.bench_function("omega_primes", |b| {
        b.iter(|| math_prime_util::omega_primes(3, 1, black_box(100_000)));
    });
    c.bench_function("omega_prime_count", |b| {
        b.iter(|| math_prime_util::omega_prime_count(3, black_box(100_000)));
    });
    c.bench_function("nth_omega_prime", |b| {
        b.iter(|| math_prime_util::nth_omega_prime(3, black_box(1000)));
    });
}

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
    c.bench_function("nth_ramanujan_prime_upper", |b| {
        b.iter(|| nth_ramanujan_prime_upper(black_box(1_000_000)));
    });
    c.bench_function("is_ramanujan_prime", |b| {
        b.iter(|| is_ramanujan_prime(black_box(947)));
    });
    c.bench_function("nth_semiprime", |b| {
        b.iter(|| nth_semiprime(black_box(100_000)));
    });
    c.bench_function("nth_semiprime_approx", |b| {
        b.iter(|| nth_semiprime_approx(black_box(100)));
    });
}
fn bench_ramanujan(c: &mut Criterion) {
    c.bench_function("ramanujan_primes", |b| {
        b.iter(|| ramanujan_primes(black_box(10_000), black_box(20_000)));
    });
    c.bench_function("ramanujan_prime_count", |b| {
        b.iter(|| ramanujan_prime_count(black_box(100_000)));
    });
    c.bench_function("ramanujan_prime_count_approx", |b| {
        b.iter(|| ramanujan_prime_count_approx(black_box(100_000_000)));
    });
    c.bench_function("ramanujan_prime_count_lower", |b| {
        b.iter(|| ramanujan_prime_count_lower(black_box(100_000_000)));
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
fn bench_squares(c: &mut Criterion) {
    c.bench_function("is_gaussian_prime", |b| {
        b.iter(|| is_gaussian_prime(black_box(519_880), black_box(2_265_929)));
    });
    c.bench_function("is_sum_of_squares", |b| {
        b.iter(|| is_sum_of_squares(black_box(18_446_744_073_709_551_557), 2));
    });
}
fn bench_pp_f05(c: &mut Criterion) {
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
fn bench_prime_power_counts(c: &mut Criterion) {
    c.bench_function("prime_power_count", |b| {
        b.iter(|| prime_power_count(black_box(1_000_000)));
    });
    c.bench_function("prime_power_count_approx", |b| {
        b.iter(|| prime_power_count_approx(black_box(100_000_000)));
    });
    c.bench_function("prime_power_count_lower", |b| {
        b.iter(|| prime_power_count_lower(black_box(100_000_000)));
    });
    c.bench_function("prime_power_count_upper", |b| {
        b.iter(|| prime_power_count_upper(black_box(100_000_000)));
    });
}
fn bench_perfect_powers(c: &mut Criterion) {
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
fn bench_almost_omega(c: &mut Criterion) {
    c.bench_function("almost_prime_count_upper", |b| {
        b.iter(|| math_prime_util::almost_prime_count_upper(5, black_box(1_000_000)));
    });
    for (name, f) in [
        (
            "nth_almost_prime",
            math_prime_util::nth_almost_prime as fn(u32, u64) -> Option<u64>,
        ),
        (
            "nth_almost_prime_lower",
            math_prime_util::nth_almost_prime_lower,
        ),
        (
            "nth_almost_prime_approx",
            math_prime_util::nth_almost_prime_approx,
        ),
    ] {
        c.bench_function(name, |b| {
            b.iter(|| f(3, black_box(1000)));
        });
    }
}
criterion_group!(
    benches,
    bench_almost_omega,
    bench_families,
    bench_almost_omega,
    bench_ramanujan,
    bench_goldbach_misc,
    bench_squares,
    bench_pp_f05,
    bench_prime_power_counts,
    bench_perfect_powers
);
criterion_main!(benches);
