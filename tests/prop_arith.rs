//! Arithmetic identities and boundary regressions.
use math_prime_util::*;
use proptest::prelude::*;
proptest! {
    #[test]
    fn divisor_identities(n in 1_u64..10000) {
        let s = divisor_sum(n, 1).unwrap();
        prop_assert_eq!(aliquot_sum(n), Some(s - n));
        prop_assert_eq!(abundance(n), Some(i64::try_from(s).unwrap() - 2 * i64::try_from(n).unwrap()));
        prop_assert!(sopf(n) <= sopfr(n));
    }
    #[test]
    fn crt_reconstructs(a in 0_u64..10000, b in 0_u64..10000) {
        let (r, m) = chinese2(&[(a, 64), (b, 81)]).unwrap();
        prop_assert_eq!(m, 5184);
        prop_assert_eq!(r % 64, a % 64);
        prop_assert_eq!(r % 81, b % 81);
        prop_assert_eq!(chinese2(&[(b, 81), (a, 64)]), Some((r,m)));
    }
    #[test]
    fn squares_are_residues(a in 0_u64..10000, n in 1_u64..10000) {
        prop_assert!(is_qr(a*a, n));
    }
    #[test]
    fn order_annihilates(a in 0_u64..1000, n in 2_u64..1000) {
        if let Some(k) = znorder(a,n) { prop_assert_eq!(powmod(a,k,n), Some(1)); }
    }
    #[test]
    fn kronecker_multiplicative(a in -1000_i64..1000, b in -1000_i64..1000, n in -1000_i64..1000) {
        prop_assert_eq!(kronecker(a*b,n), kronecker(a,n)*kronecker(b,n));
    }
}
#[test]
fn boundaries_and_repeated_calls() {
    assert_eq!(divisor_sum(2, u64::MAX), None);
    assert_eq!(divisor_sum(0, u64::MAX), Some(0));
    assert_eq!(ramanujan_tau(u64::MAX), None);
    assert_eq!(kronecker(i64::MIN, i64::MIN), 0);
    assert_eq!(kronecker(-1, 0), 1);
    assert!(!is_fundamental(i64::MIN));
    assert_eq!(chinese2(&[]), Some((0, 0)));
    assert_eq!(chinese2(&[(0, 0)]), None);
    assert_eq!(chinese2(&[(0, 2), (1, 2)]), None);
    assert_eq!(chinese2(&[(0, u64::MAX), (0, 2)]), None);
    for _ in 0..2 {
        assert_eq!(chinese2(&[(5, 12), (17, 18), (5, 12)]), Some((17, 36)));
    }
    assert_eq!(znorder(1, 0), None);
    assert_eq!(znorder(1, 1), Some(1));
    for n in 1..200 {
        if let Some(a) = znprimroot(n) {
            assert_eq!(is_primitive_root(a, n), Some(true));
        }
    }
}
