//! Compatibility with the committed MPU factor fixtures.
mod common;
use common::each;
use math_prime_util::{is_practical, is_rough, is_smooth, smooth_count};

#[test]
fn is_practical_oracle() {
    each("factor", "is_practical", |c| {
        // Signed is_practical arguments are outside the native u64 API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_practical(n))));
        }
    });
}

#[test]
fn is_smooth_oracle() {
    each("factor", "is_smooth", |c| {
        // Signed is_smooth arguments are outside the native u64 API.
        if let (Ok(n), Ok(k)) = (c.args[0].parse::<u64>(), c.args[1].parse::<u64>()) {
            c.check(Some(u8::from(is_smooth(n, k))));
        }
    });
}

#[test]
fn is_rough_oracle() {
    each("factor", "is_rough", |c| {
        // Signed is_rough arguments are outside the native u64 API.
        if let (Ok(n), Ok(k)) = (c.args[0].parse::<u64>(), c.args[1].parse::<u64>()) {
            c.check(Some(u8::from(is_rough(n, k))));
        }
    });
}

#[test]
fn smooth_count_oracle() {
    each("factor", "smooth_count", |c| {
        c.check(Some(smooth_count(c.arg(0), c.arg(1))));
    });
}
