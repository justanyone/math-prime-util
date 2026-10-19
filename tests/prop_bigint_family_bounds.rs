//! Bigint family bounds equal the native ones on the `u64` range, and keep
//! their order above it.
#![cfg(feature = "bigint")]
use math_prime_util as mpu;
use math_prime_util::bigint::family_bounds as fb;
use num_bigint::BigUint;
use proptest::prelude::*;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

proptest! {
    #[test]
    fn counts_equal_native(n: u64, small in 0_u64..5000, k in 0_u32..40) {
        for n in [n, small] {
            let b = big(n);
            prop_assert_eq!(fb::perfect_power_count_approx(&b), big(mpu::perfect_power_count_approx(n)));
            prop_assert_eq!(fb::perfect_power_count_lower(&b), big(mpu::perfect_power_count_lower(n)));
            prop_assert_eq!(fb::perfect_power_count_upper(&b), big(mpu::perfect_power_count_upper(n)));
            prop_assert_eq!(fb::prime_power_count_approx(&b), big(mpu::prime_power_count_approx(n)));
            prop_assert_eq!(fb::prime_power_count_lower(&b), big(mpu::prime_power_count_lower(n)));
            prop_assert_eq!(fb::prime_power_count_upper(&b), big(mpu::prime_power_count_upper(n)));
            prop_assert_eq!(fb::semiprime_count_approx(&b), big(mpu::semiprime_count_approx(n)));
            prop_assert_eq!(fb::twin_prime_count_approx(&b), big(mpu::twin_prime_count_approx(n)));
            prop_assert_eq!(fb::lucky_count_approx(&b), big(mpu::lucky_count_approx(n)));
            prop_assert_eq!(fb::lucky_count_lower(&b), big(mpu::lucky_count_lower(n)));
            prop_assert_eq!(fb::lucky_count_upper(&b), big(mpu::lucky_count_upper(n)));
            prop_assert_eq!(fb::ramanujan_prime_count_approx(&b), big(mpu::ramanujan_prime_count_approx(n)));
            prop_assert_eq!(fb::ramanujan_prime_count_lower(&b), big(mpu::ramanujan_prime_count_lower(n)));
            prop_assert_eq!(fb::ramanujan_prime_count_upper(&b), big(mpu::ramanujan_prime_count_upper(n)));
            prop_assert_eq!(
                fb::almost_prime_count_approx(u64::from(k), &b),
                big(mpu::almost_prime_count_approx(u64::from(k), n))
            );
            prop_assert_eq!(
                fb::almost_prime_count_lower(u64::from(k), &b),
                big(mpu::almost_prime_count_lower(u64::from(k), n))
            );
            prop_assert_eq!(
                fb::almost_prime_count_upper(k, &b),
                big(mpu::almost_prime_count_upper(k, n))
            );
        }
    }

    // The index ranges stop where the native function clamps at the largest
    // 64-bit value; the BigUint form takes over there.
    #[test]
    fn nth_equal_native(n in 0_u64..(1 << 31), mid in 0_u64..(1 << 56), k in 0_u32..30, kn in 0_u64..100_000) {
        let b = big(n);
        let m = big(mid);
        macro_rules! same {
            ($f:ident, $arg:expr, $barg:expr) => {
                prop_assert_eq!(fb::$f(&$barg), mpu::$f($arg).map(big), "{}", stringify!($f));
            };
        }
        same!(nth_perfect_power_approx, n, b);
        same!(nth_perfect_power_lower, n, b);
        same!(nth_perfect_power_upper, n, b);
        same!(nth_prime_power_approx, mid, m);
        same!(nth_prime_power_lower, mid, m);
        same!(nth_prime_power_upper, mid, m);
        same!(nth_lucky_approx, mid, m);
        same!(nth_lucky_lower, mid, m);
        same!(nth_lucky_upper, mid, m);
        same!(nth_ramanujan_prime_approx, mid >> 2, big(mid >> 2));
        same!(nth_ramanujan_prime_lower, mid >> 2, big(mid >> 2));
        same!(nth_ramanujan_prime_upper, mid >> 2, big(mid >> 2));
        same!(nth_semiprime_approx, mid, m);
        same!(nth_twin_prime_approx, mid >> 4, big(mid >> 4));
        prop_assert_eq!(fb::nth_almost_prime_approx(k, &big(kn)), mpu::nth_almost_prime_approx(k, kn).map(big));
        prop_assert_eq!(fb::nth_almost_prime_lower(k, &big(kn)), mpu::nth_almost_prime_lower(k, kn).map(big));
        prop_assert_eq!(fb::nth_almost_prime_upper(k, &big(kn)), mpu::nth_almost_prime_upper(k, kn).map(big));
    }

    #[test]
    fn inverse_li_equals_native(n in 0_u64..(1 << 58)) {
        prop_assert_eq!(fb::inverse_li(&big(n)), big(mpu::inverse_li(n)));
    }

    // li(inverse_li(n) - 1) < n <= li(inverse_li(n)) in the order of the
    // answers above 2^64, and the bounds keep their order.
    #[test]
    fn bounds_keep_their_order(shift in 64_usize..200, low in 0_u32..1000) {
        let n = (BigUint::from(1_u8) << shift) + low;
        let lo = fb::nth_perfect_power_lower(&n).expect("a value");
        let mid = fb::nth_perfect_power_approx(&n).expect("a value");
        let hi = fb::nth_perfect_power_upper(&n).expect("a value");
        prop_assert!(lo <= mid && mid <= hi);
        let lo = fb::nth_ramanujan_prime_lower(&n).expect("a value");
        let mid = fb::nth_ramanujan_prime_approx(&n).expect("a value");
        let hi = fb::nth_ramanujan_prime_upper(&n).expect("a value");
        prop_assert!(lo <= mid && mid <= hi);
        let lo = fb::nth_lucky_lower(&n).expect("a value");
        let hi = fb::nth_lucky_upper(&n).expect("a value");
        prop_assert!(lo <= fb::nth_lucky_approx(&n).expect("a value") && lo <= hi);
        prop_assert!(fb::lucky_count_lower(&n) <= fb::lucky_count_approx(&n));
        prop_assert!(fb::lucky_count_approx(&n) <= fb::lucky_count_upper(&n));
        prop_assert!(fb::ramanujan_prime_count_lower(&n) <= fb::ramanujan_prime_count_upper(&n));
    }
}

#[test]
fn zero_one_and_small_powers() {
    for (n, expected) in [
        (0_u8, 0_u8),
        (1, 1),
        (3, 1),
        (4, 2),
        (7, 2),
        (8, 3),
        (16, 5),
    ] {
        assert_eq!(
            fb::perfect_power_count_approx(&BigUint::from(n)),
            BigUint::from(expected)
        );
    }
}

#[test]
fn square_boundary() {
    for bits in 33_usize..128 {
        let root = BigUint::from(1_u8) << bits;
        let square = &root * &root;
        let before = fb::perfect_power_count_approx(&(&square - 1_u8));
        let at = fb::perfect_power_count_approx(&square);
        assert_eq!(at, &before + 1_u8);
        assert_eq!(at, fb::perfect_power_count_lower(&square));
        assert_eq!(at, fb::perfect_power_count_upper(&square));
        assert_eq!(at, fb::perfect_power_count_approx(&(&square + 1_u8)));
    }
}
