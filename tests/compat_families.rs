//! Prime power counts against the committed MPU oracle.

mod common;

use math_prime_util::*;

#[test]
fn prime_power_count_oracle() {
    common::each("families", "prime_power_count", |c| {
        // MPU's two-argument prime_power_count range overload has no Rust API yet.
        if c.args.len() != 1 { return; }
        c.check(Some(prime_power_count(c.arg(0))));
    });
}

#[test]
fn prime_power_count_approx_oracle() {
    common::each("families", "prime_power_count_approx", |c| {
        // MPU's two-argument prime_power_count range overload has no Rust API yet.
        if c.args.len() != 1 { return; }
        c.check(Some(prime_power_count_approx(c.arg(0))));
    });
}

#[test]
fn prime_power_count_lower_oracle() {
    common::each("families", "prime_power_count_lower", |c| {
        // MPU's two-argument prime_power_count range overload has no Rust API yet.
        if c.args.len() != 1 { return; }
        c.check(Some(prime_power_count_lower(c.arg(0))));
    });
}

#[test]
fn prime_power_count_upper_oracle() {
    common::each("families", "prime_power_count_upper", |c| {
        // MPU's two-argument prime_power_count range overload has no Rust API yet.
        if c.args.len() != 1 { return; }
        c.check(Some(prime_power_count_upper(c.arg(0))));
    });
}
