//! Bigint modular helpers agree with native APIs.
#![cfg(feature = "bigint")]
use math_prime_util::{self as native, bigint::modular::*};
use num_bigint::BigUint;
use proptest::prelude::*;
fn big(n: u64) -> BigUint {
    n.into()
}

proptest! {
    #[test]
    fn bigint_modhelpers_native(a: u64, b: u64, c: u64, n: u64, small in 0u64..300, k in 0u64..8, p in -20i64..20, q in -20i64..20) {
        let (x,y,z,m) = (big(a),big(b),big(c),big(n));
        prop_assert_eq!(submod(&x,&y,&m), native::submod(a,b,n).map(big));
        prop_assert_eq!(negmod(&x,&m), native::negmod(a,n).map(big));
        prop_assert_eq!(muladdmod(&x,&y,&z,&m), native::muladdmod(a,b,c,n).map(big));
        prop_assert_eq!(mulsubmod(&x,&y,&z,&m), native::mulsubmod(a,b,c,n).map(big));
        if n > 0 {
            let (u,v) = native::lucasuvmod(p,q,k,n);
            prop_assert_eq!(lucasuvmod(&p.into(),&q.into(),&big(k),&m), (big(u),big(v)));
        }
        let m = big(small);
        prop_assert_eq!(is_qr(&x,&m), native::is_qr(a,small));
        prop_assert_eq!(qnr(&m), native::qnr(small).map(big));
        prop_assert_eq!(pisano_period(&m), BigUint::from(native::pisano_period(small)));
        prop_assert_eq!(allrootmod(&x,&big(k),&m), native::allrootmod(a,k,small).into_iter().map(big).collect::<Vec<_>>());
        prop_assert_eq!(cornacchia(&big(b % 20), &m), native::cornacchia(b % 20,small).map(|(x,y)| (big(x),big(y))));
        let other = b % 300;
        prop_assert_eq!(chinese2(&[(x,m),(big(c),big(other))]), native::chinese2(&[(a,small),(c,other)]).map(|(x,y)| (big(x),big(y))));
    }
}

#[test]
fn bigint_modhelpers_edges_and_lifting() {
    assert_eq!(chinese2(&[]), Some((big(0), big(0))));
    for n in 0..150 {
        for a in 0..n + 2 {
            assert_eq!(is_qr(&big(a), &big(n)), native::is_qr(a, n));
            for k in [0, 1, 2, 3, 5, 6, 7] {
                assert_eq!(
                    allrootmod(&big(a), &big(k), &big(n)),
                    native::allrootmod(a, k, n)
                        .into_iter()
                        .map(big)
                        .collect::<Vec<_>>(),
                    "a={a} k={k} n={n}"
                );
            }
        }
        assert_eq!(qnr(&big(n)), native::qnr(n).map(big));
        assert_eq!(
            pisano_period(&big(n)),
            BigUint::from(native::pisano_period(n))
        );
    }
    for d in 0..20 {
        for n in 0..150 {
            assert_eq!(
                cornacchia(&big(d), &big(n)),
                native::cornacchia(d, n).map(|(x, y)| (big(x), big(y)))
            );
        }
    }
    for p in [307, 401, 1009] {
        assert_eq!(
            pisano_period(&big(p)),
            BigUint::from(native::pisano_period(p))
        );
    }
    for n in [0, 1, 7] {
        assert_eq!(
            submod(&big(3), &big(9), &big(n)),
            native::submod(3, 9, n).map(big)
        );
        assert_eq!(
            muladdmod(&big(3), &big(9), &big(4), &big(n)),
            native::muladdmod(3, 9, 4, n).map(big)
        );
        assert_eq!(
            mulsubmod(&big(3), &big(9), &big(4), &big(n)),
            native::mulsubmod(3, 9, 4, n).map(big)
        );
        assert_eq!(negmod(&big(0), &big(n)), native::negmod(0, n).map(big));
    }
}

#[test]
#[should_panic(expected = "modulus n must be > 0")]
fn bigint_modhelpers_zero_lucas() {
    lucasuvmod(&1.into(), &1.into(), &big(1), &big(0));
}

#[test]
#[should_panic(expected = "Maximum returned roots exceeded")]
fn bigint_modhelpers_output_limit() {
    allrootmod(&big(1), &big(0), &big(600_000_001));
}

#[test]
fn bigint_modhelpers_full_width_smooth_moduli() {
    for shift in 0..64 {
        let n = 1u64 << shift;
        let m = big(n);
        assert_eq!(pisano_period(&m), BigUint::from(native::pisano_period(n)));
        assert_eq!(qnr(&m), native::qnr(n).map(big));
        for a in [0, 1, 2, 4, 9, u64::MAX] {
            assert_eq!(is_qr(&big(a), &m), native::is_qr(a, n));
            for k in [1, 2, 3, 5] {
                if a == 0 && k > 1 {
                    continue;
                }
                assert_eq!(
                    allrootmod(&big(a), &big(k), &m),
                    native::allrootmod(a, k, n)
                        .into_iter()
                        .map(big)
                        .collect::<Vec<_>>()
                );
            }
        }
        assert_eq!(
            cornacchia(&m, &m),
            native::cornacchia(n, n).map(|(x, y)| (big(x), big(y)))
        );
    }
}
