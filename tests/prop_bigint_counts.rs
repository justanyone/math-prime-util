//! The arbitrary precision counts, bounds, estimates, loops and
//! `is_polygonal` agree with the native path on the `u64` range, and keep
//! their order above it.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::counts;
use num_bigint::BigUint;
use proptest::prelude::*;
use std::ops::ControlFlow;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

const MAX_PRIME_IDX: u64 = 425_656_284_035_217_743;

/// Every value a loop visits, as `BigUint`.
macro_rules! values {
    ($loop:path, $range:expr) => {{
        let mut values = Vec::new();
        let _: ControlFlow<()> = $loop($range, |v| {
            values.push(BigUint::from(v));
            ControlFlow::Continue(())
        });
        values
    }};
}

proptest! {
    #[test]
    fn nth_prime_bounds_agree(n: u64, small in 0_u64..1_000_000, idx in MAX_PRIME_IDX - 1000..=MAX_PRIME_IDX + 1000) {
        for n in [n, small, idx] {
            let bn = big(n);
            let native = [
                math_prime_util::nth_prime_lower(n),
                math_prime_util::nth_prime_upper(n),
                math_prime_util::nth_prime_approx(n),
            ];
            let wide = [
                counts::nth_prime_lower(&bn),
                counts::nth_prime_upper(&bn),
                counts::nth_prime_approx(&bn),
            ];
            for (native, wide) in native.into_iter().zip(wide.clone()) {
                if n <= MAX_PRIME_IDX {
                    prop_assert_eq!(native.map(BigUint::from), wide);
                } else {
                    prop_assert_eq!(native, None);
                    prop_assert!(wide.is_some());
                }
            }
            if n > MAX_PRIME_IDX {
                // Just above the index the lower bound is clamped to the largest
                // 64-bit prime while the estimate can sit a little below it, so
                // only the order of the bounds is checked here.
                prop_assert!(wide[0] <= wide[1]);
                prop_assert!(wide[2].clone().expect("estimate") > big(u64::MAX / 2));
            }
        }
    }

    #[test]
    fn prime_counts_agree(n: u64, small in 0_u64..10_000_000) {
        for n in [n, small] {
            let bn = big(n);
            prop_assert_eq!(counts::prime_count_lower(&bn), big(math_prime_util::prime_count_lower(n)));
            prop_assert_eq!(counts::prime_count_upper(&bn), big(math_prime_util::prime_count_upper(n)));
            prop_assert_eq!(counts::prime_count_approx(&bn), big(math_prime_util::prime_count_approx(n)));
        }
    }

    #[test]
    fn nth_prime_agrees(n in 0_u64..20_000) {
        prop_assert_eq!(counts::nth_prime(&big(n)), math_prime_util::nth_prime(n).map(BigUint::from));
    }

    #[test]
    fn polygonal_agrees(n: u64, k in 3_u64..=u64::MAX, small_k in 3_u64..100, small_n in 0_u64..100_000) {
        for (n, k) in [(n, k), (n, small_k), (small_n, small_k), (small_n, k)] {
            prop_assert_eq!(counts::is_polygonal(&big(n), &big(k)), math_prime_util::is_polygonal(n, k), "{} {}", n, k);
        }
    }

    #[test]
    fn streams_agree(start: u64, small in 0_u64..100_000) {
        for start in [start, small] {
            let native: Vec<BigUint> = math_prime_util::prime_iterator(start).take(5).map(BigUint::from).collect();
            let wide: Vec<BigUint> = counts::prime_iterator(&big(start)).take(5).collect();
            prop_assert_eq!(&wide[..native.len()], &native[..]);
            prop_assert_eq!(wide.len(), 5);
            prop_assert!(wide.windows(2).all(|w| w[0] < w[1]));
        }
    }

    #[test]
    fn loops_agree(lo in 0_u64..1_000_000, width in 0_u64..2_000, high in u64::MAX - 3_000..=u64::MAX, high_width in 0_u64..300) {
        for (lo, hi) in [(lo, lo + width), (high, high.saturating_add(high_width))] {
            let range = big(lo)..=big(hi);
            prop_assert_eq!(values!(counts::forprimes, range.clone()), values!(math_prime_util::forprimes, lo..=hi));
            prop_assert_eq!(values!(counts::forcomposites, range.clone()), values!(math_prime_util::forcomposites, lo..=hi));
            prop_assert_eq!(values!(counts::foroddcomposites, range), values!(math_prime_util::foroddcomposites, lo..=hi));
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]
    #[test]
    fn counts_keep_their_order_above_the_native_range(bits in 65_u32..400, low: u64) {
        let x = (BigUint::from(1_u8) << bits) + low;
        let (lower, upper, approx) = (counts::prime_count_lower(&x), counts::prime_count_upper(&x), counts::prime_count_approx(&x));
        prop_assert!(lower <= approx && approx <= upper, "{} {} {}", lower, approx, upper);
        let next = &x + 1_u8;
        prop_assert!(counts::prime_count_lower(&next) >= lower);
        prop_assert!(counts::prime_count_upper(&next) >= upper);
        prop_assert!(counts::nth_prime_lower(&x) <= counts::nth_prime_upper(&x));
    }
}
