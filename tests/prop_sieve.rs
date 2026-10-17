//! Algebraic laws for the sieve: succession, sums, clusters, and the cache.

use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 64,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn next_prev_roundtrip_on_primes(n in 2u64..1_500u64) {
        let ps = primes(n, n + 80);
        for &p in &ps {
            let nxt = next_prime(p).unwrap();
            prop_assert!(nxt > p);
            prop_assert_eq!(prev_prime(nxt), Some(p));
            if p > 2 {
                prop_assert_eq!(next_prime(prev_prime(p).unwrap()), Some(p));
            }
            if nxt > p + 1 {
                prop_assert!(primes(p + 1, nxt - 1).is_empty());
            }
        }
    }

    #[test]
    fn sum_equals_the_list(lo in 0u64..400, hi in 0u64..400) {
        let ps = primes(lo, hi);
        let s: u64 = ps.iter().copied().sum();
        prop_assert_eq!(sum_primes(lo, hi), Some(s));
    }

    #[test]
    fn range_keeps_only_rough_numbers(start in 0u64..200, width in 0u64..40, depth in 0u64..30) {
        let offs = sieve_range(start, width, depth);
        prop_assert!(offs.len() <= usize::try_from(width).unwrap_or(0));
        let mut prev = None;
        for o in offs {
            prop_assert!(o < width);
            if let Some(p) = prev {
                prop_assert!(o > p);
            }
            prev = Some(o);
            let n = start + o;
            prop_assert!(n >= 2);
            if depth >= 2 {
                for p in primes(2, depth.min(n)) {
                    if p < n {
                        prop_assert!(n % p != 0 || p > depth, "n={n} p={p}");
                    }
                }
            }
        }
    }
}

#[test]
fn cluster_members_are_prime() {
    let found = sieve_prime_cluster(0, 400, &[0, 2, 6]);
    assert!(!found.is_empty());
    for p in found {
        assert_eq!(primes(p, p), vec![p]);
        assert_eq!(primes(p + 2, p + 2), vec![p + 2]);
        assert_eq!(primes(p + 6, p + 6), vec![p + 6]);
    }
}

#[test]
fn precalc_does_not_change_values_under_threads() {
    let before = primes(100, 180);
    prime_precalc(20_000);
    let mut joins = Vec::new();
    for k in 0..6u64 {
        let expect = before.clone();
        joins.push(std::thread::spawn(move || {
            if k % 3 == 0 {
                prime_precalc(5_000 + k);
            } else if k % 3 == 1 {
                prime_memfree();
            }
            assert_eq!(primes(100, 180), expect);
            assert_eq!(next_prime(180), Some(181));
            assert_eq!(sum_primes(100, 120), sum_primes(100, 120));
        }));
    }
    for join in joins {
        join.join().unwrap();
    }
    prime_memfree();
    assert_eq!(primes(100, 180), before);
}
