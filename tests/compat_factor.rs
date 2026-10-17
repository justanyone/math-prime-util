//! MPU factor fixtures for the four functions in this port.
mod common;

use common::each;
use math_prime_util::{cheb_factor, prime_bigomega, prime_omega, prime_signature};

fn list<T: ToString>(values: &[T]) -> Option<String> {
    (!values.is_empty()).then(|| {
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    })
}

#[test]
fn cheb_factor_oracle() {
    each("factor", "cheb_factor", |c| {
        // Negative and wider-than-u64 arguments have no native Rust API.
        let Ok(n) = c.args[0].parse::<u64>() else {
            return;
        };
        let bound = c.args.get(1).map_or(0, |s| s.parse().unwrap());
        let init_x = c.args.get(2).map_or(0, |s| s.parse().unwrap());
        c.check(list(&cheb_factor(n, bound, init_x)));
    });
}

#[test]
fn prime_bigomega_oracle() {
    each("factor", "prime_bigomega", |c| {
        // Negative and wider-than-u64 arguments have no native Rust API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(prime_bigomega(n)));
        }
    });
}

#[test]
fn prime_omega_oracle() {
    each("factor", "prime_omega", |c| {
        // Negative and wider-than-u64 arguments have no native Rust API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(prime_omega(n)));
        }
    });
}

#[test]
fn prime_signature_oracle() {
    each("factor", "prime_signature", |c| {
        // Negative and wider-than-u64 arguments have no native Rust API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(list(&prime_signature(n)));
        }
    });
}
