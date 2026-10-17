//! Algebraic laws for rational arithmetic and enumeration.
use math_prime_util::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn continued_fraction_round_trip(p in any::<u64>(), q in 1..=u64::MAX) {
        let terms = contfrac(p, q);
        let g = gcd(p, q);
        prop_assert_eq!(from_contfrac(&terms), Some((p / g, q / g)));
        prop_assert_eq!(convergents(&terms).unwrap().last().copied(), Some((p / g, q / g)));
    }

    #[test]
    fn tree_indices_and_successors(n in 1..u64::MAX) {
        let (p, q) = nth_calkin_wilf(n);
        prop_assert_eq!(calkin_wilf_n(p, q), Some(n));
        prop_assert_eq!(next_calkin_wilf(p, q), Some(nth_calkin_wilf(n + 1)));
        prop_assert_eq!(nth_stern_diatomic(n), p);
        prop_assert_eq!(nth_stern_diatomic(n + 1), q);
        let (p, q) = nth_stern_brocot(n);
        prop_assert_eq!(stern_brocot_n(p, q), Some(n));
        prop_assert_eq!(next_stern_brocot(p, q), Some(nth_stern_brocot(n + 1)));
    }

    #[test]
    fn farey_neighbors_and_ranks(n in 1u32..80) {
        let terms = farey(n);
        prop_assert_eq!(terms.len() as u64, farey_len(n));
        for (k, &(p, q)) in terms.iter().enumerate() {
            prop_assert_eq!(farey_rank(n, p, q), k as u64);
            prop_assert_eq!(farey_nth(n, k as u64), Some((p, q)));
            prop_assert_eq!(next_farey(n, p, q), terms.get(k + 1).copied());
            if let Some(&(r, s)) = terms.get(k + 1) {
                prop_assert_eq!(u64::from(r) * u64::from(q) - u64::from(p) * u64::from(s), 1);
            }
        }
        prop_assert_eq!(farey_nth(n, terms.len() as u64), None);
    }
}

#[test]
fn native_overflow_and_validation() {
    assert_eq!(calkin_wilf_n(1u64 << 32, 1), None);
    assert_eq!(stern_brocot_n(u64::MAX, 1), None);
    assert_eq!(from_contfrac(&[u64::MAX, 2]), None);
    assert_eq!(convergents(&[1, 0]), None);
    assert_eq!(bestrational(f64::NAN, 10), None);
    assert!(std::panic::catch_unwind(|| bestrational(1.0, 0)).is_err());
    assert_eq!(next_farey(2, 1, 3), None);
    assert!(std::panic::catch_unwind(|| next_calkin_wilf(2, 4)).is_err());
    assert!(std::panic::catch_unwind(|| next_stern_brocot(0, 1)).is_err());
    assert!(std::panic::catch_unwind(|| contfrac(1, 0)).is_err());
}

#[test]
fn repeated_and_concurrent_calls_are_independent() {
    let expected = farey(40);
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    assert_eq!(farey(40), expected);
                    assert_eq!(farey(40), expected);
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
    });
}
