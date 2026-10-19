//! `BigUint` family counts agree with the native forms on the u64 range.
#![cfg(feature = "bigint")]
use math_prime_util::{
    almost_prime_count, bigint::family_counts as f, legendre_phi, mertens, nth_perfect_power,
    nth_powerfree, omega_prime_count, perfect_power_count, powerfree_count, powerfree_part_sum,
    powerfree_sum, powerful_count, prime_count, prime_power_count, rough_count, semiprime_count,
    smooth_count, sum_primes, sumliouville, sumpowerful, sumtotient, twin_prime_count,
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
        prop_assert_eq!(f::twin_prime_count(&z,&b), BigUint::from(twin_prime_count(n)));
        prop_assert_eq!(f::prime_power_count(&z,&b), BigUint::from(prime_power_count(n)));
        prop_assert_eq!(f::nth_perfect_power(&b), nth_perfect_power(n).map(BigUint::from));
        prop_assert_eq!(f::nth_powerfree(&b,k).expect("factors"), nth_powerfree(n,k).map(BigUint::from));
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
#[test]
fn nth_edges_and_range_splits() {
    let z = BigUint::from(0_u8);
    // Rank zero and a powerfree order below two have no value.
    assert_eq!(f::nth_perfect_power(&z), None);
    assert_eq!(f::nth_powerfree(&z, 3).expect("zero"), None);
    assert_eq!(f::nth_powerfree(&BigUint::from(5_u8), 1).expect("k"), None);
    // A rank above the bit length of k is its own value: all below 2^k are free.
    let n = BigUint::from(12345_u32);
    assert_eq!(f::nth_powerfree(&n, 40).expect("free"), Some(n.clone()));
    // Counts split over adjacent ranges.
    let (a, b, c) = (
        BigUint::from(10_u8),
        BigUint::from(60_u8),
        BigUint::from(61_u8),
    );
    let whole = f::twin_prime_count(&a, &BigUint::from(120_u8));
    let parts = f::twin_prime_count(&a, &b) + f::twin_prime_count(&c, &BigUint::from(120_u8));
    assert_eq!(whole, parts);
    assert_eq!(f::prime_power_count(&c, &b), z);
    // One is not a prime power; two and four are.
    assert_eq!(
        f::prime_power_count(&BigUint::from(1_u8), &BigUint::from(1_u8)),
        z
    );
    assert_eq!(
        f::prime_power_count(&BigUint::from(2_u8), &BigUint::from(4_u8)),
        BigUint::from(3_u8)
    );
}
