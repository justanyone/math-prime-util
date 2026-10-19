//! BigUint integer functions agree with the native functions.
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
