//! Algebraic laws for the config and platform functions in `src/config`.

use std::sync::Mutex;

use math_prime_util::{prime_get_config, prime_set_config, ConfigError, _nvmantbits, _nvmantdigits, _uvbits, _uvsize};

/// Serializes the tests that change the global configuration.
static LOCK: Mutex<()> = Mutex::new(());

#[test]
fn prop_int_set_then_get_round_trips() {
    let _guard = LOCK.lock().unwrap();
    for verbose in 0..=5_u64 {
        for rh in [0_u64, 1] {
            let set = prime_set_config(&[("verbose", verbose), ("assume_rh", rh)]).unwrap();
            assert_eq!(prime_get_config(), set);
            assert_eq!(set.verbose, verbose as u32);
            assert_eq!(set.assume_rh, rh != 0);
        }
    }
    prime_set_config(&[("verbose", 0), ("assume_rh", 0)]).unwrap();
}

#[test]
fn prop_int_set_is_idempotent() {
    let _guard = LOCK.lock().unwrap();
    let once = prime_set_config(&[("verbose", 2)]).unwrap();
    let twice = prime_set_config(&[("verbose", 2)]).unwrap();
    assert_eq!(once, twice);
    prime_set_config(&[("verbose", 0)]).unwrap();
}

#[test]
fn prop_int_failed_set_changes_nothing() {
    // The valid entry comes first. A set that applied entries one by one would
    // leave verbose changed after the bad entry fails.
    let _guard = LOCK.lock().unwrap();
    let before = prime_get_config();
    let err = prime_set_config(&[("verbose", 4), ("maxbits", 0)]).unwrap_err();
    assert_eq!(err, ConfigError::BadValue { key: "maxbits" });
    assert_eq!(prime_get_config(), before);
}

#[test]
fn prop_int_maxbits_stays_in_native_width() {
    let _guard = LOCK.lock().unwrap();
    let before = prime_get_config();
    for bits in [1_u64, 32, u64::from(_uvbits())] {
        assert_eq!(prime_set_config(&[("maxbits", bits)]).unwrap().maxbits, bits as u32);
    }
    assert!(prime_set_config(&[("maxbits", u64::from(_uvbits()) + 1)]).is_err());
    prime_set_config(&[("maxbits", u64::from(before.maxbits))]).unwrap();
}

#[test]
fn prop_int_uvbits_is_eight_times_uvsize() {
    assert_eq!(_uvbits(), 8 * _uvsize());
}

#[test]
fn prop_int_mantissa_digits_fit_mantissa_bits() {
    // Decimal digits that are exact must need fewer bits than the mantissa holds.
    assert!(2_u64.pow(_nvmantdigits()) <= 1_u64 << _nvmantbits());
    assert!(_nvmantdigits() < _nvmantbits());
}
