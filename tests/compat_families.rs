//! Compatibility with the committed MPU families oracle.
mod common;
use common::each;
use math_prime_util::{
    nth_twin_prime_approx, semi_primes, semiprime_count, semiprime_count_approx,
};

#[test]
fn nth_twin_prime_approx_oracle() {
    each("families", "nth_twin_prime_approx", |c| {
        c.check(nth_twin_prime_approx(c.arg(0)));
    });
}

#[test]
fn semi_primes_oracle() {
    each("families", "semi_primes", |c| {
        let (lo, hi) = if c.args.len() == 1 {
            (0, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        let values = semi_primes(lo, hi);
        let result = (!values.is_empty()).then(|| {
            values
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        });
        c.check(result);
    });
}

#[test]
fn semiprime_count_oracle() {
    each("families", "semiprime_count", |c| {
        let result = if c.args.len() == 1 {
            semiprime_count(c.arg(0))
        } else {
            let lo: u64 = c.arg(0);
            semiprime_count(c.arg(1)) - lo.checked_sub(1).map_or(0, semiprime_count)
        };
        c.check(Some(result));
    });
}

#[test]
fn semiprime_count_approx_oracle() {
    each("families", "semiprime_count_approx", |c| {
        c.check(Some(semiprime_count_approx(c.arg(0))));
    });
}
