#![allow(missing_docs)]
//! Benchmarks for the MPU `ChaCha20` stream.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    csrand, irand32, irand64, miller_rabin_random, random_bytes, random_nbit_prime,
    random_ndigit_prime, random_prime, random_safe_prime, urandomm, vecsample,
};
fn bench_random(c: &mut Criterion) {
    csrand(&[42; 40]);
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
    c.bench_function("random/bytes1024", |b| {
        b.iter(|| random_bytes(black_box(1024)));
    });
    let data: Vec<u64> = (0..1000).collect();
    c.bench_function("random/vecsample", |b| {
        b.iter(|| vecsample(black_box(10), black_box(&data)));
    });
    c.bench_function("random/miller_rabin_random", |b| {
        b.iter(|| miller_rabin_random(black_box(1_000_003), black_box(8)));
    });
}
criterion_group!(benches, bench_random);
criterion_main!(benches);
