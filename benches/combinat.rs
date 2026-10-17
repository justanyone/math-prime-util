#![allow(missing_docs)]
//! Benchmarks for the factorial family, ordered set partitions and the four native combinat functions.
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    bernfrac, factorial, fubini, harmfrac, multifactorial, pn_primorial, primorial, subfactorial,
};

fn bench_combinat(c: &mut Criterion) {
    c.bench_function("factorial/20", |b| b.iter(|| factorial(black_box(20))));
    c.bench_function("subfactorial/20", |b| {
        b.iter(|| subfactorial(black_box(20)));
    });
    c.bench_function("multifactorial/33/2", |b| {
        b.iter(|| multifactorial(black_box(33), black_box(2)));
    });
    c.bench_function("fubini/15", |b| b.iter(|| fubini(black_box(15))));
    c.bench_function("fubini/18", |b| b.iter(|| fubini(black_box(18))));
    c.bench_function("bernfrac_58", |b| b.iter(|| bernfrac(black_box(58))));
    c.bench_function("harmfrac_60", |b| b.iter(|| harmfrac(black_box(60))));
    c.bench_function("primorial_52", |b| b.iter(|| primorial(black_box(52))));
    c.bench_function("pn_primorial_15", |b| {
        b.iter(|| pn_primorial(black_box(15)));
    });
}

criterion_group!(benches, bench_combinat);
criterion_main!(benches);
