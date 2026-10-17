//! Lucky-number compatibility with the committed MPU 0.75 oracle.
mod common;
use common::each;
use math_prime_util::*;

#[test]
fn lucky_numbers_oracle() {
    each("families", "lucky_numbers", |c| {
        let (lo, hi) = if c.args.len() == 1 {
            (0, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        let values = lucky_numbers(lo, hi);
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
fn is_lucky_oracle() {
    each("families", "is_lucky", |c| {
        c.check(Some(u8::from(is_lucky(c.arg(0)))));
    });
}
#[test]
fn lucky_count_oracle() {
    each("families", "lucky_count", |c| {
        let (lo, hi): (u64, u64) = if c.args.len() == 1 {
            (0, c.arg(0))
        } else {
            (c.arg(0), c.arg(1))
        };
        let count = if lo > hi {
            0
        } else {
            lucky_count(hi) - if lo == 0 { 0 } else { lucky_count(lo - 1) }
        };
        c.check(Some(count));
    });
}
macro_rules! oracle {
    ($name:ident) => {
        #[test]
        fn $name() {
            each("families", stringify!($name), |c| {
                c.check(math_prime_util::$name(c.arg(0)));
            });
        }
    };
}
oracle!(nth_lucky);
oracle!(nth_lucky_approx);
oracle!(nth_lucky_lower);
oracle!(nth_lucky_upper);
#[test]
fn lucky_count_approx_oracle() {
    each("families", "lucky_count_approx", |c| {
        c.check(Some(lucky_count_approx(c.arg(0))));
    });
}
#[test]
fn lucky_count_lower_oracle() {
    each("families", "lucky_count_lower", |c| {
        c.check(Some(lucky_count_lower(c.arg(0))));
    });
}
#[test]
fn lucky_count_upper_oracle() {
    each("families", "lucky_count_upper", |c| {
        c.check(Some(lucky_count_upper(c.arg(0))));
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
