//! Modular arithmetic on `u64`: `addmod`, `submod`, `mulmod`, `powmod`,
//! `invmod` and friends. Ported from MPU's `mulmod.h` and `util.c`
//! (`modinverse`).
//!
//! All functions take the modulus last, as MPU does. A modulus of `0` makes
//! MPU return `undef`, so these return [`Option`]; `n == 1` gives `Some(0)`.
//! Inputs need not be reduced.

/// `(a + b) mod n`, or `None` if `n == 0`.
///
/// ```
/// use math_prime_util::addmod;
/// assert_eq!(addmod(u64::MAX, u64::MAX, u64::MAX - 1), Some(2));
/// assert_eq!(addmod(1, 2, 0), None);
/// ```
#[inline]
pub fn addmod(a: u64, b: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    Some(addmod_u(a % n, b % n, n))
}

/// `(a + b) mod n` for reduced `a, b < n`, `n > 0`. Kruppa's branch-free form.
#[inline]
pub(crate) fn addmod_u(a: u64, b: u64, n: u64) -> u64 {
    if n - a > b {
        a + b
    } else {
        a.wrapping_add(b).wrapping_sub(n)
    }
}

/// `(a - b) mod n`, or `None` if `n == 0`.
///
/// ```
/// use math_prime_util::submod;
/// assert_eq!(submod(3, 5, 7), Some(5));
/// ```
#[inline]
pub fn submod(a: u64, b: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let (a, b) = (a % n, b % n);
    Some(addmod_u(a, n - b, n))
}

/// `(a * b) mod n`, or `None` if `n == 0`.
///
/// ```
/// use math_prime_util::mulmod;
/// assert_eq!(mulmod(u64::MAX, u64::MAX, u64::MAX - 1), Some(1));
/// ```
#[inline]
pub fn mulmod(a: u64, b: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    Some(mulmod_u(a, b, n))
}

/// `(a * b) mod n` for `n > 0`. Uses the native 64×64→128 multiply.
#[inline]
pub(crate) fn mulmod_u(a: u64, b: u64, n: u64) -> u64 {
    #[allow(clippy::cast_possible_truncation)]
    {
        ((u128::from(a) * u128::from(b)) % u128::from(n)) as u64
    }
}

/// `(a * a) mod n`, or `None` if `n == 0`.
#[inline]
pub fn sqrmod(a: u64, n: u64) -> Option<u64> {
    mulmod(a, a, n)
}

/// `(a * b + c) mod n`, or `None` if `n == 0`.
///
/// ```
/// use math_prime_util::muladdmod;
/// assert_eq!(muladdmod(3, 4, 5, 7), Some(3));
/// ```
#[inline]
pub fn muladdmod(a: u64, b: u64, c: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    Some(addmod_u(mulmod_u(a, b, n), c % n, n))
}

/// `(a * b - c) mod n`, or `None` if `n == 0`.
///
/// ```
/// use math_prime_util::mulsubmod;
/// assert_eq!(mulsubmod(3, 4, 5, 7), Some(0));
/// ```
#[inline]
pub fn mulsubmod(a: u64, b: u64, c: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let c = c % n;
    Some(addmod_u(mulmod_u(a, b, n), n - c, n))
}

/// `a^k mod n`, or `None` if `n == 0`. `powmod(0, 0, n) == Some(1 % n)`.
///
/// ```
/// use math_prime_util::powmod;
/// assert_eq!(powmod(2, 62, u64::MAX), Some(1 << 62));
/// assert_eq!(powmod(3, 1000, 1000000007), Some(56888193));
/// ```
#[inline]
pub fn powmod(a: u64, k: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    Some(powmod_u(a, k, n))
}

