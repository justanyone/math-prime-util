//! Compare bigint sequences with native results and exact identities.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::sequences as s;
use num_bigint::{BigInt, BigUint};
use proptest::prelude::*;

fn big(n: u64) -> BigUint {
    n.into()
}

proptest! {
    #[test]
    fn bigint_sequences_native_agreement(
        n in 0_u64..120, k in 0_u64..12,
        p in -4_i64..5, q in -4_i64..5,
        a: u64, b: u64, m in 1_u64..=u64::MAX,
        values in prop::collection::vec(any::<u64>(), 0..3),
    ) {
        let bn = big(n);
        if let Some(v) = math_prime_util::fibonacci(n) {
            prop_assert_eq!(s::fibonacci(&bn), big(v));
        }
        if let Some(v) = math_prime_util::lucas_number(n) {
            prop_assert_eq!(s::lucas_number(&bn), big(v));
        }
        if let Some((u,v)) = math_prime_util::lucasuv(p,q,k) {
            let pair = s::lucasuv(&p.into(), &q.into(), &big(k));
            prop_assert_eq!(&pair, &(BigInt::from(u), BigInt::from(v)));
            prop_assert_eq!(s::lucasu(&p.into(), &q.into(), &big(k)), pair.0);
            prop_assert_eq!(s::lucasv(&p.into(), &q.into(), &big(k)), pair.1);
        }
        for (native, result) in [
            (math_prime_util::primorial(n), s::primorial(&bn)),
            (math_prime_util::pn_primorial(n), s::pn_primorial(&bn)),
            (math_prime_util::consecutive_integer_lcm(n), s::consecutive_integer_lcm(&bn)),
            (math_prime_util::powersum(n,k), s::powersum(&bn,&big(k))),
        ] {
            prop_assert_eq!(u64::try_from(result).ok(), native);
        }
        let bv: Vec<BigUint> = values.iter().copied().map(big).collect();
        prop_assert_eq!(s::vecsum(&bv), BigUint::from(math_prime_util::vecsum(&values)));
        prop_assert_eq!(s::vecprod(&bv), BigUint::from(math_prime_util::vecprod(&values)));
        let prefixes: Vec<BigUint> = math_prime_util::vecprefixsum(&values).into_iter().map(BigUint::from).collect();
        prop_assert_eq!(s::vecprefixsum(&bv), prefixes);
        prop_assert_eq!(u64::try_from(s::floor_sum(&bn,&big(m),&big(a),&big(b))).ok(), math_prime_util::floor_sum(n,m,a,b));
        let coins = [6, 9, 20 + 6*n];
        let bc: Vec<BigUint> = coins.iter().copied().map(big).collect();
        prop_assert_eq!(s::frobenius_number(&bc), math_prime_util::frobenius_number(&coins).map(BigInt::from));
    }

    #[test]
    fn bigint_sequences_wide_identities(n: u64, a: u64, b: u64, k in 0_u64..10, m in 1_u64..=u64::MAX) {
        let bn = big(n);
        prop_assert_eq!(u64::try_from(s::powersum(&bn, &big(k))).ok(), math_prime_util::powersum(n, k));
        prop_assert_eq!(u64::try_from(s::floor_sum(&bn, &big(m), &big(a), &big(b))).ok(), math_prime_util::floor_sum(n, m, a, b));
        let wide: BigUint = (big(a) << 80) + big(b);
        prop_assert_eq!(s::powersum(&bn, &big(0)), bn.clone());
        prop_assert_eq!(s::powersum(&bn, &big(1)), &bn * (&bn + 1_u8) / 2_u8);
        prop_assert_eq!(s::floor_sum(&bn, &big(1), &wide, &big(b)), &wide * &bn * (&bn - u8::from(n != 0)) / 2_u8 + &bn * b);
        let list = [wide.clone(), wide.clone(), big(0)];
        prop_assert_eq!(s::vecsum(&list), &wide * 2_u8);
        prop_assert_eq!(s::vecprod(&list), big(0));
        prop_assert_eq!(s::vecprefixsum(&list), vec![wide.clone(), &wide * 2_u8, &wide * 2_u8]);
    }
}

#[test]
fn bigint_sequences_boundaries_and_repeated_calls() {
    let huge: BigUint = big(1) << 100;
    assert_eq!(s::powersum(&big(2), &big(128)), (big(1) << 128) + 1_u8);
    assert_eq!(
        s::lucasuv(&1.into(), &0.into(), &huge),
        (1.into(), 1.into())
    );
    for _ in 0..2 {
        assert_eq!(s::vecsum(&[]), big(0));
        assert_eq!(s::vecprod(&[]), big(1));
        assert_eq!(s::vecprefixsum(&[]).len(), 0);
        assert_eq!(s::frobenius_number(&[]), None);
        assert_eq!(s::frobenius_number(&[big(7)]), None);
        assert_eq!(s::frobenius_number(&[big(1)]), Some((-1).into()));
        assert_eq!(
            s::frobenius_number(&[big(6), big(9), big(20), big(20)]),
            Some(43.into())
        );
        assert_eq!(
            s::frobenius_number(&[big(6), big(10), big(15)]),
            Some(29.into())
        );
        assert_eq!(s::floor_sum(&big(0), &big(1), &huge, &huge), big(0));
        assert_eq!(s::powersum(&big(1), &huge), big(1));
    }
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (
                    s::fibonacci(&big(100)),
                    s::frobenius_number(&[big(6), big(9), big(20)]),
                )
            })
        })
        .collect();
    let expected = (s::fibonacci(&big(100)), Some(43.into()));
    for handle in handles {
        assert_eq!(handle.join().unwrap(), expected);
    }
}

#[test]
#[should_panic(expected = "divide by zero")]
fn bigint_sequences_zero_modulus() {
    let _ = s::floor_sum(&big(0), &big(0), &big(0), &big(0));
}

#[test]
#[should_panic(expected = "positive")]
fn bigint_sequences_zero_coin() {
    let _ = s::frobenius_number(&[big(0), big(3)]);
}

#[test]
#[should_panic(expected = "coprime")]
fn bigint_sequences_noncoprime_coins() {
    let _ = s::frobenius_number(&[big(6), big(9)]);
}

#[test]
#[should_panic(expected = "table is too large")]
fn bigint_sequences_unrepresentable_table() {
    let n: BigUint = big(1) << 200;
    let _ = s::frobenius_number(&[n.clone(), &n + 1_u8, &n + 2_u8]);
}
