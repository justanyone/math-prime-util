//! Benchmarks for the four native combinat functions.
use criterion::{Criterion, black_box};
use math_prime_util::{
    bell_number, bernfrac, binomial, catalan_number, harmfrac, pn_primorial, primorial, stirling,
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

    c.bench_function("binomial", |b| {
        b.iter(|| binomial(black_box(67), black_box(31)));
    });
    c.bench_function("catalan_number", |b| {
        b.iter(|| catalan_number(black_box(36)));
    });
    c.bench_function("stirling", |b| {
        b.iter(|| stirling(black_box(15), black_box(7), black_box(1)));
    });
    c.bench_function("bell_number", |b| b.iter(|| bell_number(black_box(25))));
    c.final_summary();
}
