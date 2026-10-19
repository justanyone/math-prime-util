#![cfg(feature = "bigint")]
use math_prime_util::{
    almost_prime_count, bigint::family_counts as f, legendre_phi, mertens, omega_prime_count,
    perfect_power_count, powerfree_count, powerfree_part_sum, powerfree_sum, powerful_count,
    prime_count, rough_count, semiprime_count, smooth_count, sum_primes, sumliouville, sumpowerful,
    sumtotient,
};
use num_bigint::{BigInt, BigUint};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]
    #[test]
    fn native_agreement(n in 0_u64..300, k in 0_u32..8, a in 0_u64..6) {
        let b = BigUint::from(n);
        let z = BigUint::from(0_u8);
        prop_assert_eq!(f::perfect_power_count(&b), BigUint::from(perfect_power_count(n)));
        prop_assert_eq!(f::powerfree_count(&b,k).expect("factors"), BigUint::from(powerfree_count(n,k)));
        prop_assert_eq!(f::powerfree_sum(&b,k).expect("factors"), BigUint::from(powerfree_sum(n,k)));
        prop_assert_eq!(f::powerfree_part_sum(&b,k).expect("factors"), BigUint::from(powerfree_part_sum(n,k)));
        prop_assert_eq!(f::legendre_phi(&b,a), BigUint::from(legendre_phi(n,a)));
        prop_assert_eq!(f::rough_count(&b,u64::from(k)), BigUint::from(rough_count(n,u64::from(k))));
        prop_assert_eq!(f::smooth_count(&b,u64::from(k)), BigUint::from(smooth_count(n,u64::from(k))));
        prop_assert_eq!(f::semiprime_count(&z,&b).expect("factors"), BigUint::from(semiprime_count(n)));
        prop_assert_eq!(f::prime_count(&z,&b).expect("prime"), BigUint::from(prime_count(n)));
        prop_assert_eq!(f::sum_primes(&z,&b).expect("prime"), BigUint::from(sum_primes(0, n).expect("small")));
        prop_assert_eq!(f::sumtotient(&z,&b).expect("factors"), BigUint::from(sumtotient(n).expect("small")));
        prop_assert_eq!(f::mertens(&z,&b).expect("factors"), BigInt::from(mertens(n)));
        prop_assert_eq!(f::sumliouville(&z,&b).expect("factors"), BigInt::from(sumliouville(n)));
        prop_assert_eq!(f::almost_prime_count(u64::from(k),&z,&b).expect("factors"),
            BigUint::from(almost_prime_count(u64::from(k),n)));
        prop_assert_eq!(f::omega_prime_count(u64::from(k),&z,&b).expect("factors"),
            BigUint::from(omega_prime_count(k,n)));
        prop_assert_eq!(f::powerful_count(k,&z,&b).expect("factors"), BigUint::from(powerful_count(n,k)));
        prop_assert_eq!(f::sumpowerful(k,&z,&b).expect("factors"), BigUint::from(sumpowerful(n,k)));
    }
}

#[test]
fn empty_ranges_and_prefix_edges() {
    let z = BigUint::from(0_u8);
    let one = BigUint::from(1_u8);
    assert_eq!(f::perfect_power_count(&z), z);
    for k in 0..5 {
        assert_eq!(f::powerfree_count(&z, k).expect("zero"), z);
        assert_eq!(f::powerfree_sum(&z, k).expect("zero"), z);
        assert_eq!(f::powerfree_part_sum(&z, k).expect("zero"), z);
        assert_eq!(f::powerful_count(k, &one, &z).expect("empty"), z);
        assert_eq!(f::sumpowerful(k, &one, &z).expect("empty"), z);
    }
    assert_eq!(f::semiprime_count(&one, &z).expect("empty"), z);
    assert_eq!(f::prime_count(&one, &z).expect("empty"), z);
    assert_eq!(f::sum_primes(&one, &z).expect("empty"), z);
    assert_eq!(f::sumtotient(&one, &z).expect("empty"), z);
    assert_eq!(f::mertens(&one, &z).expect("empty"), BigInt::from(0));
    assert_eq!(f::sumliouville(&one, &z).expect("empty"), BigInt::from(0));
}
