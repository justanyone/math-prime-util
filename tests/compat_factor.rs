//! MPU oracle cases for the eight factor predicates from MPU 0.75.
mod common;

use common::each;
use math_prime_util::{
    is_almost_prime, is_carmichael, is_cyclic, is_omega_prime, is_perfect_number,
    is_quasi_carmichael, is_semiprime, is_square_free,
};

// The u64 API skips signed arguments (including -0) and the bigint cases
// 18450931744652885087, 36803912137803496523, and 36832540935240707603.
#[test]
fn is_square_free_oracle() {
    each("factor", "is_square_free", |c| {
        // Skip bigint and signed arguments outside this native unsigned API.
        // Examples include n=18450931744652885087 and k=10^30.
        if c.args.iter().any(|arg| arg.parse::<u64>().is_err()) {
            return;
        }
        c.check(Some(u8::from(is_square_free(c.arg(0)))));
    });
}

#[test]
fn is_semiprime_oracle() {
    each("factor", "is_semiprime", |c| {
        // Skip bigint and signed arguments outside this native unsigned API.
        // Examples include n=18450931744652885087 and k=10^30.
        if c.args.iter().any(|arg| arg.parse::<u64>().is_err()) {
            return;
        }
        c.check(Some(u8::from(is_semiprime(c.arg(0)))));
    });
}

#[test]
fn is_almost_prime_oracle() {
    each("factor", "is_almost_prime", |c| {
        // Skip bigint and signed arguments outside this native unsigned API.
        // Examples include n=18450931744652885087 and k=10^30.
        if c.args.iter().any(|arg| arg.parse::<u64>().is_err()) {
            return;
        }
        c.check(Some(u8::from(is_almost_prime(c.arg(0), c.arg(1)))));
    });
}

#[test]
fn is_omega_prime_oracle() {
    each("factor", "is_omega_prime", |c| {
        // Skip bigint and signed arguments outside this native unsigned API.
        // Examples include n=18450931744652885087 and k=10^30.
        if c.args.iter().any(|arg| arg.parse::<u64>().is_err()) {
            return;
        }
        c.check(Some(u8::from(is_omega_prime(c.arg(0), c.arg(1)))));
    });
}

#[test]
fn is_carmichael_oracle() {
    each("factor", "is_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_carmichael(n))));
        }
    });
}

#[test]
fn is_quasi_carmichael_oracle() {
    each("factor", "is_quasi_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(is_quasi_carmichael(n)));
        }
    });
}

#[test]
fn is_cyclic_oracle() {
    each("factor", "is_cyclic", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_cyclic(n))));
        }
    });
}

#[test]
fn is_perfect_number_oracle() {
    each("factor", "is_perfect_number", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_perfect_number(n))));
        }
    });
}
