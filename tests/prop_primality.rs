//! Algebraic laws and boundary cases for generic integers.
use math_prime_util::Int;
use proptest::prelude::*;

proptest! {
    #[test]
    fn gcd_laws(a: u128, b: u128) {
        let g = a.gcd(&b);
        prop_assert_eq!(g, b.gcd(&a));
        prop_assert_eq!(a.gcd(&0), a);
        if g != 0 {
            prop_assert_eq!(a % g, 0);
            prop_assert_eq!(b % g, 0);
        }
    }

    #[test]
    fn factor_product(n in 1_u128..1_000_000) {
        let factors = n.factor().unwrap();
        prop_assert_eq!(factors.iter().product::<u128>(), n);
        prop_assert!(factors.iter().all(|p| p.is_prime() != 0));
        prop_assert!(factors.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn powers_add(a: u128, m in 1_u128..=u128::MAX, x in 0_u128..64, y in 0_u128..64) {
        let ax = a.powmod(&x, &m).unwrap();
        // Repeated modular addition gives an independent multiplication oracle.
        let mut expected = ax;
        for _ in 0..y {
            let mut product = 0_u128;
            let mut term = expected;
            let mut b = a % m;
            while b != 0 {
                if b & 1 != 0 {
                    product = if product >= m - term { product - (m - term) } else { product + term };
                }
                term = if term >= m - term { term - (m - term) } else { term + term };
                b >>= 1;
            }
            expected = product;
        }
        prop_assert_eq!(a.powmod(&(x + y), &m), Some(expected));
    }

    #[test]
    fn next_prime_is_minimal(n in 0_u128..100_000) {
        let p = n.next_prime().unwrap();
        prop_assert!(p > n && p.is_prime() != 0);
        prop_assert!((n + 1..p).all(|k| k.is_prime() == 0));
    }
}

#[test]
fn boundaries_and_repeated_calls() {
    assert_eq!(0_u128.gcd(&0), 0);
    assert_eq!(0_u128.factor(), Ok(vec![0]));
    assert_eq!(1_u128.factor(), Ok(vec![]));
    assert_eq!(2_u128.powmod(&3, &0), None);
    assert_eq!(0_u128.powmod(&0, &1), Some(0));
    assert_eq!(0_u128.powmod(&0, &2), Some(1));
    assert_eq!((u128::MAX - 1).powmod(&2, &u128::MAX), Some(1));
    assert_eq!(u128::MAX.powmod(&1, &u128::MAX), Some(0));
    assert_eq!(u128::MAX.next_prime(), None);
    assert_eq!((u128::MAX - 1).next_prime(), None);
    assert_eq!(u64::MAX.next_prime(), None);
    assert_eq!(
        (1_u128 << 64).next_prime(),
        Some(18_446_744_073_709_551_629)
    );
    for n in 0_u128..10 {
        assert_eq!(n.next_prime(), n.next_prime());
    }
}

#[test]
fn concurrent_calls_have_no_shared_operation_state() {
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                assert_eq!(360_u128.factor(), Ok(vec![2, 2, 2, 3, 3, 5]));
                assert_eq!(97_u128.next_prime(), Some(101));
                assert_eq!(3_u128.powmod(&5, &7), Some(5));
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}
