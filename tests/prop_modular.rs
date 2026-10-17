//! Algebraic laws for the native modular functions.
use math_prime_util::{binomialmod, factorialmod, gcd, mulmod, pisano_period, powmod, znlog};
use proptest::prelude::*;

proptest! {
    #[test]
    fn factorial_recurrence(n in 1u64..1500, m in 1u64..10000) {
        prop_assert_eq!(factorialmod(n,m), mulmod(factorialmod(n-1,m).unwrap(),n,m));
    }
    #[test]
    fn binomial_pascal(n in 1u64..1000,k in 1u64..1000,m in 1u64..2000) {
        let k = k.min(n);
        prop_assert_eq!(binomialmod(n,k,m), math_prime_util::addmod(binomialmod(n-1,k-1,m).unwrap(),binomialmod(n-1,k,m).unwrap(),m));
        prop_assert_eq!(binomialmod(n,k,m),binomialmod(n,n-k,m));
    }
    #[test]
    fn logarithm_roundtrip(g in 1u64..200,n in 2u64..300,k in 0u64..400) {
        let a = powmod(g,k,n).unwrap();
        let x = znlog(a,g,n).unwrap();
        prop_assert_eq!(powmod(g,x,n),Some(a));
        prop_assert!(x <= k);
        if gcd(g,n) == 1 { prop_assert!(x < n); }
    }
    #[test]
    fn pisano_returns_first_period(n in 2u64..300) {
        let period = pisano_period(n);
        let (mut a,mut b,mut k) = (0,1,0u128);
        loop {
            (a,b) = (b,(a+b)%n); k += 1;
            if a == 0 && b == 1 { break; }
        }
        prop_assert_eq!(period,k);
    }
}

#[test]
fn zero_moduli_and_repeat_calls() {
    assert_eq!(factorialmod(0, 0), None);
    assert_eq!(binomialmod(0, 0, 0), None);
    assert_eq!(znlog(0, 0, 0), None);
    assert_eq!(pisano_period(0), 0);
    for _ in 0..2 {
        assert_eq!(znlog(0, 2, 8), Some(3));
        assert_eq!(znlog(12, 98, 100), Some(19));
        assert_eq!(binomialmod(500_000, 200_000, 1024), Some(640));
    }
}

#[test]
fn concurrent_calls() {
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                assert_eq!(znlog(5, 2, 1019), Some(10));
                assert_eq!(factorialmod(1000, 1009), Some(782));
                assert_eq!(binomialmod(10, 3, 13), Some(3));
                assert_eq!(pisano_period(1000), 1500);
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}
