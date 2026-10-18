#![allow(missing_docs)]
//! Benchmarks for the factor family.
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use math_prime_util::{
    cheb_factor, divisors, ecm_factor, factor, factor_exp, factor_u128, fermat_factor, holf_factor,
    is_almost_prime, is_carmichael, is_cyclic, is_omega_prime, is_perfect_number, is_practical,
    is_prime_u128, is_quasi_carmichael, is_rough, is_semiprime, is_smooth, is_square_free,
    lehman_factor, pbrent_factor, pminus1_factor, pplus1_factor, prho_factor, prime_bigomega,
    prime_omega, prime_signature, smooth_count, squfof_factor, trial_factor,
};

fn bench_factor(c: &mut Criterion) {
    let n = 1_000_036_000_099;
    c.bench_function("fermat_factor", |b| {
        b.iter(|| fermat_factor(black_box(n), None));
    });
    c.bench_function("holf_factor", |b| {
        b.iter(|| holf_factor(black_box(n), None));
    });
    c.bench_function("lehman_factor", |b| {
        b.iter(|| lehman_factor(black_box(n), None));
    });
    c.bench_function("squfof_factor", |b| {
        b.iter(|| squfof_factor(black_box(n), None));
    });
    c.bench_function("is_carmichael", |b| {
        b.iter(|| is_carmichael(black_box(3_215_031_751)));
    });
    c.bench_function("is_quasi_carmichael", |b| {
        b.iter(|| is_quasi_carmichael(black_box(255_255)));
    });
    c.bench_function("is_cyclic", |b| {
        b.iter(|| is_cyclic(black_box(1009 * 1013)));
    });
    c.bench_function("is_perfect_number", |b| {
        b.iter(|| is_perfect_number(black_box(2_305_843_008_139_952_128)));
    });
    {
        let n = 1_000_036_000_099;
        c.bench_function("prho_factor", |b| {
            b.iter(|| prho_factor(black_box(n), None));
        });
        c.bench_function("pbrent_factor", |b| {
            b.iter(|| pbrent_factor(black_box(n), None, None));
        });
        c.bench_function("pminus1_factor", |b| {
            b.iter(|| pminus1_factor(black_box(n), None, None));
        });
        c.bench_function("pplus1_factor", |b| {
            b.iter(|| pplus1_factor(black_box(n), None));
        });
    }
    c.bench_function("ecm_factor", |b| {
        b.iter(|| ecm_factor(black_box(1_000_036_000_099), None, None, None));
    });
    c.bench_function("cheb_factor", |b| {
        b.iter(|| cheb_factor(black_box(1_000_036_000_099), 0, 0));
    });
    c.bench_function("prime_bigomega", |b| {
        b.iter(|| prime_bigomega(black_box(600_851_475_143)));
    });
    c.bench_function("prime_omega", |b| {
        b.iter(|| prime_omega(black_box(600_851_475_143)));
    });
    c.bench_function("prime_signature", |b| {
        b.iter(|| prime_signature(black_box(600_851_475_143)));
    });
    c.bench_function("is_practical", |b| {
        b.iter(|| is_practical(black_box(720_720)));
    });
    c.bench_function("is_smooth", |b| {
        b.iter(|| is_smooth(black_box(614_889_782_588_491_410), 100));
    });
    c.bench_function("is_rough", |b| {
        b.iter(|| is_rough(black_box(1_000_003 * 1_000_033), 10_000));
    });
    c.bench_function("smooth_count", |b| {
        b.iter(|| smooth_count(black_box(10_000_000), 400));
    });
    let mut group = c.benchmark_group("factor");
    for n in [30_u64, 1_000_003 * 1_000_033, u64::MAX] {
        group.bench_with_input(BenchmarkId::new("is_square_free", n), &n, |b, &n| {
            b.iter(|| is_square_free(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("is_semiprime", n), &n, |b, &n| {
            b.iter(|| is_semiprime(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("is_almost_prime", n), &n, |b, &n| {
            b.iter(|| is_almost_prime(black_box(3), black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("is_omega_prime", n), &n, |b, &n| {
            b.iter(|| is_omega_prime(black_box(3), black_box(n)));
        });
    }
    group.finish();
}
fn bench_wide(c: &mut Criterion) {
    let n = 595_424_139_679_831_880_741u128;
    c.bench_function("factor_u128", |b| b.iter(|| factor_u128(black_box(n))));
    c.bench_function("is_prime_u128", |b| {
        b.iter(|| is_prime_u128(black_box((1u128 << 127) - 1)));
    });
}

fn bench_native(c: &mut Criterion) {
    let mut group = c.benchmark_group("factor");
    for n in [360, 600_851_475_143, 1_000_036_000_099, u64::MAX] {
        group.bench_with_input(BenchmarkId::new("factor", n), &n, |b, &n| {
            b.iter(|| factor(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("factor_exp", n), &n, |b, &n| {
            b.iter(|| factor_exp(black_box(n)));
        });
        group.bench_with_input(BenchmarkId::new("divisors", n), &n, |b, &n| {
            b.iter(|| divisors(black_box(n), None));
        });
        group.bench_with_input(BenchmarkId::new("trial_factor", n), &n, |b, &n| {
            b.iter(|| trial_factor(black_box(n), 1000));
        });
    }
    group.finish();
}
criterion_group!(benches, bench_factor, bench_native, bench_wide);
criterion_main!(benches);
