#![cfg(feature = "bigint")]
use math_prime_util::{bigint::random as big, *};
use num_bigint::BigUint;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]
    #[test]
    fn native_stream(seed in any::<u64>(), n in 1_u64..10000, bits in 4_u32..20) {
        macro_rules! same {
            ($big:expr, $native:expr) => {{
                srand(seed); let got = $big;
                srand(seed); assert_eq!(got, $native);
            }};
        }
        same!(big::urandomb(bits), BigUint::from(urandomb(bits)));
        same!(big::urandomm(&n.into()), BigUint::from(urandomm(n)));
        same!(big::urandomr(&n.into(), &(n+100).into()), urandomr(n,n+100).map(BigUint::from));
        same!(big::random_prime(&n.into(), &(n+100).into()), random_prime(n,n+100).map(BigUint::from));
        same!(big::random_nbit_prime(bits), random_nbit_prime(bits).map(BigUint::from));
        same!(big::random_ndigit_prime(3), random_ndigit_prime(3).map(BigUint::from));
        same!(big::random_safe_prime(bits), random_safe_prime(bits).map(BigUint::from));
        same!(big::random_semiprime(bits), random_semiprime(bits).map(BigUint::from));
        same!(big::random_unrestricted_semiprime(bits), random_unrestricted_semiprime(bits).map(BigUint::from));
        same!(big::miller_rabin_random(&n.into(), 3), miller_rabin_random(u128::from(n),3));
        srand(seed); let (r,f) = big::random_factored_integer(&n.into());
        srand(seed); let (nr,nf) = random_factored_integer(n);
        prop_assert_eq!((r,f), (nr.into(),nf.into_iter().map(BigUint::from).collect()));
    }
}

#[test]
fn bigint_edges_and_reseed() {
    let lo = BigUint::from(1_u8) << 80_usize;
    assert_eq!(big::urandomr(&(&lo+1_u8), &lo), None);
    assert_eq!(big::urandomr(&lo, &lo), Some(lo.clone()));
    assert_eq!(big::random_prime(&(&lo+1_u8), &lo), None);
    assert_eq!(big::random_prime(&lo, &lo), None);
    assert_eq!(big::urandomr(&0_u8.into(), &u64::MAX.into()).unwrap().bits() <= 64, true);
    for bits in [65, 79, 80, 128, 512] {
        srand(123); let a = big::random_nbit_prime(bits).unwrap();
        srand(123); assert_eq!(big::random_nbit_prime(bits), Some(a.clone()));
        assert_eq!(a.bits(), u64::from(bits));
        assert_ne!(math_prime_util::bigint::primality::is_prob_prime(&a), 0);
    }
    for seed in 0..12 {
        srand(seed);
        let p = big::random_unrestricted_semiprime(65).unwrap();
        assert_eq!(p.bits(), 65);
    }
    assert_eq!(big::miller_rabin_random(&lo, 5), Some(0));
    let handles: Vec<_> = (0..4).map(|_| std::thread::spawn(|| {
        srand(55); big::urandomb(100)
    })).collect();
    let values: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert!(values.windows(2).all(|v| v[0] == v[1]));
}

#[test]
fn invalid_inputs() {
    assert!(std::panic::catch_unwind(|| big::urandomm(&0_u8.into())).is_err());
    assert!(std::panic::catch_unwind(|| big::random_factored_integer(&0_u8.into())).is_err());
    assert!(std::panic::catch_unwind(|| big::random_nbit_prime(1)).is_err());
    assert!(std::panic::catch_unwind(|| big::random_ndigit_prime(0)).is_err());
    assert!(std::panic::catch_unwind(|| big::random_safe_prime(2)).is_err());
    assert!(std::panic::catch_unwind(|| big::random_strong_prime(127)).is_err());
    assert!(std::panic::catch_unwind(|| big::random_semiprime(3)).is_err());
    assert!(std::panic::catch_unwind(|| big::random_unrestricted_semiprime(2)).is_err());
    assert!(std::panic::catch_unwind(|| big::miller_rabin_random(&7_u8.into(),0)).is_err());
}
