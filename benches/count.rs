//! Benchmarks for prime-count bounds and the approximation.
//! Compare with `perl tools/bench_vs_perl.pl` on the same machine.
//! `prime_count(10^k)` for `k` up to 13 is the LMO count in `src/count`.
#![allow(missing_docs)]
#![allow(clippy::wildcard_imports)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::*;

fn bench_count(c: &mut Criterion) {
    let mut g = c.benchmark_group("count");
    // Powers of ten that stay inside a short bench sample. `10^13` is the
    // top of the task range; LMO on that input is one sample, not a loop.
    for n in [10_u64.pow(6), 10_u64.pow(9), 10_u64.pow(12)] {
        g.bench_function(format!("prime_count_lower/{n}"), |b| {
            b.iter(|| prime_count_lower(black_box(n)));
        });
        g.bench_function(format!("prime_count_upper/{n}"), |b| {
            b.iter(|| prime_count_upper(black_box(n)));
        });
        g.bench_function(format!("prime_count_approx/{n}"), |b| {
            b.iter(|| prime_count_approx(black_box(n)));
        });
    }
    g.bench_function("prime_count/10^7", |b| {
        b.iter(|| prime_count(black_box(10_000_000)));
    });
    g.finish();
}

criterion_group!(benches, bench_count);
criterion_main!(benches);
