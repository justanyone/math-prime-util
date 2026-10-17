//! Benchmarks for native bit-position operations.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{bitclear, bitflip, bitscan0, bitscan1, bitset, bittest};

fn bench_bits_pos(c: &mut Criterion) {
    let n = 0x1234_5678_9abc_def0_u64;
    c.bench_function("bitset", |b| b.iter(|| bitset(black_box(n), black_box(31))));
    c.bench_function("bitclear", |b| {
        b.iter(|| bitclear(black_box(n), black_box(31)))
    });
    c.bench_function("bitflip", |b| {
        b.iter(|| bitflip(black_box(n), black_box(31)))
    });
    c.bench_function("bittest", |b| {
        b.iter(|| bittest(black_box(n), black_box(31)))
    });
    c.bench_function("bitscan0", |b| {
        b.iter(|| bitscan0(black_box(n), black_box(16)))
    });
    c.bench_function("bitscan1", |b| {
        b.iter(|| bitscan1(black_box(n), black_box(16)))
    });
}

criterion_group!(benches, bench_bits_pos);
criterion_main!(benches);
