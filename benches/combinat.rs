//! Benchmarks for the four native combinat functions.
use criterion::{Criterion, black_box};
use math_prime_util::{
    bell_number, bernfrac, binomial, catalan_number, factorial, falling_factorial, fubini,
    harmfrac, hclassno, integer_complexity, is_happy, is_pillai, multifactorial, numtoperm,
    pn_primorial, powersum, primorial, rising_factorial, stirling, subfactorial,
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

    c.bench_function("falling_factorial", |b| {
        b.iter(|| falling_factorial(black_box(509), 7));
    });
    c.bench_function("rising_factorial", |b| {
        b.iter(|| rising_factorial(black_box(509), 7));
    });
    c.bench_function("powersum", |b| b.iter(|| powersum(black_box(116), 8)));
    c.bench_function("integer_complexity_cached", |b| {
        b.iter(|| integer_complexity(black_box(100_000)));
    });

    c.bench_function("is_pillai", |b| b.iter(|| is_pillai(black_box(1009))));
    c.bench_function("is_happy", |b| {
        b.iter(|| is_happy(black_box(u64::MAX), 10, 2));
    });
    c.bench_function("hclassno", |b| b.iter(|| hclassno(black_box(1_000_003))));
    c.bench_function("numtoperm", |b| {
        b.iter(|| numtoperm(24, black_box(987_654_321)));
    });
    c.final_summary();
}
