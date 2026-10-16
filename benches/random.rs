#![allow(missing_docs)]
//! Benchmarks for the MPU ChaCha20 stream.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{csrand, irand32, irand64, random_bytes, urandomm};
fn bench_random(c: &mut Criterion) {
    csrand(&[42; 40]);
    c.bench_function("random/irand32", |b| b.iter(irand32));
    c.bench_function("random/irand64", |b| b.iter(irand64));
    c.bench_function("random/urandomm", |b| {
        b.iter(|| urandomm(black_box(1_000_003)))
    });
    c.bench_function("random/bytes1024", |b| {
        b.iter(|| random_bytes(black_box(1024)))
    });
}
criterion_group!(benches, bench_random);
criterion_main!(benches);
