//! Oracle checks for the assigned Ramanujan functions.
mod common;
use common::each;
use math_prime_util::{
    ramanujan_prime_count, ramanujan_prime_count_approx, ramanujan_prime_count_lower,
    ramanujan_primes,
};

#[test]
fn ramanujan_primes_oracle() {
    each("families", "ramanujan_primes", |c| {
        let (lo, hi) = if c.args.len() == 1 {
            (0, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        let values = ramanujan_primes(lo, hi);
        c.check(if values.is_empty() {
            None
        } else {
            Some(
                values
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        });
    });
}

#[test]
fn ramanujan_prime_count_oracle() {
    each("families", "ramanujan_prime_count", |c| {
        let count = if c.args.len() == 1 {
            ramanujan_prime_count(c.arg(0))
        } else {
            let lo: u64 = c.arg(0);
            let hi: u64 = c.arg(1);
            if hi < lo {
                0
            } else {
                ramanujan_prime_count(hi) - ramanujan_prime_count(lo.saturating_sub(1))
            }
        };
        c.check(Some(count));
    });
}

#[test]
fn ramanujan_prime_count_approx_oracle() {
    each("families", "ramanujan_prime_count_approx", |c| {
        c.check(Some(ramanujan_prime_count_approx(c.arg(0))));
    });
}

#[test]
fn ramanujan_prime_count_lower_oracle() {
    each("families", "ramanujan_prime_count_lower", |c| {
        c.check(Some(ramanujan_prime_count_lower(c.arg(0))));
    });
}
