//! `BigUint` integer functions agree with the native functions.
#![cfg(feature = "bigint")]
use math_prime_util::{self as native, bigint::functions::*};
use num_bigint::{BigInt, BigUint};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(40))]
    #[test]
    fn native_agreement(n: u64, small in 0_u64..500, k in 1_u32..8) {
        for n in [n, small] {
            let b = BigUint::from(n);
            prop_assert_eq!(crootint(&b, k), BigUint::from(native::crootint(n, k)));
            prop_assert_eq!(exp_mangoldt(&b).unwrap(), BigUint::from(native::exp_mangoldt(n)));
            prop_assert_eq!(liouville(&b).unwrap(), native::liouville(n));
            prop_assert_eq!(prime_signature(&b).unwrap(), native::prime_signature(n));
            prop_assert_eq!(powerfree_part(&b, k).unwrap(), BigUint::from(native::powerfree_part(n, k)));
            prop_assert_eq!(squarefree_kernel(&b).unwrap(), BigUint::from(native::squarefree_kernel(n)));
            prop_assert_eq!(sopf(&b).unwrap(), BigUint::from(native::sopf(n)));
            prop_assert_eq!(sopfr(&b).unwrap(), BigUint::from(native::sopfr(n)));
            if let Some(v) = native::aliquot_sum(n) { prop_assert_eq!(aliquot_sum(&b).unwrap(), BigUint::from(v)); }
            if let Some(v) = native::abundance(n) { prop_assert_eq!(abundance(&b).unwrap(), BigInt::from(v)); }
            if let Some(v) = native::dedekind_psi(n) { prop_assert_eq!(dedekind_psi(&b).unwrap(), BigUint::from(v)); }
            if let Some(v) = native::jordan_totient(u64::from(k), n) { prop_assert_eq!(jordan_totient(u64::from(k), &b).unwrap(), BigUint::from(v)); }
            prop_assert!(inverse_totient(&BigUint::from(n | 3)).unwrap().is_empty());
            let factor = u64::from(k) + 1;
            prop_assert_eq!(remove_factors(&b, &BigUint::from(factor)), native::remove_factors(n, factor).map(BigUint::from));
            prop_assert_eq!(remove_factors_exp(&b, &BigUint::from(factor)), native::remove_factors_exp(n, factor).map(|(r,e)| (BigUint::from(r),u64::from(e))));
            prop_assert_eq!(ramanujan_sum(&b, &BigUint::from(1_u8)).unwrap(), BigInt::from(native::ramanujan_sum(n, 1)));
        }
        prop_assert_eq!(inverse_totient(&BigUint::from(small)).unwrap(), native::inverse_totient(small).into_iter().map(BigUint::from).collect::<Vec<_>>());
    }
}

#[test]
fn boundaries_and_repeated_calls() {
    for n in 0_u64..40 {
        let b = BigUint::from(n);
        assert_eq!(
            inverse_totient(&b).unwrap(),
            native::inverse_totient(n)
                .into_iter()
                .map(BigUint::from)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            jordan_totient(0, &b).unwrap(),
            BigUint::from(u8::from(n == 1))
        );
        assert_eq!(
            powerfree_part(&b, 0).unwrap(),
            BigUint::from(u8::from(n == 1))
        );
        assert_eq!(
            ramanujan_sum(&b, &b).unwrap(),
            BigInt::from(native::ramanujan_sum(n, n))
        );
    }
}

#[test]
#[should_panic(expected = "positive")]
fn zero_root_degree() {
    crootint(&BigUint::from(1_u8), 0);
}

#[test]
#[should_panic(expected = "exceed one")]
fn invalid_removal_factor() {
    remove_factors(&BigUint::from(0_u8), &BigUint::from(1_u8));
}

#[test]
fn bigint_removal_and_totient_identity() {
    let k = (BigUint::from(1_u8) << 80_usize) + 1_u8;
    let n = k.pow(3) * 7_u8;
    assert_eq!(remove_factors_exp(&n, &k), Some((BigUint::from(7_u8), 3)));
    let t = BigUint::from(1_u8) << 64_usize;
    let solutions = inverse_totient(&t).unwrap();
    assert_eq!(solutions.len(), 32);
    for x in solutions {
        assert_eq!(math_prime_util::bigint::divisors::euler_phi(&x).unwrap(), t);
    }
}
