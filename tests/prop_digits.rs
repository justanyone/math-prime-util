//! Algebraic laws and fixed width boundaries for digit helpers.
use math_prime_util::digits::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn digits_round_trip(n in any::<u64>(), base in 2_u64..=36) {
        let digits = todigits(i128::from(n), base, None);
        prop_assert_eq!(fromdigits(&digits, base), Some(n));
        prop_assert!(digits.iter().all(|&d| d < base));
        prop_assert_eq!(fromzeckendorf(&tozeckendorf(n)), Some(n));
        prop_assert_eq!(hammingweight(i128::from(n)), n.count_ones());
    }
    #[test]
    fn arithmetic_laws(a in any::<i64>(), b in any::<i64>(), c in any::<i64>()) {
        let (a,b,c) = (i128::from(a),i128::from(b),i128::from(c));
        prop_assert_eq!(subint(addint(a,b).unwrap(),b),Some(a));
        prop_assert_eq!(addint(a,b),addint(b,a));
        prop_assert_eq!(mulint(a,b),mulint(b,a));
        prop_assert_eq!(muladdint(a,b,c),Some(a*b+c));
        prop_assert_eq!(mulsubint(a,b,c),Some(a*b-c));
        prop_assert_eq!(negint(negint(a).unwrap()),Some(a));
        prop_assert_eq!(is_odd(a), !is_even(a));
        prop_assert!(is_congruent(a,a,b));
        if b != 0 {
            for operation in [divrem,tdivrem,fdivrem,cdivrem] {
                let (q,r)=operation(a,b).unwrap();
                prop_assert_eq!(q*b+r,a);
                prop_assert!(r.unsigned_abs() < b.unsigned_abs());
            }
            let (_,r)=divrem(a,b).unwrap();
            prop_assert!(r>=0);
            prop_assert_eq!(is_divisible(a,b),r==0);
        }
    }
    #[test]
    fn roots_and_reversal(n in 1_i64.., base in 2_u64..=36) {
        let n=i128::from(n);
        prop_assert!(digital_root(n,base)<u128::from(base));
        prop_assert!(mult_digital_root(n,base)<u128::from(base));
        prop_assert_eq!(is_palindrome(n,base),reverse_digits(n,base)==Some(n.unsigned_abs()));
    }
    #[test]
    fn shifts(n in any::<i64>(), k in 0_u32..=63) {
        let n=i128::from(n);
        let shifted=lshiftint(n,i128::from(k)).unwrap();
        prop_assert_eq!(rshiftint(shifted,i128::from(k)),Some(n));
        prop_assert_eq!(rashiftint(n,i128::from(k)),Some(n>>k));
    }
}

#[test]
fn overflow_and_extreme_signed_values() {
    assert_eq!(addint(i128::MAX, 1), None);
    assert_eq!(subint(i128::MIN, 1), None);
    assert_eq!(mulint(i128::MAX, 2), None);
    assert_eq!(negint(i128::MIN), None);
    assert_eq!(absint(i128::MIN), None);
    assert_eq!(divrem(i128::MIN, -1), None);
    assert_eq!(lshiftint(1, 127), None);
    assert_eq!(lshiftint(-1, 127), Some(i128::MIN));
    assert_eq!(rshiftint(i128::MIN, 0), Some(i128::MIN));
    assert_eq!(rashiftint(-1, 128), Some(-1));
    assert_eq!(rshiftint(-1, 128), Some(0));
    assert_eq!(lshiftint(0, i128::MAX), Some(0));
    assert_eq!(fromdigits(&[u64::MAX - 1, u64::MAX - 1], u64::MAX), None);
    assert_eq!(toint("170141183460469231731687303715884105728"), None);
    assert_eq!(
        toint("-170141183460469231731687303715884105728"),
        Some(i128::MIN)
    );
    assert!(is_divisible(0, 0));
    assert!(is_congruent(i128::MIN, 0, i128::MIN));
    assert_eq!(todigits(900, 2, Some(3)), vec![1, 0, 0]);
    assert_eq!(todigitstring(0, 10, None), "");
}

#[test]
fn input_validation() {
    assert!(std::panic::catch_unwind(|| todigits(1, 1, None)).is_err());
    assert!(std::panic::catch_unwind(|| fromdigits(&[2], 2)).is_err());
    assert!(std::panic::catch_unwind(|| divint(1, 0)).is_err());
    assert!(std::panic::catch_unwind(|| fromzeckendorf("11")).is_err());
    for invalid in ["1__0", "_1", "1_", "--1", "1e", "1e+", "1.2.3"] {
        assert!(std::panic::catch_unwind(|| toint(invalid)).is_err());
    }
}

#[test]
fn fused_cancellation_and_signed_division_edges() {
    assert_eq!(muladdint(i128::MAX, 2, -i128::MAX), Some(i128::MAX));
    assert_eq!(mulsubint(i128::MIN, -1, 1), Some(i128::MAX));
    assert_eq!(mulsubint(i128::MIN, 1, 0), Some(i128::MIN));
    assert_eq!(muladdint(i128::MAX, i128::MAX, i128::MIN), None);
    for (a, b) in [
        (i128::MIN, 1),
        (i128::MIN, i128::MAX),
        (i128::MAX, i128::MIN),
        (1, i128::MIN),
        (-1, i128::MIN),
    ] {
        for op in [divrem, tdivrem, fdivrem, cdivrem] {
            let (q, r) = op(a, b).unwrap();
            // Identity can need one extra bit in its intermediate product.
            assert_eq!(muladdint(q, b, r), Some(a));
            assert!(r.unsigned_abs() < b.unsigned_abs());
        }
    }
    assert_eq!(toint("1e-1000"), Some(0));
    assert_eq!(toint("1e1000"), None);
    assert_eq!(toint("0e1000"), Some(0));
}
