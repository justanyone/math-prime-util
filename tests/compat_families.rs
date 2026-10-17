//! MPU 0.75 oracle cases for the assigned perfect-power functions.
mod common;

use common::each;
use math_prime_util::{
    nth_perfect_power, perfect_power_count_approx, perfect_power_count_lower,
    perfect_power_count_upper,
};

#[test]
fn perfect_power_count_approx_oracle() {
    each("families", "perfect_power_count_approx", |c| {
        c.check(Some(perfect_power_count_approx(c.arg(0))));
    });
}

#[test]
fn perfect_power_count_lower_oracle() {
    each("families", "perfect_power_count_lower", |c| {
        c.check(Some(perfect_power_count_lower(c.arg(0))));
    });
}

#[test]
fn perfect_power_count_upper_oracle() {
    each("families", "perfect_power_count_upper", |c| {
        c.check(Some(perfect_power_count_upper(c.arg(0))));
    });
}

#[test]
fn nth_perfect_power_oracle() {
    each("families", "nth_perfect_power", |c| {
        c.check(nth_perfect_power(c.arg(0)));
    });
}
