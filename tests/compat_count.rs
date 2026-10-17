//! Exact count compatibility with the committed MPU oracle.
mod common;
use common::{each, load};
use math_prime_util::{
    chebyshev_psi, chebyshev_theta, legendre_phi, nth_prime, prime_count, primes_in, sum_primes,
};

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
