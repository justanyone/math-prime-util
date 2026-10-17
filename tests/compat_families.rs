//! Perfect-power compatibility with the committed MPU 0.75 oracle.
mod common;

use common::each;
use math_prime_util::{
    is_perfect_power, next_perfect_power, perfect_power_count, prev_perfect_power,
};

#[test]
fn is_perfect_power_oracle() {
    each("families", "is_perfect_power", |c| {
        // Skip signed inputs (including -8922003266371364726) and bigint
        // input 18446744073709551616: the native API takes u64.
        if let Ok(n) = c.args[0].parse::<u64>() {
            c.check(Some(u8::from(is_perfect_power(n))));
        }
    });
}

#[test]
fn next_perfect_power_oracle() {
    each("families", "next_perfect_power", |c| {
        // Skip signed inputs and bigint input 18446744073709551616.
        if let Ok(n) = c.args[0].parse::<u64>() {
            // Bigint results 18446744073709551616 become native overflow.
            let expected = c.result.as_ref().and_then(|s| s.parse::<u64>().ok());
            assert_eq!(next_perfect_power(n), expected, "{c:?}");
        }
    });
}

#[test]
fn prev_perfect_power_oracle() {
    each("families", "prev_perfect_power", |c| {
        // Skip signed inputs and bigint input 18446744073709551616.
        if let Ok(n) = c.args[0].parse::<u64>() {
            // The signed result -1 at zero has no unsigned representation.
            let expected = c.result.as_ref().and_then(|s| s.parse::<u64>().ok());
            assert_eq!(prev_perfect_power(n), expected, "{c:?}");
        }
    });
}

#[test]
fn perfect_power_count_oracle() {
    each("families", "perfect_power_count", |c| {
        let n: u64 = c.arg(0);
        let count = if c.args.len() == 1 {
            perfect_power_count(n)
        } else {
            let hi = c.arg(1);
            if hi < n {
                0
            } else {
                perfect_power_count(hi) - n.checked_sub(1).map_or(0, perfect_power_count)
            }
        };
        c.check(Some(count));
    });
}
