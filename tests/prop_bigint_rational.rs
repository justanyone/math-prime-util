//! BigUint and native rational agreement and round trips.
#![cfg(feature = "bigint")]
use math_prime_util::{bigint::rational as b, rational as n};
use num_bigint::BigUint;
use proptest::prelude::*;

fn pair(p: (u64, u64)) -> (BigUint, BigUint) {
    (p.0.into(), p.1.into())
}

proptest! {
    #[test]
    fn native_agreement(num in any::<u64>(), den in 1..=u64::MAX, index in 1..=u64::MAX, order in 1u32..1000, p in 0u32..100, q in 1u32..100) {
        let cf = n::contfrac(num, den);
        let big_cf: Vec<BigUint> = cf.iter().copied().map(BigUint::from).collect();
        prop_assert_eq!(b::contfrac(&num.into(), &den.into()), big_cf.clone());
        if let Some(v) = n::convergents(&cf) {
            prop_assert_eq!(b::convergents(&big_cf), Some(v.into_iter().map(pair).collect()));
        }
        prop_assert_eq!(b::from_contfrac(&big_cf), n::from_contfrac(&cf).map(pair));
        prop_assert_eq!(b::nth_calkin_wilf(&index.into()), pair(n::nth_calkin_wilf(index)));
        prop_assert_eq!(b::nth_stern_brocot(&index.into()), pair(n::nth_stern_brocot(index)));
        prop_assert_eq!(b::nth_stern_diatomic(&index.into()), BigUint::from(n::nth_stern_diatomic(index)));
        let (a,d) = n::nth_calkin_wilf(index);
        prop_assert_eq!(b::calkin_wilf_n(&a.into(), &d.into()), Some(index.into()));
        prop_assert_eq!(b::next_calkin_wilf(&a.into(), &d.into()), n::next_calkin_wilf(a,d).map(pair));
        let (a,d) = n::nth_stern_brocot(index);
        prop_assert_eq!(b::stern_brocot_n(&a.into(), &d.into()), Some(index.into()));
        if let Some(v) = n::next_stern_brocot(a,d) {
            prop_assert_eq!(b::next_stern_brocot(&a.into(), &d.into()), Some(pair(v)));
        }
        prop_assert_eq!(b::next_farey(&order.into(), &p.into(), &q.into()), n::next_farey(order,p,q).map(|(a,d)| (a.into(),d.into())));
    }
}

#[test]
fn edges_and_validation() {
    let z = BigUint::ZERO;
    let one = BigUint::from(1u8);
    let two = BigUint::from(2u8);
    assert_eq!(b::convergents(&[]), None);
    assert_eq!(b::convergents(&[one.clone(), z.clone()]), None);
    assert_eq!(b::from_contfrac(&[]), Some((z.clone(), one.clone())));
    assert_eq!(b::nth_stern_diatomic(&z), z);
    assert_eq!(
        b::next_farey(&two, &z, &one),
        Some((one.clone(), two.clone()))
    );
    assert_eq!(b::next_farey(&z, &one, &two), None);
    assert_eq!(b::next_farey(&one, &one, &one), None);
    assert_eq!(b::next_farey(&one, &one, &two), None);
    assert_eq!(
        b::next_stern_brocot(&two, &one),
        Some((one.clone(), 3u8.into()))
    );
    assert!(std::panic::catch_unwind(|| b::contfrac(&one, &z)).is_err());
    assert!(std::panic::catch_unwind(|| b::from_contfrac(&[one.clone(), z.clone()])).is_err());
    assert!(std::panic::catch_unwind(|| b::nth_calkin_wilf(&z)).is_err());
    assert!(std::panic::catch_unwind(|| b::nth_stern_brocot(&z)).is_err());
    assert!(std::panic::catch_unwind(|| b::next_calkin_wilf(&two, &two)).is_err());
    assert!(std::panic::catch_unwind(|| b::next_stern_brocot(&z, &one)).is_err());
    assert!(std::panic::catch_unwind(|| b::calkin_wilf_n(&one, &z)).is_err());
    assert!(std::panic::catch_unwind(|| b::stern_brocot_n(&two, &two)).is_err());
    // The same inputs can be used twice: all functions leave caller state intact.
    let large = (one.clone() << 160usize) + 123u8;
    let fraction = b::nth_calkin_wilf(&large);
    assert_eq!(
        b::calkin_wilf_n(&fraction.0, &fraction.1),
        Some(large.clone())
    );
    assert_eq!(b::nth_calkin_wilf(&large), fraction);
}

#[test]
fn address_space_and_threads() {
    let one = BigUint::from(1u8);
    let too_many_bits = &one << usize::BITS as usize;
    assert_eq!(b::calkin_wilf_n(&one, &too_many_bits), None);
    assert_eq!(b::stern_brocot_n(&one, &too_many_bits), None);
    let handles: Vec<_> = (1u64..=4)
        .map(|offset| {
            std::thread::spawn(move || {
                let index = (BigUint::from(1u8) << 128usize) + offset;
                let (p, q) = b::nth_stern_brocot(&index);
                assert_eq!(b::stern_brocot_n(&p, &q), Some(index));
            })
        })
        .collect();
    for handle in handles {
        handle.join().expect("thread succeeds");
    }
}
