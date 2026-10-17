//! Exact count compatibility with the committed MPU oracle.
mod common;
use common::each;
use math_prime_util::{legendre_phi, prime_count, primes_in, sum_primes};

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
    each("count", "legendre_phi", |c| c.check(Some(legendre_phi(c.arg(0), c.arg(1)))));
}

#[test]
fn sum_primes_oracle() {
    each("count", "sum_primes", |c| {
        let (lo, hi) = if c.args.len() == 1 { (0, c.arg(0)) } else { (c.arg(0), c.arg(1)) };
        c.check(sum_primes(lo, hi));
    });
}
