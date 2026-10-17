//! Compat tests for the sieve family. Fixture: `tests/fixtures/sieve.tsv`.
//!
//! Skipped cases (the Rust API takes `u64` only):
//! - `primes(-4)`, `primes(2, -4)`, `primes(-10, 7)`, `primes(-10, -4)`
//! - `sieve_range(10, 5, 18446744073709551617)` and
//!   `sieve_range(10, 18446744073709551617, 1)` (`2^64 + 1` does not fit)

mod common;

use common::{Case, each, each_may_panic};
use math_prime_util::*;

fn unrepresentable(c: &Case) -> bool {
    c.args
        .iter()
        .any(|a| a.starts_with('-') || a.parse::<u64>().is_err())
}

fn fmt_list(v: &[u64]) -> String {
    v.iter().map(u64::to_string).collect::<Vec<_>>().join(" ")
}

#[test]
fn primes_oracle() {
    each("sieve", "primes", |c| {
        if unrepresentable(c) {
            return;
        }
        let list = if c.args.len() == 1 {
            primes(0, c.arg(0))
        } else {
            primes(c.arg(0), c.arg(1))
        };
        c.check(Some(fmt_list(&list)));
    });
}

#[test]
fn next_prime_oracle() {
    each("sieve", "next_prime", |c| {
        c.check(next_prime(c.arg(0)));
    });
}

#[test]
fn prev_prime_oracle() {
    each("sieve", "prev_prime", |c| {
        c.check(prev_prime(c.arg(0)));
    });
}

#[test]
fn sieve_range_oracle() {
    each("sieve", "sieve_range", |c| {
        if unrepresentable(c) {
            return;
        }
        let list = sieve_range(c.arg(0), c.arg(1), c.arg(2));
        c.check(Some(fmt_list(&list)));
    });
}

#[test]
fn sieve_prime_cluster_oracle() {
    each_may_panic("sieve", "sieve_prime_cluster", |c| {
        let lo = c.arg(0);
        let hi = c.arg(1);
        let offs: Vec<u64> = (2..c.args.len()).map(|i| c.arg(i)).collect();
        Some(fmt_list(&sieve_prime_cluster(lo, hi, &offs)))
    });
}

#[test]
fn sum_primes_oracle() {
    each("sieve", "sum_primes", |c| {
        let got = if c.args.len() == 1 {
            sum_primes(0, c.arg(0))
        } else {
            sum_primes(c.arg(0), c.arg(1))
        };
        c.check(got);
    });
}

#[test]
fn cache_state_repeat_and_threads() {
    // Input validation: bad clusters panic (also covered by the oracle).
    assert!(std::panic::catch_unwind(|| sieve_prime_cluster(0, 30, &[2, 0])).is_err());
    // Duplicate prime_precalc must not shrink or deadlock.
    prime_precalc(50_000);
    prime_precalc(50_000);
    prime_precalc(10);
    assert_eq!(next_prime(50_000), Some(50_021));
    let mut joins = Vec::new();
    for i in 0..4u64 {
        joins.push(std::thread::spawn(move || {
            prime_precalc(8_000 + i);
            assert_eq!(primes(0, 20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
            if i % 2 == 0 {
                prime_memfree();
            }
            assert_eq!(prev_prime(20), Some(19));
            assert_eq!(sum_primes(0, 10), Some(17));
        }));
    }
    for join in joins {
        join.join().expect("cache thread");
    }
    prime_memfree();
    assert_eq!(
        sieve_range(0, 20, 2),
        vec![2, 3, 5, 7, 9, 11, 13, 15, 17, 19]
    );
}
