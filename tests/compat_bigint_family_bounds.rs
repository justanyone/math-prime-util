//! Replay MPU 0.75 pure Perl family bounds above the native range.
#![cfg(feature = "bigint")]
mod common;
use math_prime_util::bigint::family_bounds;
use num_bigint::BigUint;

#[test]
fn bigint_family_bounds_oracle() {
    macro_rules! replay {
        ($($name:ident),+ $(,)?) => {
            $(common::each("bigint_family_bounds", stringify!($name), |case| {
                let n: BigUint = case.arg(0);
                case.check(Some(family_bounds::$name(&n)));
            });)+
        };
    }
    replay!(
        perfect_power_count_approx,
        perfect_power_count_lower,
        perfect_power_count_upper,
    );
}
