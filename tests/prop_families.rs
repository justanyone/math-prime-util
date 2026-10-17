//! Algebraic checks for the assigned almost/omega functions.
use math_prime_util::{is_almost_prime, is_omega_prime, nth_almost_prime_upper, nth_omega_prime, omega_prime_count, omega_primes};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]
    #[test]
    fn count_difference_is_membership(k in 0_u32..7, n in 1_u64..10000) {
        prop_assert_eq!(omega_prime_count(k,n)-omega_prime_count(k,n-1), u64::from(is_omega_prime(k,n)));
    }
    #[test]
    fn list_matches_count(k in 0_u32..7, lo in 0_u64..1000, width in 0_u64..1000) {
        let hi=lo+width;
        let list=omega_primes(k,lo,hi);
        prop_assert!(list.windows(2).all(|v| v[0]<v[1]));
        prop_assert!(list.iter().all(|&v| is_omega_prime(k,v)));
        prop_assert_eq!(list.len() as u64,omega_prime_count(k,hi)-if lo==0 { 0 } else { omega_prime_count(k,lo-1) });
    }
    #[test]
    fn nth_inverts_count(k in 1_u32..6, n in 1_u64..100) {
        let value=nth_omega_prime(k,n).unwrap();
        prop_assert!(is_omega_prime(k,value));
        prop_assert_eq!(omega_prime_count(k,value),n);
        prop_assert_eq!(omega_prime_count(k,value-1),n-1);
    }
    #[test]
    fn almost_upper_bounds_nth(k in 1_u32..7,n in 1_u64..60) {
        let upper=nth_almost_prime_upper(k,n).unwrap();
        let count=(1..=upper).filter(|&v| is_almost_prime(k,v)).count() as u64;
        prop_assert!(count>=n);
    }
}

#[test]
fn invalid_and_native_limits() {
    assert_eq!(nth_almost_prime_upper(64,1),None);
    assert_eq!(nth_almost_prime_upper(63,3),None);
    assert_eq!(nth_almost_prime_upper(63,2),Some(13_835_058_055_282_163_712));
    assert_eq!(nth_omega_prime(16,1),None);
    assert_eq!(nth_omega_prime(15,299_179),None);
    assert_eq!(omega_prime_count(16,u64::MAX),0);
    assert!(omega_primes(16,1,u64::MAX).is_empty());
    assert!(omega_primes(0,2,100).is_empty());
    assert!(omega_primes(3,1,29).is_empty());
    assert!(omega_primes(2,100,1).is_empty());
}

#[test]
fn concurrent_repeated_calls_are_independent() {
    let expected=omega_primes(3,1,3000);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let expected=&expected;
            scope.spawn(move || {
                for _ in 0..2 {
                    assert_eq!(&omega_primes(3,1,3000),expected);
                    assert_eq!(omega_prime_count(3,3000),expected.len() as u64);
                    assert_eq!(nth_omega_prime(3,1000),Some(2988));
                }
            });
        }
    });
}
