#![allow(missing_docs)]
//! Benchmarks for the five native 64-bit bit operations.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{bitand, bitandnot, bitnot, bitor, bitxor};

fn bench_bits_ops1(c: &mut Criterion) {
    let a = 0xaaaa_aaaa_aaaa_aaaa_u64;
    let b = 0xcccc_cccc_cccc_cccc_u64;
    let mut group = c.benchmark_group("bits_ops1");
    group.bench_function("bitand", |bench| {
        bench.iter(|| bitand(black_box(a), black_box(b)));
    });
    group.bench_function("bitor", |bench| {
        bench.iter(|| bitor(black_box(a), black_box(b)));
    });
    group.bench_function("bitxor", |bench| {
        bench.iter(|| bitxor(black_box(a), black_box(b)));
    });
    group.bench_function("bitnot", |bench| {
        bench.iter(|| bitnot(black_box(a)));
    });
    group.bench_function("bitandnot", |bench| {
        bench.iter(|| bitandnot(black_box(a), black_box(b)));
    });
    group.finish();
}

criterion_group!(benches, bench_bits_ops1);
criterion_main!(benches);
