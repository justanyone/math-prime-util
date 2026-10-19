//! Native agreement and edge cases for the additional bigint functions.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::{factor, primality};
use num_bigint::BigUint;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn bigint_moretests_native(n: u64, b: u64, base in 2_u32..=10, restriction in 0_u32..=3) {
        let big = BigUint::from(n);
        prop_assert_eq!(primality::is_safe_prime(&big), math_prime_util::is_safe_prime(n));
        prop_assert_eq!(primality::is_chen_prime(&big), math_prime_util::is_chen_prime(n));
        prop_assert_eq!(primality::is_delicate_prime(&big, base), math_prime_util::is_delicate_prime(n, base));
        prop_assert_eq!(primality::is_gaussian_prime(&big, &BigUint::from(b)), math_prime_util::is_gaussian_prime(n, b));
        prop_assert_eq!(primality::is_frobenius_pseudoprime(&big), math_prime_util::is_frobenius_pseudoprime(n));
        prop_assert_eq!(primality::is_frobenius_khashin_pseudoprime(&big), math_prime_util::is_frobenius_khashin_pseudoprime(n));
        prop_assert_eq!(primality::is_frobenius_underwood_pseudoprime(&big), math_prime_util::is_frobenius_underwood_pseudoprime(n));
        prop_assert_eq!(primality::is_perrin_pseudoprime(&big, restriction), math_prime_util::is_perrin_pseudoprime(n, restriction));
        prop_assert_eq!(factor::lehman_factor(&big, None), math_prime_util::lehman_factor(n, None).into_iter().map(BigUint::from).collect::<Vec<_>>());
        prop_assert_eq!(factor::fermat_factor(&big, Some(10)), math_prime_util::fermat_factor(n, Some(10)).into_iter().map(BigUint::from).collect::<Vec<_>>());
        prop_assert_eq!(factor::cheb_factor(&big, 100, 72), math_prime_util::cheb_factor(n, 100, 72).into_iter().map(BigUint::from).collect::<Vec<_>>());
    }
}

#[test]
fn bigint_moretests_boundaries() {
    for n in 0_u64..=300 {
        let big = BigUint::from(n);
        for r in 0..=3 {
            assert_eq!(
                primality::is_perrin_pseudoprime(&big, r),
                math_prime_util::is_perrin_pseudoprime(n, r)
            );
        }
        assert_eq!(
            primality::is_frobenius_pseudoprime(&big),
            math_prime_util::is_frobenius_pseudoprime(n)
        );
        assert_eq!(
            primality::is_frobenius_khashin_pseudoprime(&big),
            math_prime_util::is_frobenius_khashin_pseudoprime(n)
        );
        assert_eq!(
            primality::is_frobenius_underwood_pseudoprime(&big),
            math_prime_util::is_frobenius_underwood_pseudoprime(n)
        );
    }
    let p = (BigUint::from(1_u8) << 127_usize) - 1_u8;
    let square = &p * &p;
    assert_eq!(factor::fermat_factor(&square, Some(0)), vec![p.clone(), p]);
    assert_eq!(
        factor::fermat_factor(&(&square + 2_u8), Some(0))
            .iter()
            .product::<BigUint>(),
        &square + 2_u8
    );
}

#[test]
#[should_panic(expected = "base must be >= 2")]
fn bigint_moretests_invalid_base() {
    let _ = primality::is_delicate_prime(&2_u8.into(), 1);
}

#[test]
#[should_panic(expected = "restriction must")]
fn bigint_moretests_invalid_restriction() {
    let _ = primality::is_perrin_pseudoprime(&2_u8.into(), 4);
}

#[test]
fn bigint_moretests_lehman_cutoff() {
    let p = (BigUint::from(1_u8) << 127_usize) - 1_u8;
    let n = &p * 7_u8;
    assert_eq!(factor::lehman_factor(&n, None), vec![7_u8.into(), p]);
    assert_eq!(factor::lehman_factor(&n, Some(false)), vec![n]);
    let unsplit = BigUint::from(2017_u16).pow(10);
    assert_eq!(factor::lehman_factor(&unsplit, Some(true)), vec![unsplit]);
}
