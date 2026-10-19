//! Generic unsigned integer operations.
//!
//! Use [`Int`] methods to keep the native free functions unambiguous.
//! The backends are `u64`, `u128`, with feature `bigint` `num_bigint::BigUint`
//! (pure Rust), and with feature `gmp` `rug::Integer` (links the system GMP).

#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod arith;
#[cfg(feature = "bigint")]
mod biguint;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod counts;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod digits;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod divisors;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod factor;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod family_counts;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod functions;
#[cfg(feature = "gmp")]
mod gmp;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod modular;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod predicates;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod primality;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod proving;
#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub mod rational;

/// The shared number theory surface for unsigned integers.
///
/// ```
/// use math_prime_util::Int;
/// fn coprime<T: Int + PartialEq + From<u8>>(a: &T, b: &T) -> bool {
///     a.gcd(b) == T::from(1)
/// }
/// assert!(coprime(&35_u128, &64));
/// ```
pub trait Int: Sized {
    /// Returns 0 for composite, 1 for probable prime, or 2 for proven prime.
    ///
    /// Uses MPU 0.75 `primality.c::is_prime` and `factor128.c::is_prime128`.
    /// The latter uses the BPSW test, as does `PP.pm::_is_prime7`.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(97_u64.is_prime(), 2);
    /// assert_eq!(((1_u128 << 127) - 1).is_prime(), 1);
    /// ```
    fn is_prime(&self) -> u8;

    /// Returns sorted prime factors. Zero returns `[0]`; one returns `[]`.
    ///
    /// Uses MPU 0.75 `factor.c::factor` and `factor128.c::factorintp128`.
    ///
    /// # Errors
    /// Returns an unsplit cofactor if the factoring stages fail.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(360_u128.factor(), Ok(vec![2, 2, 2, 3, 3, 5]));
    /// ```
    fn factor(&self) -> Result<Vec<Self>, Self>;

    /// Returns `self^exponent mod modulus`, or `None` for modulus zero.
    ///
    /// Uses binary exponentiation from MPU 0.75 `mulmod.h::powmod`.
    /// This is the unsigned form of `PP.pm::powmod`.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(3_u128.powmod(&5, &7), Some(5));
    /// assert_eq!(0_u64.powmod(&0, &1), Some(0));
    /// ```
    fn powmod(&self, exponent: &Self, modulus: &Self) -> Option<Self>;

    /// Returns the greatest common divisor. `gcd(0, 0)` is zero.
    ///
    /// Uses the Euclidean loop in MPU 0.75 `util.h::gcd_ui` and `PP.pm::gcd`.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(12_u128.gcd(&18), 6);
    /// ```
    #[must_use]
    fn gcd(&self, other: &Self) -> Self;

    /// Returns the next prime, or `None` if it exceeds the integer range.
    ///
    /// Uses MPU 0.75 `util.c::next_prime` and the wheel in `PP.pm::next_prime`.
    /// Above the `u64` range, candidates pass a probable prime test.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(14_u128.next_prime(), Some(17));
    /// assert_eq!(u128::MAX.next_prime(), None);
    /// ```
    fn next_prime(&self) -> Option<Self>;

    /// Returns `self + other`, or `None` on overflow of a fixed-width type.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(2_u64.add_int(&3), Some(5));
    /// assert_eq!(u64::MAX.add_int(&1), None);
    /// ```
    fn add_int(&self, other: &Self) -> Option<Self>;

    /// Returns `self - other`, or `None` if the result is negative.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(5_u128.sub_int(&3), Some(2));
    /// assert_eq!(3_u128.sub_int(&5), None);
    /// ```
    fn sub_int(&self, other: &Self) -> Option<Self>;

    /// Returns `self * other`, or `None` on overflow of a fixed-width type.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(6_u64.mul_int(&7), Some(42));
    /// assert_eq!(u64::MAX.mul_int(&2), None);
    /// ```
    fn mul_int(&self, other: &Self) -> Option<Self>;

    /// Returns quotient and remainder, or `None` for a zero divisor.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(17_u64.divrem(&5), Some((3, 2)));
    /// assert_eq!(17_u64.divrem(&0), None);
    /// ```
    fn divrem(&self, divisor: &Self) -> Option<(Self, Self)>;

    /// Returns `self * other mod modulus`, or `None` for modulus zero.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(7_u128.mulmod(&8, &10), Some(6));
    /// assert_eq!(7_u128.mulmod(&8, &0), None);
    /// ```
    fn mulmod(&self, other: &Self, modulus: &Self) -> Option<Self>;

    /// Returns `self << bits`, or `None` if bits are lost in a fixed-width type.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(3_u64.shl_bits(4), Some(48));
    /// assert_eq!(1_u64.shl_bits(64), None);
    /// ```
    fn shl_bits(&self, bits: u32) -> Option<Self>;

    /// Returns `self >> bits`. Shifting by the full width or more gives zero.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(48_u64.shr_bits(4), 3);
    /// assert_eq!(48_u64.shr_bits(70), 0);
    /// ```
    #[must_use]
    fn shr_bits(&self, bits: u32) -> Self;

    /// Returns the number of bits needed to write the value. Zero has length 0.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(255_u64.bit_length(), 8);
    /// assert_eq!(0_u128.bit_length(), 0);
    /// ```
    fn bit_length(&self) -> u64;

    /// Returns true if the value is even.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert!(10_u64.is_even());
    /// assert!(!7_u128.is_even());
    /// ```
    fn is_even(&self) -> bool;

    /// Converts from `u64`.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(<u128 as Int>::from_u64(9), 9);
    /// ```
    fn from_u64(value: u64) -> Self;

