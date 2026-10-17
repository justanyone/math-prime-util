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
fn minimal_goldbach_pair_oracle() {
    each("families", "minimal_goldbach_pair", |c| {
        // Skip minimal_goldbach_pair(83778272185315920949659591651127238812):
        // this bigint argument exceeds the native u64 API.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(minimal_goldbach_pair(n));
        }
    });
}

#[test]
fn goldbach_pair_count_oracle() {
    each("families", "goldbach_pair_count", |c| {
        c.check(Some(goldbach_pair_count(c.arg(0))));
    });
}

#[test]
fn goldbach_pairs_oracle() {
    each("families", "goldbach_pairs", |c| {
        let pairs = goldbach_pairs(c.arg(0));
        let text = pairs
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        c.check((!pairs.is_empty()).then_some(text));
    });
}

#[test]
fn next_chen_prime_oracle() {
    each("families", "next_chen_prime", |c| {
        c.check(next_chen_prime(c.arg(0)));
    });
}
