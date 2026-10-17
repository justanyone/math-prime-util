#![allow(missing_docs)]
//! Benchmarks for native random sampling.
#![allow(clippy::wildcard_imports)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::*;

fn bench_random(c: &mut Criterion) {
    srand(42);
    csrand(&[42; 40]);
    c.bench_function("random/maurer_cert32", |b| {
        b.iter(|| math_prime_util::random_maurer_prime_with_cert(black_box(32)));
    });
    c.bench_function("random/shawe_taylor32", |b| {
        b.iter(|| math_prime_util::random_shawe_taylor_prime(black_box(32)));
    });
    c.bench_function("random/shawe_taylor_cert64", |b| {
        b.iter(|| math_prime_util::random_shawe_taylor_prime_with_cert(black_box(64)));
    });
    c.bench_function("random/semiprime64", |b| {
        b.iter(|| math_prime_util::random_semiprime(black_box(64)));
    });
    c.bench_function("random/maurer64", |b| {
        b.iter(|| math_prime_util::random_maurer_prime(black_box(64)));
    });
    c.bench_function("random/proven64", |b| {
        b.iter(|| math_prime_util::random_proven_prime(black_box(64)));
    });
    c.bench_function("random/proven_cert64", |b| {
        b.iter(|| math_prime_util::random_proven_prime_with_cert(black_box(64)));
    });
    c.bench_function("random_unrestricted_semiprime/32", |b| {
        b.iter(|| random_unrestricted_semiprime(black_box(32)));
    });
    c.bench_function("random/prime", |b| {
        b.iter(|| random_prime(black_box(1_000_000), black_box(2_000_000)));
    });
    c.bench_function("random/ndigit_prime", |b| {
        b.iter(|| random_ndigit_prime(black_box(12)));
    });
    c.bench_function("random/nbit_prime", |b| {
        b.iter(|| random_nbit_prime(black_box(64)));
    });
    c.bench_function("random/safe_prime", |b| {
        b.iter(|| random_safe_prime(black_box(32)));
    });
    c.bench_function("random/irand32", |b| b.iter(irand32));
    c.bench_function("random/irand64", |b| b.iter(irand64));
    c.bench_function("random/urandomm", |b| {
        b.iter(|| urandomm(black_box(1_000_003)));
    });
    c.bench_function("random_factored_integer/32", |b| {
        b.iter(|| random_factored_integer(black_box(1 << 32)));
    });
    c.bench_function("randperm/sparse", |b| {
        b.iter(|| randperm(black_box(1_000_000), black_box(20)));
    });
    let mut items: Vec<_> = (0..1000).collect();
    c.bench_function("shuffle/1000", |b| {
        b.iter(|| shuffle(black_box(&mut items)));
    });
}
criterion_group!(benches, bench_random);
criterion_main!(benches);
