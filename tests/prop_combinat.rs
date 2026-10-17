//! Algebraic checks for combinat functions.
use math_prime_util::{
    consecutive_integer_lcm, frobenius_number, gcd, is_congruent_number, is_polygonal, lcm,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn lcm_recurrence(n in 1u64..=47) {
        prop_assert_eq!(consecutive_integer_lcm(n), consecutive_integer_lcm(n-1).and_then(|v| lcm(v,n)));
    }

    #[test]
    fn polygonal_construction(k in 3u64..=10000, r in 0u64..=10000) {
        let n = if r == 0 { 0 } else { r * ((k-2)*(r-1)+2)/2 };
        prop_assert!(is_polygonal(n,k));
    }

    #[test]
    fn frobenius_pair(a in 2u64..100, b in 2u64..100) {
        if gcd(a,b) == 1 {
            prop_assert_eq!(frobenius_number(&[a,b]),Some(i128::from(a*b-a-b)));
        }
    }

    #[test]
    fn frobenius_redundant_and_permuted(a in 2u64..50, b in 2u64..50) {
        if gcd(a,b) == 1 {
            prop_assert_eq!(frobenius_number(&[a,b,a+b,a]),frobenius_number(&[b,a]));
        }
    }

    #[test]
    fn frobenius_last_gap(a in 2u64..20, b in 2u64..20, c in 2u64..20) {
        if gcd(gcd(a,b),c) == 1 {
            let mut reachable = [false; 400];
            reachable[0] = true;
            for i in 1..reachable.len() {
                reachable[i] = [a,b,c].into_iter().any(|v| {
                    let v = usize::try_from(v).unwrap();
                    i >= v && reachable[i-v]
                });
            }
            let last = reachable.iter().rposition(|&r| !r).unwrap();
            prop_assert_eq!(frobenius_number(&[a,b,c]), Some(i128::try_from(last).unwrap()));
        }
    }

    #[test]
    fn congruent_square_scaling(n in 0u64..500, s in 1u64..20) {
        prop_assert_eq!(is_congruent_number(n*s*s),is_congruent_number(n));
    }
}

#[test]
fn boundaries_and_repeat_calls() {
    assert_eq!(consecutive_integer_lcm(0), Some(1));
    assert_eq!(frobenius_number(&[]), None);
    assert_eq!(frobenius_number(&[7]), None);
    assert_eq!(frobenius_number(&[1]), Some(-1));
    assert_eq!(consecutive_integer_lcm(u64::MAX), None);
    assert_eq!(frobenius_number(&[u64::MAX, u64::MAX - 1]), None);
    assert_eq!(frobenius_number(&[4, 8, 7_000_000_000_000_000_001]), None);
    for _ in 0..2 {
        assert_eq!(frobenius_number(&[6, 9, 20]), Some(43));
        assert!(is_congruent_number(13706));
        assert!(!is_polygonal(14, 3));
    }
}

#[test]
#[should_panic(expected = "coprime")]
fn frobenius_non_coprime() {
    let _ = frobenius_number(&[2, 4]);
}

#[test]
#[should_panic(expected = "positive")]
fn frobenius_zero() {
    let _ = frobenius_number(&[0, 5]);
}

#[test]
#[should_panic(expected = "root < 3")]
fn polygonal_invalid_sides() {
    let _ = is_polygonal(0, 2);
}

#[test]
fn concurrent_calls() {
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                assert_eq!(frobenius_number(&[6, 9, 20]), Some(43));
                assert!(is_congruent_number(34));
                assert!(is_polygonal(36, 4));
                assert_eq!(consecutive_integer_lcm(10), Some(2520));
            });
        }
    });
}
