//! Native agreement and large structured inputs.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::predicates as b;
use num_bigint::BigUint;
use proptest::prelude::*;
fn big(n: u64) -> BigUint { BigUint::from(n) }
proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]
    #[test]
    fn bigint_predicates_agree(n: u64, small in 0_u64..10000, k in 0_u32..8) {
        for n in [n, small] {
            let bn = big(n);
            let bk = big(u64::from(k));
            prop_assert_eq!(b::is_semiprime(&bn).unwrap(), math_prime_util::is_semiprime(n));
            prop_assert_eq!(b::is_cyclic(&bn).unwrap(), math_prime_util::is_cyclic(n));
            if let Ok(signed) = i64::try_from(n) {
                prop_assert_eq!(b::is_fundamental(&bn).unwrap(), math_prime_util::is_fundamental(signed));
            }
            prop_assert_eq!(b::is_perfect_number(&bn).unwrap(), math_prime_util::is_perfect_number(n));
            prop_assert_eq!(b::is_perfect_power(&bn), math_prime_util::is_perfect_power(n));
            prop_assert_eq!(b::is_prime_power(&bn), math_prime_util::is_prime_power(n));
            prop_assert_eq!(b::is_practical(&bn).unwrap(), math_prime_util::is_practical(n));
            prop_assert_eq!(b::is_totient(&bn).unwrap(), math_prime_util::is_totient(n));
            prop_assert_eq!(b::is_powerfree(&bn, &bk).unwrap(), math_prime_util::is_powerfree(n, k));
            prop_assert_eq!(b::is_powerful(&bn, &bk).unwrap(), math_prime_util::is_powerful(n, k));
            prop_assert_eq!(b::is_rough(&bn, &bk).unwrap(), math_prime_util::is_rough(n, u64::from(k)));
            prop_assert_eq!(b::is_smooth(&bn, &bk).unwrap(), math_prime_util::is_smooth(n, u64::from(k)));
            prop_assert_eq!(b::is_sum_of_squares(&bn, &bk).unwrap(), math_prime_util::is_sum_of_squares(n, u64::from(k)));
            prop_assert_eq!(b::is_almost_prime(&bk, &bn).unwrap(), math_prime_util::is_almost_prime(u64::from(k), n));
            prop_assert_eq!(b::is_omega_prime(&bk, &bn).unwrap(), math_prime_util::is_omega_prime(k, n));
        }
    }
}
#[test]
fn bigint_predicates_boundaries() {
    let zero = big(0);
    let one = big(1);
    let huge = &one << 100_usize;
    assert_eq!(b::is_almost_prime(&big(100), &huge), Ok(true));
    assert_eq!(b::is_omega_prime(&one, &huge), Ok(true));
    assert_eq!(b::is_powerful(&huge, &big(100)), Ok(true));
    assert_eq!(b::is_powerful(&huge, &big(101)), Ok(false));
    assert_eq!(b::is_powerfree(&huge, &big(100)), Ok(false));
    assert_eq!(b::is_powerfree(&huge, &big(101)), Ok(true));
    assert_eq!(b::is_powerfree(&huge, &huge), Ok(true));
    assert_eq!(b::is_smooth(&huge, &huge), Ok(true));
    assert_eq!(b::is_rough(&huge, &huge), Ok(false));
    assert_eq!(b::is_sum_of_squares(&huge, &huge), Ok(true));
    assert_eq!(b::is_sum_of_squares(&zero, &zero), Ok(true));
    assert_eq!(b::is_fundamental(&big(8)), Ok(true));
    assert_eq!(b::is_fundamental(&big(4)), Ok(false));
    let perfect = ((&one << 89_usize) - 1_u8) << 88_usize;
    assert_eq!(b::is_perfect_number(&perfect), Ok(true));
    assert_eq!(b::is_perfect_number(&(&perfect + 2_u8)), Ok(false));
    assert_eq!(b::is_prime_power(&huge), 100);
    assert_eq!(b::is_totient(&huge), Ok(true));
    assert_eq!(b::is_practical(&huge), Ok(true));
}
