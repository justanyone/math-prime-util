//! Ramanujan prime cases from the committed MPU oracle.
mod common;
use common::each;
use math_prime_util::*;

#[test]
fn nth_ramanujan_prime_oracle() {
    each("families", "nth_ramanujan_prime", |c| {
        c.check(nth_ramanujan_prime(c.arg(0)));
    });
}
#[test]
fn nth_ramanujan_prime_lower_oracle() {
    each("families", "nth_ramanujan_prime_lower", |c| {
        c.check(nth_ramanujan_prime_lower(c.arg(0)));
    });
}
#[test]
fn nth_ramanujan_prime_approx_oracle() {
    each("families", "nth_ramanujan_prime_approx", |c| {
        c.check(nth_ramanujan_prime_approx(c.arg(0)));
    });
}
#[test]
fn ramanujan_prime_count_upper_oracle() {
    each("families", "ramanujan_prime_count_upper", |c| {
        c.check(Some(ramanujan_prime_count_upper(c.arg(0))));
    });
}

#[test]
fn nth_prime_power_oracle() {
    each("families", "nth_prime_power", |c| {
        c.check(nth_prime_power(c.arg(0)));
    });
}
#[test]
fn nth_prime_power_approx_oracle() {
    each("families", "nth_prime_power_approx", |c| {
        c.check(nth_prime_power_approx(c.arg(0)));
    });
}
#[test]
fn nth_prime_power_lower_oracle() {
    each("families", "nth_prime_power_lower", |c| {
        c.check(nth_prime_power_lower(c.arg(0)));
    });
}
#[test]
fn nth_prime_power_upper_oracle() {
    each("families", "nth_prime_power_upper", |c| {
        c.check(nth_prime_power_upper(c.arg(0)));
    });
}
