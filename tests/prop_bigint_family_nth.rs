//! `BigUint` nth values and powerful number lists agree with the native
//! forms on the u64 range, and have the defining property past it.
#![cfg(feature = "bigint")]
use math_prime_util::bigint::divisors::{prime_bigomega, prime_omega};
use math_prime_util::bigint::family_nth as f;
use math_prime_util::bigint::predicates::is_powerful;
use math_prime_util::{nth_almost_prime, nth_omega_prime, nth_powerful, powerful_numbers};
use num_bigint::BigUint;
use proptest::prelude::*;

fn two64() -> BigUint {
    BigUint::from(1_u8) << 64_usize
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn native_agreement(n in 0_u64..3000, k in 0_u32..9, lo in 0_u64..5000, len in 0_u64..3000) {
        let b = BigUint::from(n);
        prop_assert_eq!(f::nth_almost_prime(k, &b), nth_almost_prime(k, n).map(BigUint::from));
        prop_assert_eq!(f::nth_omega_prime(k, &b), nth_omega_prime(k, n).map(BigUint::from));
        prop_assert_eq!(f::nth_powerful(&b, k), nth_powerful(n, k).map(BigUint::from));
        let want: Vec<BigUint> = powerful_numbers(lo, lo + len, k).into_iter().map(BigUint::from).collect();
        prop_assert_eq!(f::powerful_numbers(&lo.into(), &(lo + len).into(), k), want);
    }

    #[test]
    fn native_agreement_large_k(n in 1_u64..400, k in 9_u32..64) {
        let b = BigUint::from(n);
        if let Some(v) = nth_almost_prime(k, n) {
            prop_assert_eq!(f::nth_almost_prime(k, &b), Some(BigUint::from(v)));
        }
        if let Some(v) = nth_powerful(n, k) {
            prop_assert_eq!(f::nth_powerful(&b, k), Some(BigUint::from(v)));
        }
    }

    #[test]
    fn almost_primes_past_two_to_64(n in 1_u64..2000, k in 60_u32..90) {
        let v = f::nth_almost_prime(k, &BigUint::from(n)).expect("value");
        let next = f::nth_almost_prime(k, &BigUint::from(n + 1)).expect("value");
        prop_assert_eq!(prime_bigomega(&v), Ok(u64::from(k)));
        prop_assert!(v < next);
        prop_assert!(k < 64 || v >= two64());
        // One more factor of two below 3^k: the nth (k + 1)-almost prime is twice it.
        if v < BigUint::from(3_u8).pow(k) {
            prop_assert_eq!(f::nth_almost_prime(k + 1, &BigUint::from(n)), Some(v * 2_u8));
        }
    }

    #[test]
    fn powerful_past_two_to_64(n in 2_u64..60, k in 64_u32..80) {
        let v = f::nth_powerful(&BigUint::from(n), k).expect("value");
        let next = f::nth_powerful(&BigUint::from(n + 1), k).expect("value");
        prop_assert_eq!(is_powerful(&v, &BigUint::from(k)), Ok(true));
        prop_assert!(v < next && v >= two64());
        // Nothing k-powerful lies strictly between them.
        prop_assert_eq!(f::powerful_numbers(&v, &next, k), vec![v, next]);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(8))]
    #[test]
    fn powerful_windows_past_two_to_64(offset in 0_u64..1_000_000_000, len in 0_u64..60, k in 2_u32..7) {
        let lo = two64() + offset;
        let hi = &lo + len;
        let got = f::powerful_numbers(&lo, &hi, k);
        let mut want = Vec::new();
        let mut x = lo.clone();
        while x <= hi {
            if is_powerful(&x, &BigUint::from(k)) == Ok(true) {
                want.push(x.clone());
            }
            x += 1_u8;
        }
        prop_assert_eq!(got, want);
    }

    #[test]
    fn omega_primes_past_two_to_64(n in 1_u64..6, k in 16_u32..18) {
        let v = f::nth_omega_prime(k, &BigUint::from(n)).expect("value");
        let next = f::nth_omega_prime(k, &BigUint::from(n + 1)).expect("value");
        prop_assert_eq!(prime_omega(&v), Ok(u64::from(k)));
        prop_assert!(v < next);
    }
}

#[test]
fn edges() {
    let z = BigUint::from(0_u8);
    let one = BigUint::from(1_u8);
    for k in [0, 1, 2, 70] {
        assert_eq!(f::nth_almost_prime(k, &z), None);
        assert_eq!(f::nth_omega_prime(k, &z), None);
        assert_eq!(f::nth_powerful(&z, k), None);
    }
    assert_eq!(f::nth_almost_prime(0, &one), Some(one.clone()));
    assert_eq!(f::nth_almost_prime(0, &two64()), None);
    assert_eq!(f::nth_omega_prime(0, &one), Some(one.clone()));
    assert_eq!(f::nth_omega_prime(0, &BigUint::from(2_u8)), None);
    // k <= 1 makes every positive integer powerful.
    assert_eq!(f::nth_powerful(&two64(), 1), Some(two64()));
    assert_eq!(f::nth_powerful(&two64(), 0), Some(two64()));
    let hi = two64() + 3_u8;
    assert_eq!(f::powerful_numbers(&two64(), &hi, 1).len(), 4);
    assert_eq!(f::powerful_numbers(&hi, &two64(), 2), Vec::<BigUint>::new());
    // k past log2(hi): only 1 is k-powerful.
    assert_eq!(f::powerful_numbers(&z, &hi, 65), vec![one.clone()]);
    assert_eq!(
        f::powerful_numbers(&two64(), &hi, 65),
        Vec::<BigUint>::new()
    );
    // The first k-powerful integers past 1 are 2^k and 2^(k+1).
    let k = 100;
    assert_eq!(
        f::nth_powerful(&BigUint::from(2_u8), k),
        Some(&one << 100_usize)
    );
    assert_eq!(
        f::nth_powerful(&BigUint::from(3_u8), k),
        Some(&one << 101_usize)
    );
    // An index past 2^64 for large k: 2^r times a native almost prime.
    let v = f::nth_almost_prime(1000, &BigUint::from(500_u16)).expect("value");
    assert_eq!(prime_bigomega(&v), Ok(1000));
}
