//! Benchmarks for rational operations.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{contfrac, farey_rank, nth_calkin_wilf};
fn rational(c: &mut Criterion) {
    c.bench_function("contfrac", |b| {
        b.iter(|| contfrac(black_box(9_238_492_834), 2_398_702_938_777));
    });
    c.bench_function("nth_calkin_wilf", |b| {
        b.iter(|| nth_calkin_wilf(black_box(1_000_003)));
    });
    c.bench_function("farey_rank", |b| {
        b.iter(|| farey_rank(black_box(10_000), 1, 3));
    });
}
criterion_group!(benches, rational);
criterion_main!(benches);
