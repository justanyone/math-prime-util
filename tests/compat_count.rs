//! Compat tests against MPU 0.75 `prime_counts.c` for the prime counts, their
//! bounds and the nth-prime bounds. Fixture: `tests/fixtures/count.tsv`.
//!
//! Count bounds use the x87 long double model. Only nth-prime rows above
//! the largest 64-bit prime index are skipped.

mod common;

use common::{each, load};
use math_prime_util::*;

#[test]
fn compat_count_prime_count_lower() {
    each("count", "prime_count_lower", |c| {
        c.check(Some(prime_count_lower(c.arg(0))));
    });
}

#[test]
fn compat_count_prime_count_upper() {
    each("count", "prime_count_upper", |c| {
        c.check(Some(prime_count_upper(c.arg(0))));
    });
}

#[test]
fn compat_count_prime_count_approx() {
    each("count", "prime_count_approx", |c| {
        c.check(Some(prime_count_approx(c.arg(0))));
    });
}

#[test]
fn count_bounds_repeated_and_concurrent() {
    // These endpoint values differ on the old f64 paths. Interleave the
    // exact and estimate paths to check that calls do not share state.
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..2 {
                    assert_eq!(prime_count_lower(u64::MAX), 425_656_283_922_083_342);
                    assert_eq!(prime_count_upper(u64::MAX), 425_656_284_115_718_592);
                    assert_eq!(prime_count_approx(u64::MAX), 425_656_284_014_012_123);
                    for f in [prime_count_lower, prime_count_upper, prime_count_approx] {
                        assert_eq!(f(0), 0);
                        assert_eq!(f(1), 0);
                        assert_eq!(f(2), 1);
                    }
                }
            });
        }
    });
}

/// `nth_prime_*` rows with `n` above `MPU_MAX_PRIME_IDX`
/// (`425_656_284_035_217_743`): MPU's XS hands them to its bigint Perl code,
/// and the `n`th prime is above `u64::MAX`, so the Rust functions return
/// `None` there. Skipped for each of the three functions:
/// `425_656_284_035_217_744` and `425_656_284_035_217_745`.
const ABOVE_MAX_PRIME_IDX: [u64; 2] = [425_656_284_035_217_744, 425_656_284_035_217_745];

/// Checks every fixture row of `func`, skipping only [`ABOVE_MAX_PRIME_IDX`].
fn check_nth(func: &str, f: fn(u64) -> Option<u64>) {
    let mut skipped = Vec::new();
    each("count", func, |c| {
        let n: u64 = c.arg(0);
        if ABOVE_MAX_PRIME_IDX.contains(&n) {
            assert_eq!(f(n), None, "{func}({n})");
            skipped.push(n);
        } else {
            c.check(f(n));
        }
    });
    assert_eq!(skipped, ABOVE_MAX_PRIME_IDX);
}

#[test]
fn compat_count_nth_prime_lower() {
    check_nth("nth_prime_lower", nth_prime_lower);
}

#[test]
fn compat_count_nth_prime_upper() {
    check_nth("nth_prime_upper", nth_prime_upper);
}

#[test]
fn compat_count_nth_prime_approx() {
    check_nth("nth_prime_approx", nth_prime_approx);
}

#[test]
fn prime_count_oracle() {
    each("count", "prime_count", |c| {
        let value = if c.args.len() == 2 {
            primes_in(c.arg(0)..=c.arg(1)).count() as u64
        } else {
            prime_count(c.arg(0))
        };
        c.check(Some(value));
    });
}

#[test]
fn legendre_phi_oracle() {
    each("count", "legendre_phi", |c| {
        c.check(Some(legendre_phi(c.arg(0), c.arg(1))));
    });
}

#[test]
fn sum_primes_oracle() {
    each("count", "sum_primes", |c| {
        let (lo, hi) = if c.args.len() == 1 {
            (0, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        c.check(sum_primes(lo, hi));
    });
}

#[test]
fn nth_prime_oracle() {
    each("count", "nth_prime", |c| {
        c.check(nth_prime(c.arg(0)));
    });
}

/// `chebyshev_theta` and `chebyshev_psi` are floats: MPU prints 15 digits
/// and sums in long double, so compare with a relative tolerance.
/// Cases with an argument above 10^8 are skipped: the `f64` sum needs a
/// sieve up to that argument, which is too slow for a default test.
#[test]
fn chebyshev_oracle() {
    for c in load("count")
        .iter()
        .filter(|c| c.func.starts_with("chebyshev_") && c.arg::<u64>(0) <= 100_000_000)
    {
        let want: f64 = c
            .result
            .as_deref()
            .expect("chebyshev results are defined")
            .parse()
            .unwrap();
        let got = if c.func == "chebyshev_theta" {
            chebyshev_theta(c.arg(0))
        } else {
            chebyshev_psi(c.arg(0))
        };
        assert!(
            (got - want).abs() <= 1e-9 * want.abs().max(1.0),
            "fixture line {}: {} {got} vs {want}",
            c.line,
            c.func
        );
    }
}
