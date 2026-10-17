//! Generic unsigned integer operations.
//!
//! Use [`Int`] methods to keep the native free functions unambiguous.
//! The current backends are `u64` and `u128`.

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
    /// Uses the Euclidean loop in MPU 0.75 `PP.pm::gcd`.
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
}

impl Int for u64 {
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
