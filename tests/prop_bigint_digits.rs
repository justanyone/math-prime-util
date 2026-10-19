//! Native parity and arbitrary precision round trips.
#![cfg(feature = "bigint")]
use math_prime_util::{self as native, bigint::digits};
use num_bigint::BigUint;
use proptest::prelude::*;

proptest! {
    #[test]
    fn bigint_digits_native_parity(n: u64, base in 2_u64..=36, length in 0_usize..90, exponent in 0_u32..=3) {
        let b = BigUint::from(n);
        let i = i128::from(n);
        for len in [None, Some(length)] {
            prop_assert_eq!(digits::todigits(&b, base, len), native::todigits(i, base, len));
            prop_assert_eq!(digits::todigitstring(&b, base, len), native::todigitstring(i, base, len));
        }
        let ds = native::todigits(i, base, None);
        prop_assert_eq!(digits::fromdigits(&ds, base), native::fromdigits(&ds, base).map(BigUint::from));
        prop_assert_eq!(digits::reverse_digits(&b, base), native::reverse_digits(i, base).map(BigUint::from));
        prop_assert_eq!(digits::digital_root(&b, base), BigUint::from(native::digital_root(i, base)));
        prop_assert_eq!(digits::mult_digital_root(&b, base), BigUint::from(native::mult_digital_root(i, base)));
        prop_assert_eq!(digits::is_harshad(&b, base), native::is_harshad(i, base));
        prop_assert_eq!(digits::is_palindrome(&b, base), native::is_palindrome(i, base));
        prop_assert_eq!(digits::is_happy(&b, u32::try_from(base).unwrap(), exponent), native::is_happy(n, u32::try_from(base).unwrap(), exponent));
        let z = native::tozeckendorf(n);
        prop_assert_eq!(digits::tozeckendorf(&b), z.clone());
        prop_assert_eq!(digits::fromzeckendorf(&z), native::fromzeckendorf(&z).map(BigUint::from));
        prop_assert_eq!(digits::toint(&n.to_string()), native::toint(&n.to_string()).map(|v| BigUint::try_from(v).unwrap()));
    }

    #[test]
    fn bigint_digits_wide_round_trip(bytes in prop::collection::vec(any::<u8>(), 9..129), base in 2_u64..=36) {
        let n = BigUint::from_bytes_le(&bytes);
        prop_assert_eq!(digits::fromdigits(&digits::todigits(&n, base, None), base), Some(n.clone()));
        prop_assert_eq!(digits::fromzeckendorf(&digits::tozeckendorf(&n)), Some(n.clone()));
        prop_assert_eq!(digits::toint(&n.to_string()), Some(n));
    }
}

#[test]
fn bigint_digits_validation_and_decimal_forms() {
    for (text, expected) in [
        ("", 0),
        (" -0.99 ", 0),
        ("+12_345", 12345),
        ("1.2e1", 12),
        ("0e100", 0),
        ("1e-100", 0),
        ("0Xf_f", 255),
        ("0O77", 63),
        ("0B11", 3),
        ("42E+2", 4200),
    ] {
        assert_eq!(digits::toint(text), Some(BigUint::from(expected as u32)));
    }
    assert_eq!(digits::toint("-1"), None);
    for text in [
        " ", "_1", "1_", "1__2", "0x", "0b2", "1e", "1e2e3", "1.2.3", ".", "abc",
    ] {
        assert!(
            std::panic::catch_unwind(|| digits::toint(text)).is_err(),
            "{text}"
        );
    }
    for text in ["", "00", "01", "11", "10x"] {
        assert!(std::panic::catch_unwind(|| digits::fromzeckendorf(text)).is_err());
    }
    let n = BigUint::from(19_u32);
    assert!(std::panic::catch_unwind(|| digits::todigits(&n, 1, None)).is_err());
    assert!(std::panic::catch_unwind(|| digits::todigitstring(&n, 37, None)).is_err());
    assert!(std::panic::catch_unwind(|| digits::fromdigits(&[2], 2)).is_err());
    assert!(std::panic::catch_unwind(|| digits::fromdigits(&[], 1)).is_err());
    assert!(std::panic::catch_unwind(|| digits::digital_root(&n, 0)).is_err());
    assert!(std::panic::catch_unwind(|| digits::mult_digital_root(&n, 0)).is_err());
    assert!(std::panic::catch_unwind(|| digits::is_happy(&n, 1, 2)).is_err());
    assert!(std::panic::catch_unwind(|| digits::is_happy(&n, 10, 11)).is_err());
    assert_eq!(digits::is_happy(&n, 36, 10), native::is_happy(19, 36, 10));
}
