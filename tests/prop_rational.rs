//! Algebraic laws for rational operations.
use math_prime_util::*;
use proptest::prelude::*;
proptest! {
    #[test]
    fn continued_fraction_roundtrip(a in -1_000_000i128..1_000_000, b in 1i128..1_000_000) {
        let terms = contfrac(a,b);
        let (p,q) = from_contfrac(&terms);
        prop_assert_eq!(p*b,a*q);
        prop_assert_eq!(convergents(&terms).last(),Some(&(p,q)));
    }
    #[test]
    fn tree_roundtrip(n in 1u128..1_000_000) {
        let (a,b)=nth_calkin_wilf(n);
        prop_assert_eq!(calkin_wilf_n(a,b),n);
        prop_assert_eq!(next_calkin_wilf(a,b),nth_calkin_wilf(n+1));
        prop_assert_eq!(nth_stern_diatomic(n),a);
        let (a,b)=nth_stern_brocot(n);
        prop_assert_eq!(stern_brocot_n(a,b),n);
        prop_assert_eq!(next_stern_brocot(a,b),nth_stern_brocot(n+1));
    }
    #[test]
    fn farey_neighbors(n in 1u64..40) {
        let sequence=farey(n);
        for (i, &(a,b)) in sequence.iter().enumerate() {
            prop_assert_eq!(farey_rank(n,a,b),i as u128);
            prop_assert_eq!(next_farey(n,a,b),sequence.get(i+1).copied());
            if let Some(&(c,d))=sequence.get(i+1) { prop_assert_eq!(b*c-a*d,1); }
        }
    }
}
#[test]
fn invalid_inputs_and_repeat_calls() {
    assert!(std::panic::catch_unwind(|| contfrac(1, 0)).is_err());
    assert!(std::panic::catch_unwind(|| from_contfrac(&[1, 0])).is_err());
    assert!(std::panic::catch_unwind(|| next_calkin_wilf(2, 4)).is_err());
    assert!(std::panic::catch_unwind(|| nth_stern_brocot(0)).is_err());
    assert!(std::panic::catch_unwind(|| bestrational(f64::NAN, 10)).is_err());
    let expected = farey(12);
    std::thread::scope(|s| {
        for _ in 0..4 {
            let expected = &expected;
            s.spawn(move || assert_eq!(&farey(12), expected));
        }
    });
    assert_eq!(farey(12), expected);
}
