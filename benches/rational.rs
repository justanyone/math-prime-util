#![allow(missing_docs)]
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    bestrational, contfrac, farey, farey_nth, farey_rank, nth_calkin_wilf, nth_stern_brocot,
};

fn bench_rational(c: &mut Criterion) {
    c.bench_function("contfrac", |b| {
        b.iter(|| contfrac(black_box(832_040), 514_229));
    });
    c.bench_function("bestrational", |b| {
        b.iter(|| bestrational(black_box(std::f64::consts::PI), 10_000));
    });
    c.bench_function("nth_calkin_wilf", |b| {
        b.iter(|| nth_calkin_wilf(black_box(1_000_000)));
    });
    c.bench_function("nth_stern_brocot", |b| {
        b.iter(|| nth_stern_brocot(black_box(1_000_000)));
    });
    c.bench_function("farey", |b| b.iter(|| farey(black_box(100))));
    c.bench_function("farey_rank", |b| {
        b.iter(|| farey_rank(black_box(1000), 1, 3));
    });
    c.bench_function("farey_nth", |b| {
        b.iter(|| farey_nth(black_box(1000), 123_456));
    });
}
criterion_group!(benches, bench_rational);
criterion_main!(benches);
