//! Native parity, arbitrary precision laws and repeated calls.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::sets as wide;
use num_bigint::{BigInt, BigUint};
use proptest::prelude::*;
fn big(a: &[i64]) -> Vec<BigUint> {
    a.iter().map(|x| BigUint::from(x.unsigned_abs())).collect()
}
fn signed(a: &[i64]) -> Vec<BigInt> {
    a.iter().copied().map(BigInt::from).collect()
}
proptest! {
    #[test]
    fn bigint_sets_native_parity(
        a in prop::collection::vec(0_i64..10000, 0..20),
        b in prop::collection::vec(0_i64..10000, 0..20),
    ) {
        let aa = big(&a);
        let bb = big(&b);
        prop_assert_eq!(wide::toset(&aa), big(&math_prime_util::toset(&a)));
        prop_assert_eq!(wide::vecuniq(&aa), big(&math_prime_util::vecuniq(&a)));
        prop_assert_eq!(wide::vecsingleton(&aa), big(&math_prime_util::vecsingleton(&a)));
        prop_assert_eq!(wide::vecsort(&aa), big(&math_prime_util::vecsort(&a)));
        prop_assert_eq!(wide::vecrsort(&aa), big(&math_prime_util::vecrsort(&a)));
        prop_assert_eq!(wide::setunion(&aa, &bb), big(&math_prime_util::setunion(&a, &b)));
        prop_assert_eq!(wide::setintersect(&aa, &bb), big(&math_prime_util::setintersect(&a, &b)));
        prop_assert_eq!(wide::setminus(&aa, &bb), big(&math_prime_util::setminus(&a, &b)));
        prop_assert_eq!(wide::setdelta(&aa, &bb), big(&math_prime_util::setdelta(&a, &b)));
        prop_assert_eq!(wide::sumset(&aa, &bb), big(&math_prime_util::sumset(&a, &b)));
        prop_assert_eq!(wide::setcontains(&aa, &bb), math_prime_util::setcontains(&a, &b));
        prop_assert_eq!(wide::setcontainsany(&aa, &bb), math_prime_util::setcontainsany(&a, &b));
        prop_assert_eq!(wide::set_is_disjoint(&aa, &bb), math_prime_util::set_is_disjoint(&a, &b));
        prop_assert_eq!(wide::set_is_equal(&aa, &bb), math_prime_util::set_is_equal(&a, &b));
        prop_assert_eq!(wide::set_is_subset(&aa, &bb), math_prime_util::set_is_subset(&a, &b));
        prop_assert_eq!(wide::set_is_superset(&aa, &bb), math_prime_util::set_is_superset(&a, &b));
        prop_assert_eq!(wide::set_is_proper_subset(&aa, &bb), math_prime_util::set_is_proper_subset(&a, &b));
        prop_assert_eq!(wide::set_is_proper_superset(&aa, &bb), math_prime_util::set_is_proper_superset(&a, &b));
        prop_assert_eq!(wide::set_is_proper_intersection(&aa, &bb), math_prime_util::set_is_proper_intersection(&a, &b));
        prop_assert_eq!(wide::is_sidon_set(&aa), math_prime_util::is_sidon_set(&a));
        prop_assert_eq!(wide::is_sumfree_set(&aa), math_prime_util::is_sumfree_set(&a));
        prop_assert_eq!(wide::vecmin(&aa),
            math_prime_util::vecmin(&a).map(|x| BigUint::from(x.unsigned_abs())));
        prop_assert_eq!(wide::vecmax(&aa),
            math_prime_util::vecmax(&a).map(|x| BigUint::from(x.unsigned_abs())));
        let mut x = a.clone(); let mut y = aa.clone();
        math_prime_util::vecsorti(&mut x); wide::vecsorti(&mut y);
        prop_assert_eq!(y, big(&x));
        let mut x = a.clone(); let mut y = aa.clone();
        math_prime_util::vecrsorti(&mut x); wide::vecrsorti(&mut y);
        prop_assert_eq!(y, big(&x));
        let x = a.clone();
        prop_assert_eq!(wide::vecmex(&big(&x)),
            BigUint::from(math_prime_util::vecmex(&x)));
        let x: Vec<_> = a.iter().copied().filter(|x| *x > 0).collect();
        prop_assert_eq!(wide::vecpmex(&big(&x)),
            BigUint::from(math_prime_util::vecpmex(&x)));
        let mut x = a.clone(); let mut y = aa.clone();
        prop_assert_eq!(wide::setinsert(&mut y, &bb), math_prime_util::setinsert(&mut x, &b));
        prop_assert_eq!(y, big(&x));
        let mut x = a.clone(); let mut y = aa.clone();
        prop_assert_eq!(wide::setremove(&mut y, &bb), math_prime_util::setremove(&mut x, &b));
        prop_assert_eq!(y, big(&x));
        let mut x = a.clone(); let mut y = aa.clone();
        prop_assert_eq!(wide::setinvert(&mut y, &bb), math_prime_util::setinvert(&mut x, &b));
        prop_assert_eq!(y, big(&x));
        prop_assert_eq!(wide::setbinop(&aa, &bb, |x, y| x + y),
            big(&math_prime_util::setbinop(&a, &b, |x, y| x + y)));
        let f: Vec<_> = math_prime_util::vecfreq(&a).into_iter()
            .map(|(x, n)| (BigUint::from(x.unsigned_abs()), n)).collect();
        prop_assert_eq!(wide::vecfreq(&aa), f);
    }
    #[test]
    fn full_u64_order(a in prop::collection::vec(any::<u64>(), 0..20)) {
        let aa: Vec<BigUint> = a.iter().copied().map(BigUint::from).collect();
        prop_assert_eq!(wide::vecmin(&aa), math_prime_util::vecmin(&a).map(BigUint::from));
        prop_assert_eq!(wide::vecmax(&aa), math_prime_util::vecmax(&a).map(BigUint::from));
    }
}
#[test]
#[should_panic(expected = "values must be positive")]
fn positive_mex_rejects_zero() {
    wide::vecpmex(&[BigUint::from(0_u8)]);
}
#[test]
fn signed_edges_and_repeated_calls() {
    let a = signed(&[-1, 0, 1, -1]);
    assert!(!wide::is_sidon_set(&a));
    assert!(!wide::is_sumfree_set(&a));
    assert_eq!(wide::vecuniq(&a), signed(&[-1, 0, 1]));
    assert_eq!(wide::vecsingleton(&a), signed(&[0, 1]));
    for _ in 0..2 {
        let mut a = signed(&[1, 2]);
        assert_eq!(wide::setinsert(&mut a, &signed(&[2, 3, 3])), 1);
        assert_eq!(wide::setinsert(&mut a, &signed(&[2, 3, 3])), 0);
        assert_eq!(wide::setremove(&mut a, &signed(&[3, 3])), 1);
        assert_eq!(wide::setremove(&mut a, &signed(&[3, 3])), 0);
        assert_eq!(wide::setinvert(&mut a, &signed(&[2, 3, 3])), 0);
        assert_eq!(a, signed(&[1, 3]));
    }
    let threads: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                let a = vec![BigUint::from(1_u8) << 100_usize];
                assert!(wide::is_sidon_set(&a));
                assert!(wide::is_sumfree_set(&a));
                assert_eq!(wide::sumset(&a, &a), vec![&a[0] * 2_u8]);
            })
        })
        .collect();
    for thread in threads {
        thread.join().expect("independent call");
    }
}
