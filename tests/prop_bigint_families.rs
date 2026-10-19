//! Check bigint family lists against native results.
#![cfg(feature = "bigint")]
use math_prime_util::{self as native, bigint::families as big};
use num_bigint::BigUint;
use proptest::prelude::*;
fn values(v: Vec<u64>) -> Vec<BigUint> {
    v.into_iter().map(BigUint::from).collect()
}
proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]
    #[test]
    fn native_agreement(n in 0_u64..300, width in 0_u64..20, k in 0_u32..5) {
        let lo = BigUint::from(n);
        let hi = BigUint::from(n + width);
        prop_assert_eq!(big::primes(&lo,&hi), values(native::primes(n,n+width)));
        prop_assert_eq!(big::twin_primes(&lo,&hi), values(native::twin_primes(n,n+width)));
        prop_assert_eq!(big::prime_powers(&lo,&hi), values(native::prime_powers(n,n+width)));
        prop_assert_eq!(big::semi_primes(&lo,&hi).unwrap(), values(native::semi_primes(n,n+width)));
        prop_assert_eq!(big::almost_primes(u64::from(k),&lo,&hi).unwrap(), values(native::almost_primes(u64::from(k),n,n+width)));
        prop_assert_eq!(big::omega_primes(k,&lo,&hi).unwrap(), values(native::omega_primes(k,n,n+width)));
        prop_assert_eq!(big::powerful_numbers(&lo,&hi,k).unwrap(), values(native::powerful_numbers(n,n+width,k)));
        prop_assert_eq!(big::sieve_range(&lo,&hi,&7_u8.into()), values(native::sieve_range(n,width+1,7)));
        prop_assert_eq!(big::sieve_prime_cluster(&lo,&hi,&[0,2,6]), values(native::sieve_prime_cluster(n,n+width,&[0,2,6])));
        prop_assert_eq!(Some(big::next_prime_power(&lo)),native::next_prime_power(n).map(BigUint::from));
        prop_assert_eq!(big::prev_prime_power(&lo),native::prev_prime_power(n).map(BigUint::from));
        prop_assert_eq!(Some(big::next_perfect_power(&lo)),native::next_perfect_power(n).map(BigUint::from));
        prop_assert_eq!(big::prev_perfect_power(&lo),native::prev_perfect_power(n).map(BigUint::from));
        prop_assert_eq!(Some(big::next_chen_prime(&lo).unwrap()),native::next_chen_prime(n).map(BigUint::from));
        prop_assert_eq!(big::minimal_goldbach_pair(&lo),native::minimal_goldbach_pair(n).map(BigUint::from));
        let mut a=Vec::new(); let mut b=Vec::new();
        big::print_primes(&lo,&hi,&mut a).unwrap();
        native::print_primes(n,n+width,&mut b).unwrap();
        prop_assert_eq!(a,b);
    }
}

#[test]
fn edges_and_repeated_calls() {
    for n in 0_u8..12 {
        let n = BigUint::from(n);
        assert_eq!(big::next_perfect_power(&n), big::next_perfect_power(&n));
        assert_eq!(
            big::prev_perfect_power(&n),
            native::prev_perfect_power(u64::try_from(&n).unwrap()).map(BigUint::from)
        );
    }
    assert!(big::primes(&10_u8.into(), &2_u8.into()).is_empty());
    assert!(big::primes(&0_u8.into(), &1_u8.into()).is_empty());
    assert!(
        big::almost_primes(0, &0_u8.into(), &0_u8.into())
            .unwrap()
            .is_empty()
    );
    assert!(
        big::omega_primes(0, &0_u8.into(), &0_u8.into())
            .unwrap()
            .is_empty()
    );
    assert!(
        big::powerful_numbers(&0_u8.into(), &0_u8.into(), 2)
            .unwrap()
            .is_empty()
    );
    assert!(big::sieve_range(&10_u8.into(), &2_u8.into(), &0_u8.into()).is_empty());
    let handles: Vec<_> = (0..2)
        .map(|_| std::thread::spawn(|| big::primes(&2_u8.into(), &19_u8.into())))
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), values(native::primes(2, 19)));
    }
    assert!(
        big::print_primes(
            &2_u8.into(),
            &3_u8.into(),
            std::io::Write::by_ref(&mut FailingWriter)
        )
        .is_err()
    );
}
struct FailingWriter;
impl std::io::Write for FailingWriter {
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("test"))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[test]
#[should_panic(expected = "offsets must be even and increasing")]
fn rejects_odd_offset() {
    big::sieve_prime_cluster(&2_u8.into(), &5_u8.into(), &[1]);
}
#[test]
#[should_panic(expected = "offsets must be even and increasing")]
fn rejects_duplicate_offset() {
    big::sieve_prime_cluster(&2_u8.into(), &5_u8.into(), &[2, 2]);
}
