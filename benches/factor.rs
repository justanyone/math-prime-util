#![allow(missing_docs)]
//! ECM benchmarks.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::ecm_factor;

fn bench_factor(c: &mut Criterion) {
    c.bench_function("ecm_factor", |b| {
        b.iter(|| ecm_factor(black_box(1_000_036_000_099), None, None, None));
    });
}
criterion_group!(benches, bench_factor);
criterion_main!(benches);
