//! Benchmarks for the four native combinat functions.
use criterion::{Criterion, black_box};
use math_prime_util::{
    bell_number, bernfrac, binomial, catalan_number, consecutive_integer_lcm, factorial,
    falling_factorial, frobenius_number, fubini, harmfrac, integer_complexity, is_congruent_number,
    is_polygonal, multifactorial, pn_primorial, powersum, primorial, rising_factorial, stirling,
    subfactorial,
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
    c.bench_function("consecutive_integer_lcm", |b| {
        b.iter(|| consecutive_integer_lcm(black_box(43)));
    });
    c.bench_function("frobenius_number", |b| {
        b.iter(|| frobenius_number(black_box(&[47, 74, 97, 126, 157, 188])));
    });
    c.bench_function("is_congruent_number", |b| {
        b.iter(|| is_congruent_number(black_box(1001)));
    });
    c.bench_function("is_polygonal", |b| {
        b.iter(|| is_polygonal(black_box(15), black_box(3)));
    });
    c.final_summary();
}
