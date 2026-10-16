//! Run-time configuration and platform introspection.
//!
//! Ports `prime_get_config` and `prime_set_config` from
//! `reference/mpu/lib/Math/Prime/Util.pm` (MPU 0.75), and the constants that
//! `XS.xs` installs in its `BOOT` block (`_uvsize`, `_uvbits`, `_ivsize`,
//! `_nvsize`, `_nvmantbits`, `_nvmantdigits`).

use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// The settings that MPU keeps in its `%_Config` hash and that can be read
/// and changed from Rust.
///
/// Read the current values with [`prime_get_config`]. Change them with
/// [`prime_set_config`].
///
/// # Examples
///
/// ```
/// let config = math_prime_util::prime_get_config();
/// assert_eq!(config.maxbits, 64);
/// assert_eq!(config.verbose, 0);
/// assert!(!config.assume_rh);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    /// The widest input the native path accepts, in bits (`maxbits` in MPU).
    /// Starts at the width of a native word.
    pub maxbits: u32,
    /// The verbosity level for progress output (`verbose` in MPU).
    pub verbose: u32,
    /// When `true`, functions that depend on the Riemann hypothesis may
    /// assume it (`assume_rh` in MPU).
    pub assume_rh: bool,
}

impl Config {
    /// The defaults that MPU sets at load time.
    ///
    /// `maxbits` starts at the width of a native word (see [`_uvbits`]),
    /// `verbose` at 0 and `assume_rh` at `false`.
    pub const DEFAULT: Self = Self {
        maxbits: _uvbits(),
        verbose: 0,
        assume_rh: false,
    };
}

impl Default for Config {
    /// Returns [`Config::DEFAULT`].
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The live settings. Writers take the lock for the whole update, so a
/// reader never sees a half-applied [`prime_set_config`] call.
static CONFIG: RwLock<Config> = RwLock::new(Config::DEFAULT);

/// Reads the lock. A panic in another thread does not stop reads: the
/// stored value is a plain `Copy` struct, so it is never left half-written.
fn read_lock() -> RwLockReadGuard<'static, Config> {
    CONFIG.read().unwrap_or_else(PoisonError::into_inner)
}

/// Writes the lock. A poisoned lock is recovered for the same reason as in
/// [`read_lock`].
fn write_lock() -> RwLockWriteGuard<'static, Config> {
    CONFIG.write().unwrap_or_else(PoisonError::into_inner)
}

/// Returns a copy of the current configuration.
///
/// Port of `prime_get_config` in `reference/mpu/lib/Math/Prime/Util.pm`.
/// MPU returns a hash. The Rust version returns a [`Config`] value, so the
/// caller can read the fields by name.
///
/// # Examples
///
/// ```
/// let config = math_prime_util::prime_get_config();
/// assert!(config.maxbits >= 32);
/// ```
#[must_use]
pub fn prime_get_config() -> Config {
    *read_lock()
}

/// Changes one or more settings and returns the new configuration.
///
/// Port of `prime_set_config` in `reference/mpu/lib/Math/Prime/Util.pm`.
/// Each entry is a key and a value. Keys are matched without regard to case,
/// as MPU does with `lc`. The valid keys are:
///
/// - `maxbits`: an integer from 1 to [`_uvbits`]. A larger value is an error.
/// - `verbose`: an integer that fits in `u32`.
/// - `assume_rh`: `0` for `false`, any other value for `true`.
///
/// The update is all or nothing. If any entry is invalid, no setting
/// changes and the error names the bad key.
///
/// # Errors
///
/// Returns [`ConfigError::UnknownKey`] for a key that is not listed above.
/// Returns [`ConfigError::BadValue`] for a value out of range for its key.
///
/// # Examples
///
/// ```
/// let before = math_prime_util::prime_get_config();
/// let after = math_prime_util::prime_set_config(&[("verbose", 2)]).unwrap();
/// assert_eq!(after.verbose, 2);
/// assert_eq!(after.maxbits, before.maxbits);
/// math_prime_util::prime_set_config(&[("verbose", 0)]).unwrap();
/// ```
pub fn prime_set_config(params: &[(&str, u64)]) -> Result<Config, ConfigError> {
    // Hold the write lock while validating, so the check and the update see the same state.
    let mut config = write_lock();
    let mut next = *config;
    for &(key, value) in params {
        match key.to_ascii_lowercase().as_str() {
            "maxbits" => {
                next.maxbits = u32::try_from(value)
                    .ok()
                    .filter(|&bits| (1..=_uvbits()).contains(&bits))
                    .ok_or(ConfigError::BadValue { key: "maxbits" })?;
            }
            "verbose" => {
                next.verbose =
                    u32::try_from(value).map_err(|_| ConfigError::BadValue { key: "verbose" })?;
            }
            "assume_rh" => next.assume_rh = value != 0,
            _ => return Err(ConfigError::UnknownKey { key: key.to_owned() }),
        }
    }
    *config = next;
    Ok(next)
}

/// An error from [`prime_set_config`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigError {
    /// The key is not a setting that [`prime_set_config`] knows.
    UnknownKey {
        /// The key as the caller wrote it.
        key: String,
    },
    /// The value is out of range for its key.
    BadValue {
        /// The key whose value is out of range (lower case).
        key: &'static str,
    },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownKey { key } => write!(f, "prime_set_config: unknown key '{key}'"),
            Self::BadValue { key } => write!(f, "prime_set_config: bad value for '{key}'"),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Size of a native unsigned integer (UV), in bytes.
///
/// Port of `_uvsize` in `reference/mpu/XS.xs` (`UVSIZE`, set in `BOOT`).
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::_uvsize(), std::mem::size_of::<u64>() as u32);
/// ```
#[must_use]
pub const fn _uvsize() -> u32 {
    8
}

/// Width of a native unsigned integer (UV), in bits.
///
/// Port of `_uvbits` in `reference/mpu/XS.xs` (`UVSIZE * 8`).
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::_uvbits(), 64);
/// ```
#[must_use]
pub const fn _uvbits() -> u32 {
    _uvsize() * 8
}

/// Size of a native signed integer (IV), in bytes.
///
/// Port of `_ivsize` in `reference/mpu/XS.xs` (`IVSIZE`, set in `BOOT`).
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::_ivsize(), 8);
/// ```
#[must_use]
pub const fn _ivsize() -> u32 {
    8
}

/// Size of a native floating-point number (NV), in bytes.
///
/// Port of `_nvsize` in `reference/mpu/XS.xs` (`NVSIZE`, set in `BOOT`).
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::_nvsize(), 8);
/// ```
#[must_use]
pub const fn _nvsize() -> u32 {
    8
}

/// Number of stored mantissa bits in a native floating-point number (NV).
///
/// Port of `_nvmantbits` in `reference/mpu/XS.xs` (`NVMANTBITS`, set in
/// `BOOT`). An IEEE 754 double stores 52 bits and has one more implicit bit.
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::_nvmantbits(), 53);
/// ```
#[must_use]
pub const fn _nvmantbits() -> u32 {
    53
}

/// Number of decimal digits that a native floating-point number (NV) holds
/// exactly.
///
/// Port of `_nvmantdigits` in `reference/mpu/XS.xs`:
/// `(NVMANTBITS + 1) / 3.322`, truncated to an integer.
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::_nvmantdigits(), 16);
/// ```
#[must_use]
pub const fn _nvmantdigits() -> u32 {
    // Integer form of (NVMANTBITS + 1) / 3.322 for NVMANTBITS = 53: 54 * 1000 / 3322 = 16.
    (_nvmantbits() + 1) * 1000 / 3322
}
