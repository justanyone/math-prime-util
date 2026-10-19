#![allow(missing_docs)]
//! Benchmarks comparing the arbitrary precision backends: `bigint`
//! (num-bigint, pure Rust) and `gmp` (rug over the system GMP) on a
//! 200-digit `is_prime` and a 60-digit `factor`. Without either feature
//! the bench has no cases.
use criterion::{Criterion, criterion_group, criterion_main};

/// The first prime above 10^199.
#[cfg(any(feature = "bigint", feature = "gmp"))]
const PRIME_200: &str = "1\
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000153";

/// 1000003 * 1000033 * 1000081 * (10^41 + 109): three Brent splits, then a
/// 42-digit prime cofactor for the BPSW test.
#[cfg(any(feature = "bigint", feature = "gmp"))]
const COMPOSITE_60: &str = "100011700301500801900000000000000000000109012753328635874071";

#[cfg(any(feature = "bigint", feature = "gmp"))]
fn bench_backends(c: &mut Criterion) {
    use criterion::{BenchmarkId, black_box};
    use math_prime_util::Int;

    let mut group = c.benchmark_group("bigint");
    #[cfg(feature = "bigint")]
    {
        let p: num_bigint::BigUint = PRIME_200.parse().unwrap();
        let n: num_bigint::BigUint = COMPOSITE_60.parse().unwrap();
        group.bench_function(BenchmarkId::new("is_prime_200", "num-bigint"), |b| {
            b.iter(|| black_box(&p).is_prime());
        });
        group.bench_function(BenchmarkId::new("factor_60", "num-bigint"), |b| {
            b.iter(|| black_box(&n).factor());
        });
    }
    #[cfg(feature = "gmp")]
    {
        let p: rug::Integer = PRIME_200.parse().unwrap();
        let n: rug::Integer = COMPOSITE_60.parse().unwrap();
        group.bench_function(BenchmarkId::new("is_prime_200", "rug"), |b| {
            b.iter(|| black_box(&p).is_prime());
        });
        group.bench_function(BenchmarkId::new("factor_60", "rug"), |b| {
            b.iter(|| black_box(&n).factor());
        });
    }
    group.finish();
}

#[cfg(not(any(feature = "bigint", feature = "gmp")))]
fn bench_backends(_: &mut Criterion) {}

criterion_group!(benches, bench_backends);
criterion_main!(benches);
