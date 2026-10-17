//! Hot vector and set benchmarks.
#![allow(missing_docs)] // Criterion generates a public benchmark entry point.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{setunion, sumset, vecsort, vecsum};
fn bench_vec(c: &mut Criterion) {
    let a: Vec<i64> = (0..2000).map(|i| (i * 7919) % 2003).collect();
    let u: Vec<u64> = (0..2000).collect();
    c.bench_function("vecsum", |b| b.iter(|| vecsum(black_box(&u))));
    c.bench_function("vecsort", |b| b.iter(|| vecsort(black_box(&a))));
    c.bench_function("setunion", |b| {
        b.iter(|| setunion(black_box(&a), black_box(&a)));
    });
    c.bench_function("sumset", |b| {
        b.iter(|| sumset(black_box(&a[..40]), black_box(&a[..40])));
    });
}
criterion_group!(benches, bench_vec);
criterion_main!(benches);
