//! MPU oracle cases for the four factor predicates.
mod common;

use common::each;
use math_prime_util::{is_almost_prime, is_omega_prime, is_semiprime, is_square_free};

#[test]
fn is_square_free_oracle() {
    each("factor", "is_square_free", |c| {
        c.check(Some(u8::from(is_square_free(c.arg(0)))));
    });
}

#[test]
fn is_semiprime_oracle() {
    each("factor", "is_semiprime", |c| {
        c.check(Some(u8::from(is_semiprime(c.arg(0)))));
    });
}

#[test]
fn is_almost_prime_oracle() {
    each("factor", "is_almost_prime", |c| {
        c.check(Some(u8::from(is_almost_prime(c.arg(0), c.arg(1)))));
    });
}

#[test]
fn is_omega_prime_oracle() {
    each("factor", "is_omega_prime", |c| {
        c.check(Some(u8::from(is_omega_prime(c.arg(0), c.arg(1)))));
    });
}
