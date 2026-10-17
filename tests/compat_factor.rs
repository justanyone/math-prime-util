//! Compatibility with the committed MPU factor fixtures.
mod common;
use common::each;
use math_prime_util::{
    is_carmichael, is_cyclic, is_perfect_number, is_practical, is_quasi_carmichael, is_rough,
    is_smooth, smooth_count,
};

// The u64 API skips signed arguments (including -0) and the bigint cases
// 18450931744652885087, 36803912137803496523, and 36832540935240707603.

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
fn is_carmichael_oracle() {
    each("factor", "is_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_carmichael(n))));
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
fn is_quasi_carmichael_oracle() {
    each("factor", "is_quasi_carmichael", |c| {
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(is_quasi_carmichael(n)));
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

#[test]
fn smooth_count_oracle() {
    each("factor", "smooth_count", |c| {
        c.check(Some(smooth_count(c.arg(0), c.arg(1))));
    });
}
