//! Benchmarks for the four native combinat functions.
use criterion::{Criterion, black_box};
use math_prime_util::{
    bernfrac, harmfrac, partitions, partitionsq, permtonum, pn_primorial, primorial,
};

#[test]
fn combinat() {
    let mut c = Criterion::default().configure_from_args();
    c.bench_function("bernfrac_58", |b| b.iter(|| bernfrac(black_box(58))));
    c.bench_function("harmfrac_60", |b| b.iter(|| harmfrac(black_box(60))));
    c.bench_function("primorial_52", |b| b.iter(|| primorial(black_box(52))));
    c.bench_function("pn_primorial_15", |b| {
        b.iter(|| pn_primorial(black_box(15)));
    });
    c.bench_function("partitions_200", |b| b.iter(|| partitions(black_box(200))));
    c.bench_function("partitionsq_400", |b| {
        b.iter(|| partitionsq(black_box(400)));
    });
    let perm: Vec<u64> = (0..20).rev().collect();
    c.bench_function("permtonum_20", |b| b.iter(|| permtonum(black_box(&perm))));
    c.final_summary();
}
