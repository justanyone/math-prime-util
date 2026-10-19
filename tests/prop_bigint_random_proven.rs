//! Native agreement, certificates, and input checks for the proven random primes.
#![cfg(feature = "bigint")]
use math_prime_util::{bigint::random as big, *};
use num_bigint::BigUint;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]
    #[test]
    fn native_stream(seed in any::<u64>(), bits in 2_u32..=64) {
        srand(seed);
        let got = big::random_maurer_prime(bits);
        srand(seed);
        prop_assert_eq!(got, random_maurer_prime(bits).map(BigUint::from));
        srand(seed);
        let got = big::random_maurer_prime_with_cert(bits).unwrap();
        srand(seed);
        let (p, _) = random_maurer_prime_with_cert(bits).unwrap();
        prop_assert_eq!(&got.0, &BigUint::from(p));
        prop_assert_eq!(got.1.verify(), Ok(true));
        srand(seed);
        let got = big::random_proven_prime(bits);
        srand(seed);
        prop_assert_eq!(got, random_proven_prime(bits).map(BigUint::from));
        srand(seed);
        let got = big::random_proven_prime_with_cert(bits).unwrap();
        srand(seed);
        let (p, _) = random_proven_prime_with_cert(bits).unwrap();
        prop_assert_eq!(&got.0, &BigUint::from(p));
        prop_assert_eq!(got.1.verify(), Ok(true));
        srand(seed);
        let got = big::random_shawe_taylor_prime(bits).unwrap();
        srand(seed);
        let native = BigUint::from(random_shawe_taylor_prime(bits).unwrap());
        prop_assert_eq!(got, native);
        srand(seed);
        let got = big::random_shawe_taylor_prime_with_cert(bits).unwrap();
        srand(seed);
        let (p, _) = random_shawe_taylor_prime_with_cert(bits).unwrap().unwrap();
        prop_assert_eq!(&got.0, &BigUint::from(p));
        prop_assert_eq!(got.1.verify(), Ok(true));
    }

    #[test]
    fn big_sizes_are_proven(seed in any::<u64>(), bits in 65_u32..=160) {
        srand(seed);
        let p = big::random_maurer_prime(bits).unwrap();
        prop_assert_eq!(p.bits(), u64::from(bits));
        let (p, c) = big::random_shawe_taylor_prime_with_cert(bits).unwrap();
        prop_assert_eq!(p.bits(), u64::from(bits));
        prop_assert_eq!(c.verify(), Ok(true));
        let (p, c) = big::random_proven_prime_with_cert(bits).unwrap();
        prop_assert_eq!(p.bits(), u64::from(bits));
        prop_assert_eq!(c.verify(), Ok(true));
    }
}

#[test]
fn reseed_repeats() {
    for bits in [65, 128, 200] {
        srand(9);
        let a = big::random_maurer_prime(bits);
        srand(9);
        assert_eq!(big::random_maurer_prime(bits), a);
        srand(9);
        let a = big::random_shawe_taylor_prime(bits);
        srand(9);
        assert_eq!(big::random_shawe_taylor_prime(bits), a);
    }
}

#[test]
fn invalid_inputs() {
    for bits in [0, 1] {
        assert!(std::panic::catch_unwind(|| big::random_maurer_prime(bits)).is_err());
        assert!(std::panic::catch_unwind(|| big::random_maurer_prime_with_cert(bits)).is_err());
        assert!(std::panic::catch_unwind(|| big::random_shawe_taylor_prime(bits)).is_err());
        assert!(
            std::panic::catch_unwind(|| big::random_shawe_taylor_prime_with_cert(bits)).is_err()
        );
        assert!(std::panic::catch_unwind(|| big::random_proven_prime(bits)).is_err());
        assert!(std::panic::catch_unwind(|| big::random_proven_prime_with_cert(bits)).is_err());
    }
}
