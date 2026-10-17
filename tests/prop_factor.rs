//! Algebraic laws and boundary regressions for the factor predicates.
use math_prime_util::{is_practical, is_rough, is_smooth, smooth_count};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn smooth_count_difference(n in 1u64..2000, k in 0u64..50) {
        prop_assert_eq!(smooth_count(n, k) - smooth_count(n - 1, k),
            u64::from(is_smooth(n, k)));
    }

    #[test]
    fn smooth_and_rough_product(a in 1u64..10_000, b in 1u64..10_000, k in 0u64..100) {
        prop_assert_eq!(is_smooth(a * b, k), is_smooth(a, k) && is_smooth(b, k));
        prop_assert_eq!(is_rough(a * b, k), is_rough(a, k) && is_rough(b, k));
    }

    #[test]
    fn bound_monotonicity(n in any::<u64>(), k in 0u64..5000) {
        prop_assert!(!is_smooth(n, k) || is_smooth(n, k + 1));
        prop_assert!(!is_rough(n, k + 1) || is_rough(n, k));
    }

    #[test]
    fn practical_doubling(n in 1u64..1_000_000) {
        prop_assert!(!is_practical(n) || is_practical(2 * n));
    }
}

#[test]
fn practical_matches_subset_sums() {
    for n in 1..=256 {
        let mut sums = vec![false; usize::try_from(n + 1).unwrap()];
        sums[0] = true;
        for d in 1..=n {
            if n % d == 0 {
                for j in (d..=n).rev() {
                    let j = usize::try_from(j).unwrap();
                    sums[j] |= sums[j - usize::try_from(d).unwrap()];
                }
            }
        }
        assert_eq!(is_practical(n), sums.into_iter().all(|b| b), "n={n}");
    }
}

#[test]
fn native_boundaries_and_repeated_calls() {
    for k in [0, 1, 2, 3, u64::MAX] {
        assert!(is_smooth(0, k));
        assert!(is_smooth(1, k));
        assert!(is_rough(1, k));
        assert_eq!(is_rough(0, k), k == 0);
        assert_eq!(smooth_count(0, k), 0);
        assert_eq!(smooth_count(1, k), 1);
    }
    for exponent in 0..64 {
        let n = 1u64 << exponent;
        assert!(is_practical(n));
        assert!(is_smooth(n, 2));
    }
    // The divisor sum of this practical number exceeds u64::MAX.
    assert!(is_practical(3u64 << 62));
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                for _ in 0..4 {
                    assert_eq!(smooth_count(100, 17), 67);
                    assert!(is_practical(120));
                    assert!(is_smooth(1_000_003 * 1_000_033, 1_000_033));
                    assert!(is_rough(1_000_003 * 1_000_033, 1_000_003));
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}
