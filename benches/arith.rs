//! Benchmarks for the totient and Moebius functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{euler_phi, euler_phi_range, mertens, moebius, sumtotient};
fn bench_arith(c: &mut Criterion) {
    c.bench_function("euler_phi", |b| {
        b.iter(|| euler_phi(black_box(1_000_000_007)))
    });
    c.bench_function("moebius", |b| b.iter(|| moebius(black_box(1_000_000_007))));
    c.bench_function("totient_range", |b| {
        b.iter(|| euler_phi_range(0, black_box(10000)))
    });
    c.bench_function("mertens", |b| b.iter(|| mertens(black_box(100000))));
    c.bench_function("sumtotient", |b| b.iter(|| sumtotient(black_box(100000))));
}
criterion_group!(benches, bench_arith);
criterion_main!(benches);
