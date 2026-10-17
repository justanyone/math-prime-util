//! Benchmarks for native modular roots.
#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{allrootmod, allsqrtmod, cornacchia, divmod, rootmod, sqrtmod};
fn bench_modular(c: &mut Criterion) {
    c.bench_function("sqrtmod", |b| b.iter(|| sqrtmod(black_box(42), 1_000_003)));
    c.bench_function("allsqrtmod", |b| b.iter(|| allsqrtmod(black_box(1), 72)));
    c.bench_function("rootmod", |b| b.iter(|| rootmod(black_box(8), 3, 1009)));
    c.bench_function("allrootmod", |b| {
        b.iter(|| allrootmod(black_box(1), 6, 1009))
    });
    c.bench_function("cornacchia", |b| b.iter(|| cornacchia(black_box(5), 29)));
    c.bench_function("divmod", |b| b.iter(|| divmod(black_box(6), 2, 1009)));
}
criterion_group!(benches, bench_modular);
criterion_main!(benches);
