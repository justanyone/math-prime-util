//! Benchmarks for the four native combinat functions.
use criterion::{Criterion, black_box};
use math_prime_util::{
    bell_number, bernfrac, binomial, catalan_number, factorial, fubini, harmfrac, multifactorial,
    pn_primorial, primorial, stirling, subfactorial,
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

    c.bench_function("factorial/20", |b| b.iter(|| factorial(black_box(20))));
    c.bench_function("subfactorial/20", |b| {
        b.iter(|| subfactorial(black_box(20)));
    });
    c.bench_function("multifactorial/33/2", |b| {
        b.iter(|| multifactorial(black_box(33), black_box(2)));
    });
    c.bench_function("fubini/15", |b| b.iter(|| fubini(black_box(15))));
    c.bench_function("fubini/18", |b| b.iter(|| fubini(black_box(18))));
    c.final_summary();
}