/// `a^k mod n` for `n > 0`.
pub(crate) fn powmod_u(mut a: u64, mut k: u64, n: u64) -> u64 {
    let mut t = 1 % n;
    a %= n;
    if n < (1 << 32) {
        while k != 0 {
            if k & 1 != 0 {
                t = (t * a) % n;
            }
            k >>= 1;
            if k != 0 {
                a = (a * a) % n;
            }
        }
    } else {
        while k != 0 {
            if k & 1 != 0 {
                t = mulmod_u(t, a, n);
            }
            k >>= 1;
            if k != 0 {
                a = mulmod_u(a, a, n);
            }
        }
    }
    t
}

/// `-a mod n`, or `None` if `n == 0`.
///
/// ```
/// use math_prime_util::negmod;
/// assert_eq!(negmod(100, 123), Some(23));
/// assert_eq!(negmod(0, 1), Some(0));
/// assert_eq!(negmod(1, 0), None);
/// ```
#[inline]
pub fn negmod(a: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let a = a % n;
    Some(if a == 0 { 0 } else { n - a })
}

/// Modular inverse: the `x < n` with `a * x ≡ 1 (mod n)`, or `None` if no
/// inverse exists (including `n == 0`). `invmod(0, 1) == Some(0)`.
///
/// ```
/// use math_prime_util::invmod;
/// assert_eq!(invmod(45, 59), Some(21));
/// assert_eq!(invmod(14, 28474), None);
/// assert_eq!(invmod(14, u64::MAX), Some(17129119497016012214));
/// ```
pub fn invmod(a: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(0);
    }
    // MPU modinverse: extended Euclid with the coefficient kept as a signed
    // value; done here with wrapping arithmetic on u64 and a sign fix-up,
    // which is exact because |t| < n at every step.
    let (mut t, mut nt) = (0u64, 1u64);
    let (mut r, mut nr) = (n, a % n);
    let mut t_neg = false;
    let mut nt_neg = false;
    while nr != 0 {
        let q = r / nr;
        // (t, nt) = (nt, t - q*nt) in signed arithmetic.
        let (next, next_neg) = signed_sub(t, t_neg, q.wrapping_mul(nt), nt_neg);
        (t, t_neg, nt, nt_neg) = (nt, nt_neg, next, next_neg);
        (r, nr) = (nr, r - q * nr);
    }
    if r > 1 {
        return None;
    }
    Some(if t_neg { n - t } else { t })
}

/// Signed subtraction `(x - y)` on sign-magnitude pairs with magnitudes `< 2^64`.
#[inline]
fn signed_sub(x: u64, xneg: bool, y: u64, yneg: bool) -> (u64, bool) {
    match (xneg, yneg) {
        (false, true) => (x + y, false),
        (true, false) => (x + y, true),
        (false, false) => {
            if x >= y {
                (x - y, false)
            } else {
                (y - x, true)
            }
        }
        (true, true) => {
            if y >= x {
                (y - x, false)
            } else {
                (x - y, true)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invmod_roundtrip() {
        for n in [
            2u64,
            3,
            59,
            2017,
            65_537,
            1 << 32,
            (1 << 61) - 1,
            u64::MAX - 58,
            u64::MAX,
        ] {
            for a in [
                1u64,
                2,
                3,
                13,
                14,
                45,
                12_345,
                u64::from(u32::MAX),
                u64::MAX - 1,
            ] {
                match invmod(a, n) {
                    Some(x) => assert_eq!(mulmod(a, x, n), Some(1 % n), "a={a} n={n} x={x}"),
                    None => assert_ne!(crate::gcd(a, n), 1, "a={a} n={n}"),
                }
            }
        }
    }

    #[test]
    fn fermat() {
        let p = (1u64 << 61) - 1;
        for a in [2u64, 3, 1_234_567, u64::from(u32::MAX), p - 1] {
            assert_eq!(powmod(a, p - 1, p), Some(1));
        }
    }
}

mod native;
mod binomialmod;
pub use binomialmod::binomialmod;
mod factorialmod;
pub use factorialmod::factorialmod;
mod pisano_period;
pub use pisano_period::pisano_period;
mod znlog;
pub use znlog::znlog;
