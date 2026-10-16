//! Compatibility checks for the config and platform functions in `src/config`.
//!
//! The `int` fixture family has no cases for these functions, so the checks
//! compare against the values MPU 0.75 installs in `XS.xs` and against the
//! Rust type sizes.

use std::sync::Mutex;

use math_prime_util::{
    prime_get_config, prime_set_config, ConfigError, _ivsize, _nvmantbits, _nvmantdigits,
    _nvsize, _uvbits, _uvsize,
};

/// Serializes the tests that change the global configuration.
static LOCK: Mutex<()> = Mutex::new(());

#[test]
fn compat_int_uvsize() {
    assert_eq!(_uvsize(), u64::BITS / 8);
    assert_eq!(_uvsize(), std::mem::size_of::<u64>() as u32);
}

#[test]
fn compat_int_uvbits() {
    assert_eq!(_uvbits(), 64);
    assert_eq!(_uvbits(), _uvsize() * 8);
}

#[test]
fn compat_int_ivsize() {
    assert_eq!(_ivsize(), std::mem::size_of::<i64>() as u32);
}

#[test]
fn compat_int_nvsize() {
    assert_eq!(_nvsize(), std::mem::size_of::<f64>() as u32);
}

#[test]
fn compat_int_nvmantbits() {
    assert_eq!(_nvmantbits(), f64::MANTISSA_DIGITS);
}

#[test]
fn compat_int_nvmantdigits() {
    // XS.xs: (NVMANTBITS + 1) / 3.322 truncated, with NVMANTBITS = 53.
    assert_eq!(_nvmantdigits(), 16);
}

#[test]
fn compat_int_prime_get_config() {
    let _guard = LOCK.lock().unwrap();
    let config = prime_get_config();
    assert_eq!(config.maxbits, _uvbits());
    assert_eq!(config.verbose, 0);
    assert!(!config.assume_rh);
}

#[test]
fn compat_int_prime_set_config() {
    let _guard = LOCK.lock().unwrap();
    let before = prime_get_config();
    let after = prime_set_config(&[("VERBOSE", 3), ("assume_rh", 1)]).unwrap();
    assert_eq!(after.verbose, 3);
    assert!(after.assume_rh);
    assert_eq!(prime_get_config(), after);
    prime_set_config(&[("verbose", before.verbose.into()), ("assume_rh", 0)]).unwrap();
}

#[test]
fn compat_int_prime_set_config_rejects_unknown_key() {
    let _guard = LOCK.lock().unwrap();
    let err = prime_set_config(&[("no_such_key", 1)]).unwrap_err();
    assert_eq!(
        err,
        ConfigError::UnknownKey {
            key: "no_such_key".to_owned()
        }
    );
}
