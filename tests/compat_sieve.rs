//! Checks against the committed MPU sieve oracle.
mod common;
use common::{each, each_may_panic};
use math_prime_util::*;

fn list(values: &[u64]) -> String {
    values
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

fn bounds(c: &common::Case) -> (u64, u64) {
    if c.args.len() == 1 {
        (0, c.arg(0))
    } else {
        (c.arg(0), c.arg(1))
    }
}

#[test]
fn primes_oracle() {
    each("sieve", "primes", |c| {
        // Signed primes arguments (-4, 2 -4, -10 7, -10 -4) cannot enter the u64 API.
        if c.args.iter().any(|a| a.starts_with('-')) {
            return;
        }
        let (lo, hi) = bounds(c);
        c.check(Some(list(&primes(lo, hi))));
    });
}
#[test]
fn next_prime_oracle() {
    each("sieve", "next_prime", |c| c.check(next_prime(c.arg(0))));
}
#[test]
fn prev_prime_oracle() {
    each("sieve", "prev_prime", |c| c.check(prev_prime(c.arg(0))));
}
#[test]
fn sieve_range_oracle() {
    each("sieve", "sieve_range", |c| {
        // Width or depth 18_446_744_073_709_551_617 exceeds the u64 API.
        if c.args.iter().any(|a| a.parse::<u64>().is_err()) {
            return;
        }
        c.check(Some(list(&sieve_range(c.arg(0), c.arg(1), c.arg(2)))));
    });
}
#[test]
fn sieve_prime_cluster_oracle() {
    each_may_panic("sieve", "sieve_prime_cluster", |c| {
        let offsets: Vec<u64> = c.args[2..].iter().map(|v| v.parse().unwrap()).collect();
        Some(list(&sieve_prime_cluster(c.arg(0), c.arg(1), &offsets)))
    });
}
#[test]
fn sum_primes_oracle() {
    each("sieve", "sum_primes", |c| {
        let (lo, hi) = bounds(c);
        c.check(sum_primes(lo, hi));
    });
}
#[test]
fn prime_precalc_behavior() {
    // The committed fixture has no prime_precalc rows.
    prime_precalc(10000);
    prime_precalc(100);
    prime_precalc(10000);
    assert_eq!(primes(990, 1000), vec![991, 997]);
}
#[test]
fn prime_memfree_behavior() {
    // The committed fixture has no prime_memfree rows.
    prime_precalc(1000);
    prime_memfree();
    prime_memfree();
    assert_eq!(primes(2, 10), vec![2, 3, 5, 7]);
}
