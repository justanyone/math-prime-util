//! Twin prime compatibility with the committed MPU fixtures.
mod common;

use common::each;
use math_prime_util::{nth_twin_prime, twin_prime_count, twin_prime_count_approx, twin_primes};

#[test]
fn twin_primes_oracle() {
    each("families", "twin_primes", |c| {
        let (lo, hi) = if c.args.len() == 1 {
            (0, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        let values = twin_primes(lo, hi);
        c.check((!values.is_empty()).then(|| {
            values
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        }));
    });
}

#[test]
fn twin_prime_count_oracle() {
    each("families", "twin_prime_count", |c| {
        let value = if c.args.len() == 1 {
            twin_prime_count(c.arg(0))
        } else {
            let lo: u64 = c.arg(0);
            let hi: u64 = c.arg(1);
            if lo > hi {
                0
            } else {
                twin_prime_count(hi) - lo.checked_sub(1).map_or(0, twin_prime_count)
            }
        };
        c.check(Some(value));
    });
}

#[test]
fn twin_prime_count_approx_oracle() {
    each("families", "twin_prime_count_approx", |c| {
        c.check(Some(twin_prime_count_approx(c.arg(0))));
    });
}

#[test]
fn nth_twin_prime_oracle() {
    each("families", "nth_twin_prime", |c| {
        c.check(nth_twin_prime(c.arg(0)));
    });
}