    /// Converts to `u64`, or `None` if the value is too large.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(9_u128.to_u64(), Some(9));
    /// assert_eq!(u128::MAX.to_u64(), None);
    /// ```
    fn to_u64(&self) -> Option<u64>;

    /// Parses a decimal string of digits. Returns `None` for bad input or overflow.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(<u64 as Int>::from_decimal("123"), Some(123));
    /// assert_eq!(<u64 as Int>::from_decimal("12x"), None);
    /// assert_eq!(<u64 as Int>::from_decimal("99999999999999999999"), None);
    /// ```
    fn from_decimal(text: &str) -> Option<Self>;

    /// Prints the value in decimal.
    ///
    /// ```
    /// use math_prime_util::Int;
    /// assert_eq!(1234_u64.to_decimal(), "1234");
    /// ```
    fn to_decimal(&self) -> String;
}

macro_rules! native_ops {
    ($t:ty, $narrow:expr) => {
        fn add_int(&self, other: &Self) -> Option<Self> {
            self.checked_add(*other)
        }

        fn sub_int(&self, other: &Self) -> Option<Self> {
            self.checked_sub(*other)
        }

        fn mul_int(&self, other: &Self) -> Option<Self> {
            self.checked_mul(*other)
        }

        fn divrem(&self, divisor: &Self) -> Option<(Self, Self)> {
            Some((self.checked_div(*divisor)?, self.checked_rem(*divisor)?))
        }

        fn mulmod(&self, other: &Self, modulus: &Self) -> Option<Self> {
            if *modulus == 0 {
                return None;
            }
            // Operands must be smaller than the modulus: `addmod` subtracts from it.
            let n = u128::from(*modulus);
            let a = u128::from(*self % *modulus);
            let b = u128::from(*other % *modulus);
            // The result is below the modulus, so it fits the narrow type.
            $narrow(mulmod(a, b, n))
        }

        fn shl_bits(&self, bits: u32) -> Option<Self> {
            let shifted = self.checked_shl(bits)?;
            (shifted >> bits == *self).then_some(shifted)
        }

        fn shr_bits(&self, bits: u32) -> Self {
            self.checked_shr(bits).unwrap_or(0)
        }

        fn bit_length(&self) -> u64 {
            u64::from(<$t>::BITS - self.leading_zeros())
        }

        fn is_even(&self) -> bool {
            self & 1 == 0
        }

        fn from_decimal(text: &str) -> Option<Self> {
            if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            text.parse().ok()
        }

        fn to_decimal(&self) -> String {
            self.to_string()
        }
    };
}

impl Int for u64 {
    native_ops!(u64, |v: u128| u64::try_from(v).ok());

    fn from_u64(value: u64) -> Self {
        value
    }

    fn to_u64(&self) -> Option<u64> {
        Some(*self)
    }

    fn is_prime(&self) -> u8 {
        crate::primality::is_prime(*self)
    }

    fn factor(&self) -> Result<Vec<Self>, Self> {
        Ok(crate::factor::factor(*self))
    }

    fn powmod(&self, exponent: &Self, modulus: &Self) -> Option<Self> {
        crate::modular::powmod(*self, *exponent, *modulus)
    }

    fn gcd(&self, other: &Self) -> Self {
        crate::int::gcd(*self, *other)
    }

    fn next_prime(&self) -> Option<Self> {
        crate::sieve::next_prime(*self)
    }
}

// Reduced operands let subtraction replace an overflowing addition.
fn addmod(a: u128, b: u128, n: u128) -> u128 {
    if a >= n - b { a - (n - b) } else { a + b }
}

fn mulmod(mut a: u128, mut b: u128, n: u128) -> u128 {
    if let Some(product) = a.checked_mul(b) {
        return product % n;
    }
    let mut result = 0;
    while b != 0 {
        if b & 1 != 0 {
            result = addmod(result, a, n);
        }
        b >>= 1;
        a = addmod(a, a, n);
    }
    result
}

impl Int for u128 {
    native_ops!(u128, |v: u128| Some(v));

    fn from_u64(value: u64) -> Self {
        Self::from(value)
    }

    fn to_u64(&self) -> Option<u64> {
        u64::try_from(*self).ok()
    }

    fn is_prime(&self) -> u8 {
        crate::factor::is_prime_u128(*self)
    }

    fn factor(&self) -> Result<Vec<Self>, Self> {
        crate::factor::factor_u128(*self)
    }

    fn powmod(&self, exponent: &Self, modulus: &Self) -> Option<Self> {
        let n = *modulus;
        if n == 0 {
            return None;
        }
        let mut a = self % n;
        let mut b = *exponent;
        let mut result = 1 % n;
        while b != 0 {
            if b & 1 != 0 {
                result = mulmod(result, a, n);
            }
            b >>= 1;
            if b != 0 {
                a = mulmod(a, a, n);
            }
        }
        Some(result)
    }

    fn gcd(&self, other: &Self) -> Self {
        let (mut a, mut b) = (*self, *other);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    fn next_prime(&self) -> Option<Self> {
        const ADVANCE: [u128; 30] = [
            1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1,
            2,
        ];
        if *self < 7 {
            return Some(match *self {
                0 | 1 => 2,
                2 => 3,
                3 | 4 => 5,
                _ => 7,
            });
        }
        let mut n = *self;
        loop {
            n = n.checked_add(ADVANCE[usize::try_from(n % 30).ok()?])?;
            if n % 7 != 0 && n.is_prime() != 0 {
                return Some(n);
            }
        }
    }
}
