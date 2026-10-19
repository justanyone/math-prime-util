//! The integer core on arbitrary precision values: the functions of
//! [`crate::int`], [`crate::modular`], [`crate::combinat`] and
//! [`crate::arith`] with the same names, for [`BigUint`].
//!
//! Signed inputs and results (`gcdext`, `jacobi`, `kronecker`) use
//! [`BigInt`], as the native functions use `i64`. A modulus of zero gives
//! `None`, as on the native path. Where a native function panics on an
//! invalid argument, so does the arbitrary precision one.
//!
//! ```
//! use math_prime_util::bigint::arith::{gcd, powmod};
//! use num_bigint::BigUint;
//! let m = (BigUint::from(1_u8) << 127_usize) - 1_u8;
//! assert_eq!(gcd(&(&m * 6_u8), &(&m * 4_u8)), &m * 2_u8);
//! assert_eq!(powmod(&BigUint::from(2_u8), &(&m - 1_u8), &m), Some(BigUint::from(1_u8)));
//! ```
use super::Int;
use num_bigint::{BigInt, BigUint, Sign};

fn zero() -> BigUint {
    BigUint::from(0_u8)
}

fn one() -> BigUint {
    BigUint::from(1_u8)
}

fn is_zero(n: &BigUint) -> bool {
    n.bits() == 0
}

/// Greatest common divisor. `gcd(0, 0)` is zero.
///
/// Uses the Euclidean loop of MPU 0.75 `PP.pm::gcd` through [`Int::gcd`].
///
/// ```
/// use math_prime_util::bigint::arith::gcd;
/// use num_bigint::BigUint;
/// assert_eq!(gcd(&BigUint::from(12_u8), &BigUint::from(18_u8)), BigUint::from(6_u8));
/// ```
pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    Int::gcd(a, b)
}

/// Least common multiple. `lcm(0, b)` is zero. Never overflows, so there is
/// no `Option` as on the native path.
///
/// Port of MPU 0.75 `PP.pm::lcm`.
///
/// ```
/// use math_prime_util::bigint::arith::lcm;
/// use num_bigint::BigUint;
/// assert_eq!(lcm(&BigUint::from(4_u8), &BigUint::from(6_u8)), BigUint::from(12_u8));
/// assert_eq!(lcm(&BigUint::from(0_u8), &BigUint::from(6_u8)), BigUint::from(0_u8));
/// ```
pub fn lcm(a: &BigUint, b: &BigUint) -> BigUint {
    if is_zero(a) || is_zero(b) {
        return zero();
    }
    a / gcd(a, b) * b
}

/// Extended Euclidean algorithm: `(g, u, v)` with `g = gcd(a, b)` and
/// `a*u + b*v == g`, the same coefficients as the native `gcdext`.
///
/// Port of MPU 0.75 `PP.pm::gcdext` with truncated division, so the signed
/// results agree with [`crate::gcdext`] on every `i64` input.
///
/// ```
/// use math_prime_util::bigint::arith::gcdext;
/// use num_bigint::BigInt;
/// let (g, u, v) = gcdext(&BigInt::from(240), &BigInt::from(46));
/// assert_eq!((g, u, v), (BigInt::from(2), BigInt::from(-9), BigInt::from(47)));
/// ```
pub fn gcdext(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    // (s, t, r) are the running coefficients and remainder; (u, v, g) the
    // previous ones, which hold the answer when r reaches 0.
    let (mut s, mut u) = (BigInt::from(0), BigInt::from(1));
    let (mut t, mut v) = (BigInt::from(1), BigInt::from(0));
    let (mut r, mut g) = (b.clone(), a.clone());
    if a.sign() == Sign::NoSign && b.sign() == Sign::NoSign {
        u = BigInt::from(0);
        t = BigInt::from(0);
    }
    while r.sign() != Sign::NoSign {
        let q = &g / &r;
        (g, r) = (r.clone(), g - &q * &r);
        (u, s) = (s.clone(), u - &q * &s);
        (v, t) = (t.clone(), v - &q * &t);
    }
    if g.sign() == Sign::Minus {
        (g, u, v) = (-g, -u, -v);
    }
    (g, u, v)
}

/// Modular inverse: the `x < n` with `a * x ≡ 1 (mod n)`, or `None` if no
/// inverse exists (including `n == 0`). `invmod(a, 1)` is `Some(0)`.
///
/// Port of MPU 0.75 `PP.pm::invmod` through [`gcdext`].
///
/// ```
/// use math_prime_util::bigint::arith::invmod;
/// use num_bigint::BigUint;
/// assert_eq!(invmod(&BigUint::from(45_u8), &BigUint::from(59_u8)), Some(BigUint::from(21_u8)));
/// assert_eq!(invmod(&BigUint::from(14_u8), &BigUint::from(28_u8)), None);
/// ```
pub fn invmod(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    if is_zero(n) {
        return None;
    }
    if *n == one() {
        return Some(zero());
    }
    let (g, u, _) = gcdext(&BigInt::from(a % n), &BigInt::from(n.clone()));
    if g != BigInt::from(1) {
        return None;
    }
    let (sign, magnitude) = u.into_parts();
    Some(match sign {
        Sign::Minus => n - magnitude % n,
        Sign::NoSign | Sign::Plus => magnitude % n,
    })
}

/// `a^k mod n`, or `None` if `n == 0`. `powmod(0, 0, n)` is `1 mod n`.
///
/// Port of MPU 0.75 `PP.pm::powmod` through [`Int::powmod`].
///
/// ```
/// use math_prime_util::bigint::arith::powmod;
/// use num_bigint::BigUint;
/// let (a, k, n) = (BigUint::from(3_u8), BigUint::from(5_u8), BigUint::from(7_u8));
/// assert_eq!(powmod(&a, &k, &n), Some(BigUint::from(5_u8)));
/// assert_eq!(powmod(&a, &k, &BigUint::from(0_u8)), None);
/// ```
pub fn powmod(a: &BigUint, k: &BigUint, n: &BigUint) -> Option<BigUint> {
    Int::powmod(a, k, n)
}

/// `(a + b) mod n`, or `None` if `n == 0`.
///
/// Port of MPU 0.75 `PP.pm::addmod`.
///
/// ```
/// use math_prime_util::bigint::arith::addmod;
/// use num_bigint::BigUint;
/// let n = BigUint::from(10_u8);
/// assert_eq!(addmod(&BigUint::from(7_u8), &BigUint::from(8_u8), &n), Some(BigUint::from(5_u8)));
/// ```
pub fn addmod(a: &BigUint, b: &BigUint, n: &BigUint) -> Option<BigUint> {
    if is_zero(n) {
        return None;
    }
    Some((a + b) % n)
}

/// `(a * b) mod n`, or `None` if `n == 0`.
///
/// Port of MPU 0.75 `PP.pm::mulmod` through [`Int::mulmod`].
///
/// ```
/// use math_prime_util::bigint::arith::mulmod;
/// use num_bigint::BigUint;
/// let n = BigUint::from(10_u8);
/// assert_eq!(mulmod(&BigUint::from(7_u8), &BigUint::from(8_u8), &n), Some(BigUint::from(6_u8)));
/// ```
pub fn mulmod(a: &BigUint, b: &BigUint, n: &BigUint) -> Option<BigUint> {
    Int::mulmod(a, b, n)
}

/// `a / b mod n`, or `None` if `n == 0` or `b` has no inverse mod `n`.
///
/// Port of MPU 0.75 `PP.pm::divmod`: the inverse of `b` times `a`.
///
/// ```
/// use math_prime_util::bigint::arith::divmod;
/// use num_bigint::BigUint;
/// let (a, b) = (BigUint::from(6_u8), BigUint::from(3_u8));
/// assert_eq!(divmod(&a, &b, &BigUint::from(7_u8)), Some(BigUint::from(2_u8)));
/// assert_eq!(divmod(&a, &b, &BigUint::from(9_u8)), None);
/// ```
pub fn divmod(a: &BigUint, b: &BigUint, n: &BigUint) -> Option<BigUint> {
    let inverse = invmod(b, n)?;
    mulmod(a, &inverse, n)
}

/// Integer square root: the largest `r` with `r * r <= n`.
///
/// Port of MPU 0.75 `PP.pm::sqrtint`.
///
/// ```
/// use math_prime_util::bigint::arith::sqrtint;
/// use num_bigint::BigUint;
/// assert_eq!(sqrtint(&BigUint::from(1524155677489_u64)), BigUint::from(1234567_u32));
/// ```
pub fn sqrtint(n: &BigUint) -> BigUint {
    n.sqrt()
}

/// Integer square root under the name the standard library uses; the same
/// as [`sqrtint`].
///
/// ```
/// use math_prime_util::bigint::arith::isqrt;
/// use num_bigint::BigUint;
/// assert_eq!(isqrt(&BigUint::from(99_u8)), BigUint::from(9_u8));
/// ```
pub fn isqrt(n: &BigUint) -> BigUint {
    sqrtint(n)
}

/// `true` if `n` is a perfect square.
///
/// Port of MPU 0.75 `PP.pm::is_square`.
///
/// ```
/// use math_prime_util::bigint::arith::is_square;
/// use num_bigint::BigUint;
/// assert!(is_square(&BigUint::from(1524155677489_u64)));
/// assert!(!is_square(&BigUint::from(1524155677490_u64)));
/// ```
pub fn is_square(n: &BigUint) -> bool {
    let r = n.sqrt();
    &r * &r == *n
}

/// Integer `k`-th root: the largest `r` with `r^k <= n`.
///
/// Port of MPU 0.75 `PP.pm::rootint`.
///
/// # Panics
///
/// Panics if `k == 0`, as MPU's `rootint` dies there.
///
/// ```
/// use math_prime_util::bigint::arith::rootint;
/// use num_bigint::BigUint;
/// assert_eq!(rootint(&BigUint::from(15625_u16), 3), BigUint::from(25_u8));
/// assert_eq!(rootint(&(BigUint::from(1_u8) << 1000_usize), 999), BigUint::from(2_u8));
/// ```
pub fn rootint(n: &BigUint, k: u32) -> BigUint {
    assert!(k != 0, "rootint: k must be at least 1");
    match k {
        1 => n.clone(),
        2 => n.sqrt(),
        _ => n.nth_root(k),
    }
}

/// Integer logarithm: the largest `e` with `b^e <= n`.
///
/// Port of MPU 0.75 `PP.pm::logint`.
///
/// # Panics
///
/// Panics if `n == 0` or `b < 2`, like the native `logint`.
///
/// ```
/// use math_prime_util::bigint::arith::logint;
/// use num_bigint::BigUint;
/// assert_eq!(logint(&BigUint::from(1000_u16), &BigUint::from(10_u8)), 3);
/// assert_eq!(logint(&(BigUint::from(1_u8) << 1000_usize), &BigUint::from(2_u8)), 1000);
/// ```
pub fn logint(n: &BigUint, b: &BigUint) -> u64 {
    assert!(!is_zero(n), "logint: n must be at least 1");
    assert!(*b >= BigUint::from(2_u8), "logint: base must be at least 2");
    if *b == BigUint::from(2_u8) {
        return n.bits() - 1;
    }
    let mut e = 0;
    let mut power = b.clone();
    while power <= *n {
        e += 1;
        power *= b;
    }
    e
}

/// `true` if `n` is a perfect `k`-th power (`n == r^k` for some `r`).
///
/// With `k == 0`, returns `true` if `n` is a perfect power for *any* `k >= 2`
/// (see [`is_power_exp`] for MPU's one-argument form).
///
/// Port of MPU 0.75 `PP.pm::is_power`.
///
/// ```
/// use math_prime_util::bigint::arith::is_power;
/// use num_bigint::BigUint;
/// assert!(is_power(&BigUint::from(815730721_u32), 8));
/// assert!(!is_power(&BigUint::from(815730722_u32), 8));
/// assert!(is_power(&BigUint::from(1024_u16), 0));
/// ```
pub fn is_power(n: &BigUint, k: u32) -> bool {
    match k {
        0 => is_power_exp(n) != 0,
        1 => true,
        _ => rootint(n, k).pow(k) == *n,
    }
}

/// The largest `k >= 2` such that `n` is a perfect `k`-th power, or `0` if
/// `n` is not a perfect power. `0` and `1` return `0`, as in MPU.
///
/// Port of MPU 0.75 `PP.pm::_powerof_ret`: the smallest prime exponent
/// that works is found first, then the root is examined again.
///
/// ```
/// use math_prime_util::bigint::arith::is_power_exp;
/// use num_bigint::BigUint;
/// assert_eq!(is_power_exp(&BigUint::from(1024_u16)), 10);
/// assert_eq!(is_power_exp(&(BigUint::from(1_u8) << 1000_usize)), 1000);
/// assert_eq!(is_power_exp(&BigUint::from(37_u8)), 0);
/// ```
pub fn is_power_exp(n: &BigUint) -> u32 {
    if *n < BigUint::from(4_u8) {
        return 0;
    }
    let mut kmax = 0;
    let mut m = n.clone();
    loop {
        // A k-th root of m has at least one bit, so k is below the bit length.
        let limit = u32::try_from(m.bits() - 1).unwrap_or(u32::MAX);
        let Some(k) = small_primes(limit).find(|&k| is_power(&m, k)) else {
            return kmax;
        };
        m = rootint(&m, k);
        kmax = if kmax == 0 { k } else { kmax * k };
    }
}

/// The primes up to `limit`, by trial division (the limit is a bit count).
fn small_primes(limit: u32) -> impl Iterator<Item = u32> {
    (2..=limit).filter(|&k| (2..k).take_while(|d| d * d <= k).all(|d| k % d != 0))
}

/// Jacobi symbol `(a / n)` for odd `n`.
///
/// MPU has no separate `jacobi`; this is [`kronecker`] on the domain where
/// the two symbols agree.
///
/// # Panics
///
/// Panics if `n` is even (including zero).
///
/// ```
/// use math_prime_util::bigint::arith::jacobi;
/// use num_bigint::{BigInt, BigUint};
/// assert_eq!(jacobi(&BigInt::from(5), &BigUint::from(11_u8)), 1);
/// assert_eq!(jacobi(&BigInt::from(-1), &BigUint::from(7_u8)), -1);
/// assert_eq!(jacobi(&BigInt::from(6), &BigUint::from(9_u8)), 0);
/// ```
pub fn jacobi(a: &BigInt, n: &BigUint) -> i32 {
    assert!(n.bit(0), "jacobi: n must be odd");
    kronecker(a, &BigInt::from(n.clone()))
}

/// Kronecker symbol `(a / b)` for any signed `a` and `b`.
///
/// Port of MPU 0.75 `PP.pm::kronecker`: the power of two in `b` is split
/// off, then quadratic reciprocity reduces the odd part.
///
/// ```
/// use math_prime_util::bigint::arith::kronecker;
/// use num_bigint::BigInt;
/// assert_eq!(kronecker(&BigInt::from(-1), &BigInt::from(3)), -1);
/// assert_eq!(kronecker(&BigInt::from(3), &BigInt::from(8)), -1);
/// assert_eq!(kronecker(&BigInt::from(2), &BigInt::from(0)), 0);
/// ```
pub fn kronecker(a: &BigInt, b: &BigInt) -> i32 {
    let (a_negative, a_magnitude) = (a.sign() == Sign::Minus, a.magnitude());
    let (b_negative, b_magnitude) = (b.sign() == Sign::Minus, b.magnitude());
    if is_zero(b_magnitude) {
        return i32::from(*a_magnitude == one());
    }
    if !a_magnitude.bit(0) && !b_magnitude.bit(0) {
        return 0;
    }
    let mut sign = if a_negative && b_negative { -1 } else { 1 };
    let twos = b_magnitude.trailing_zeros().unwrap_or(0);
    let mut n = b_magnitude >> twos;
    if twos % 2 == 1 {
        // (a / 2) is -1 exactly when a is 3 or 5 mod 8; a is odd here.
        let residue = a_magnitude % 8_u8;
        let residue = if a_negative {
            (BigUint::from(8_u8) - residue) % 8_u8
        } else {
            residue
        };
        if residue == BigUint::from(3_u8) || residue == BigUint::from(5_u8) {
            sign = -sign;
        }
    }
    let reduced = a_magnitude % &n;
    let mut m = if a_negative && !is_zero(&reduced) {
        &n - reduced
    } else {
        reduced
    };
    let (three, four, five, eight) = (
        BigUint::from(3_u8),
        BigUint::from(4_u8),
        BigUint::from(5_u8),
        BigUint::from(8_u8),
    );
    while !is_zero(&m) {
        let twos = m.trailing_zeros().unwrap_or(0);
        m >>= twos;
        if twos % 2 == 1 {
            let residue = &n % &eight;
            if residue == three || residue == five {
                sign = -sign;
            }
        }
        if &m % &four == three && &n % &four == three {
            sign = -sign;
        }
        let remainder = &n % &m;
        n = m;
        m = remainder;
    }
    if n == one() { sign } else { 0 }
}

/// Binomial coefficient `C(n, k)`, or `None` if the smaller of `k` and
/// `n - k` does not fit in a `u64` (such a result has more than 2^64 bits).
///
/// Port of MPU 0.75 `PP.pm::binomial` for non-negative arguments, with the
/// reflection `k -> n - k` that the Perl source leaves as a TODO.
///
/// ```
/// use math_prime_util::bigint::arith::binomial;
/// use num_bigint::BigUint;
/// assert_eq!(binomial(&BigUint::from(20_u8), &BigUint::from(15_u8)), Some(BigUint::from(15504_u16)));
/// assert_eq!(binomial(&BigUint::from(5_u8), &BigUint::from(6_u8)), Some(BigUint::from(0_u8)));
/// ```
pub fn binomial(n: &BigUint, k: &BigUint) -> Option<BigUint> {
    if k > n {
        return Some(zero());
    }
    let smaller = (n - k).min(k.clone());
    let steps = u64::try_from(&smaller).ok()?;
    let base = n - &smaller;
    let mut result = one();
    for i in 1..=steps {
        result = result * (&base + i) / i;
    }
    Some(result)
}

/// `n!` as an arbitrary precision value.
///
/// Port of MPU 0.75 `PP.pm::factorial`.
///
/// ```
/// use math_prime_util::bigint::arith::factorial;
/// use num_bigint::BigUint;
/// assert_eq!(factorial(5), BigUint::from(120_u8));
/// assert_eq!(factorial(25).to_string(), "15511210043330985984000000");
/// ```
pub fn factorial(n: u64) -> BigUint {
    (2..=n).fold(one(), |product, i| product * i)
}

/// The exponent of `k` in `n`, or `None` when `n` is zero.
///
/// Port of MPU 0.75 `PP.pm::valuation`.
///
/// # Panics
///
/// Panics when `k` is less than two, like the native `valuation`.
///
/// ```
/// use math_prime_util::bigint::arith::valuation;
/// use num_bigint::BigUint;
/// assert_eq!(valuation(&BigUint::from(72_u8), &BigUint::from(3_u8)), Some(2));
/// assert_eq!(valuation(&BigUint::from(0_u8), &BigUint::from(2_u8)), None);
/// ```
pub fn valuation(n: &BigUint, k: &BigUint) -> Option<u64> {
    assert!(*k >= BigUint::from(2_u8), "valuation: k must be > 1");
    if is_zero(n) {
        return None;
    }
    if *k == BigUint::from(2_u8) {
        return n.trailing_zeros();
    }
    let mut v = 0;
    let mut m = n.clone();
    loop {
        let (quotient, remainder) = m.divrem(k)?;
        if !is_zero(&remainder) {
            return Some(v);
        }
        v += 1;
        m = quotient;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn big(n: u64) -> BigUint {
        BigUint::from(n)
    }

    fn signed(n: i64) -> BigInt {
        BigInt::from(n)
    }

    #[test]
    fn edge_cases() {
        let p = (one() << 127_usize) - 1_u8;
        assert_eq!(gcd(&zero(), &zero()), zero());
        assert_eq!(lcm(&zero(), &p), zero());
        assert_eq!(lcm(&p, &zero()), zero());
        assert_eq!(lcm(&(&p * 4_u8), &(&p * 6_u8)), &p * 12_u8);
        assert_eq!(
            gcdext(&signed(0), &signed(0)),
            (signed(0), signed(0), signed(0))
        );
        assert_eq!(
            gcdext(&signed(-7), &signed(0)),
            (signed(7), signed(-1), signed(0))
        );
        assert_eq!(invmod(&p, &zero()), None);
        assert_eq!(invmod(&p, &one()), Some(zero()));
        assert_eq!(invmod(&zero(), &p), None);
        assert_eq!(invmod(&(&p * 3_u8), &(&p * 7_u8)), None);
        let inverse = invmod(&big(14), &p).unwrap();
        assert_eq!(big(14) * inverse % &p, one());
        assert_eq!(addmod(&p, &p, &zero()), None);
        assert_eq!(addmod(&p, &p, &one()), Some(zero()));
        assert_eq!(addmod(&(&p + 1_u8), &(&p + 2_u8), &p), Some(big(3)));
        assert_eq!(divmod(&p, &big(2), &zero()), None);
        assert_eq!(divmod(&big(6), &big(3), &big(9)), None);
        assert_eq!(divmod(&big(5), &big(1), &one()), Some(zero()));
        assert_eq!(powmod(&p, &p, &zero()), None);
        assert_eq!(mulmod(&p, &p, &zero()), None);
        assert_eq!(sqrtint(&(&p * &p)), p);
        assert_eq!(isqrt(&(&p * &p + 1_u8)), p);
        assert!(is_square(&(&p * &p)));
        assert!(!is_square(&(&p * &p + 1_u8)));
        assert!(is_square(&zero()));
        assert_eq!(rootint(&p, 1), p);
        assert_eq!(rootint(&(&p * &p * &p), 3), p);
        assert_eq!(rootint(&(one() << 1000_usize), 1000), big(2));
        assert_eq!(rootint(&(one() << 1000_usize), 1001), one());
        assert_eq!(logint(&((one() << 1000_usize) - 1_u8), &big(2)), 999);
        assert_eq!(logint(&p, &p), 1);
        assert_eq!(logint(&(&p - 1_u8), &p), 0);
        assert_eq!(logint(&big(1000), &big(10)), 3);
        assert_eq!(logint(&big(999), &big(10)), 2);
        assert!(is_power(&p, 1));
        assert!(is_power(&(&p * &p * &p), 3));
        assert!(!is_power(&(&p * &p * &p + 1_u8), 3));
        assert!(!is_power(&p, 0));
        assert!(is_power(&(&p * &p), 0));
        assert_eq!(is_power_exp(&zero()), 0);
        assert_eq!(is_power_exp(&big(3)), 0);
        assert_eq!(is_power_exp(&big(4)), 2);
        assert_eq!(is_power_exp(&(one() << 1000_usize)), 1000);
        assert_eq!(is_power_exp(&(one() << 41_usize)), 41);
        assert_eq!(is_power_exp(&big(6).pow(24)), 24);
        assert_eq!(is_power_exp(&(&p * &p * &p * &p)), 4);
        assert_eq!(small_primes(1).count(), 0);
        assert_eq!(
            small_primes(30).collect::<Vec<_>>(),
            [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
        );
    }

    #[test]
    #[should_panic(expected = "rootint: k must be at least 1")]
    fn rootint_zero_exponent() {
        let _ = rootint(&big(8), 0);
    }

    #[test]
    #[should_panic(expected = "logint: n must be at least 1")]
    fn logint_zero() {
        let _ = logint(&zero(), &big(2));
    }

    #[test]
    #[should_panic(expected = "logint: base must be at least 2")]
    fn logint_base_one() {
        let _ = logint(&big(8), &one());
    }

    #[test]
    #[should_panic(expected = "jacobi: n must be odd")]
    fn jacobi_even_modulus() {
        let _ = jacobi(&signed(3), &big(8));
    }

    #[test]
    #[should_panic(expected = "valuation: k must be > 1")]
    fn valuation_base_one() {
        let _ = valuation(&big(8), &one());
    }

    #[test]
    fn symbols_and_combinatorics() {
        let p = (one() << 127_usize) - 1_u8;
        let n = BigInt::from(p.clone());
        assert_eq!(jacobi(&signed(2), &p), 1);
        assert_eq!(jacobi(&(-&n), &p), 0);
        assert_eq!(jacobi(&signed(-1), &p), -1);
        assert_eq!(jacobi(&signed(0), &one()), 1);
        assert_eq!(kronecker(&signed(0), &signed(0)), 0);
        assert_eq!(kronecker(&signed(-1), &signed(0)), 1);
        assert_eq!(kronecker(&signed(4), &signed(6)), 0);
        assert_eq!(kronecker(&signed(-3), &signed(-1)), -1);
        assert_eq!(kronecker(&signed(3), &signed(-1)), 1);
        assert_eq!(kronecker(&signed(0), &signed(-1)), 1);
        assert_eq!(kronecker(&signed(-5), &signed(2)), -1);
        assert_eq!(kronecker(&signed(-3), &signed(2)), -1);
        assert_eq!(kronecker(&signed(5), &signed(2)), -1);
        assert_eq!(kronecker(&signed(7), &signed(2)), 1);
        assert_eq!(kronecker(&signed(5), &signed(-8)), -1);
        assert_eq!(kronecker(&signed(-5), &signed(-8)), 1);
        assert_eq!(kronecker(&signed(10), &signed(9)), 1);
        assert_eq!(kronecker(&signed(6), &signed(9)), 0);
        assert_eq!(binomial(&p, &(&p + 1_u8)), Some(zero()));
        assert_eq!(binomial(&p, &p), Some(one()));
        assert_eq!(binomial(&p, &zero()), Some(one()));
        assert_eq!(binomial(&p, &one()), Some(p.clone()));
        assert_eq!(binomial(&p, &(&p - 1_u8)), Some(p.clone()));
        assert_eq!(binomial(&p, &(&p - 2_u8)), binomial(&p, &big(2)));
        assert_eq!(binomial(&p, &big(2)), Some(&p * (&p - 1_u8) / 2_u8));
        let huge = one() << 70_usize;
        assert_eq!(binomial(&(&huge * 2_u8), &huge), None);
        assert_eq!(factorial(0), one());
        assert_eq!(factorial(1), one());
        assert_eq!(factorial(21).to_string(), "51090942171709440000");
        assert_eq!(valuation(&zero(), &big(3)), None);
        assert_eq!(valuation(&one(), &big(3)), Some(0));
        assert_eq!(valuation(&(one() << 300_usize), &big(2)), Some(300));
        assert_eq!(valuation(&(big(3).pow(200) * &p), &big(3)), Some(200));
        assert_eq!(valuation(&(&p * &p * 5_u8), &p), Some(2));
    }

    proptest! {
        #[test]
        fn int_functions_agree_with_native(a: u64, b: u64, k in 1_u32..70, base in 2_u64..1000) {
            let (x, y) = (big(a), big(b));
            prop_assert_eq!(gcd(&x, &y), big(crate::gcd(a, b)));
            prop_assert_eq!(lcm(&x, &y).to_u64(), crate::lcm(a, b));
            prop_assert_eq!(sqrtint(&x), big(crate::sqrtint(a)));
            prop_assert_eq!(isqrt(&x), big(crate::sqrtint(a)));
            prop_assert_eq!(is_square(&x), crate::is_square(a));
            prop_assert_eq!(rootint(&x, k), big(crate::rootint(a, k)));
            prop_assert_eq!(is_power(&x, k), crate::is_power(a, k));
            prop_assert_eq!(is_power(&x, 0), crate::is_power(a, 0));
            let root = big(crate::rootint(a, k));
            prop_assert_eq!(is_power(&root.pow(k), k), crate::is_power(crate::rootint(a, k).pow(k), k));
            if a != 0 {
                prop_assert_eq!(logint(&x, &big(base)), u64::from(crate::logint(a, base)));
                prop_assert_eq!(logint(&x, &big(2)), u64::from(crate::logint(a, 2)));
                prop_assert_eq!(valuation(&x, &big(base)), crate::valuation(a, base));
                prop_assert_eq!(valuation(&x, &big(2)), crate::valuation(a, 2));
            }
        }

        #[test]
        fn perfect_powers_agree_with_native(root in 2_u64..1000, k in 2_u32..64) {
            let Some(n) = root.checked_pow(k) else { return Ok(()) };
            // The native `is_power_exp` tests prime exponents up to 37 only, so it
            // returns 0 for 2^41, 2^43, 2^47, 2^53, 2^59 and 2^61 (MPU gives k).
            if root != 2 || k <= 37 {
                prop_assert_eq!(is_power_exp(&big(n)), crate::int::is_power_exp(n));
            }
            prop_assert!(is_power(&big(n), k));
            prop_assert_eq!(rootint(&big(n), k), big(root));
            prop_assert_eq!(rootint(&big(n - 1), k), big(root - 1));
        }

        #[test]
        fn modular_functions_agree_with_native(a: u64, b: u64, n: u64, e in 0_u64..256) {
            let (x, y, m) = (big(a), big(b), big(n));
            prop_assert_eq!(addmod(&x, &y, &m).map(|v| v.to_u64()), crate::addmod(a, b, n).map(Some));
            prop_assert_eq!(mulmod(&x, &y, &m).map(|v| v.to_u64()), crate::mulmod(a, b, n).map(Some));
            prop_assert_eq!(powmod(&x, &big(e), &m).map(|v| v.to_u64()), crate::powmod(a, e, n).map(Some));
            prop_assert_eq!(invmod(&x, &m).map(|v| v.to_u64()), crate::invmod(a, n).map(Some));
            prop_assert_eq!(divmod(&x, &y, &m).map(|v| v.to_u64()), crate::divmod(a, b, n).map(Some));
        }

        #[test]
        fn signed_functions_agree_with_native(a: i64, b: i64) {
            let (x, y) = (signed(a), signed(b));
            let (g, u, v) = crate::gcdext(a, b);
            prop_assert_eq!(gcdext(&x, &y), (signed(g), signed(u), signed(v)));
            prop_assert_eq!(kronecker(&x, &y), crate::kronecker(a, b));
            if b % 2 != 0 {
                prop_assert_eq!(jacobi(&x, &big(b.unsigned_abs())), crate::kronecker(a, b.abs()));
            }
        }

        #[test]
        fn combinatorics_agree_with_native(n in 0_u64..200, k in 0_u64..200, f in 0_u64..25) {
            prop_assert_eq!(binomial(&big(n), &big(k)).and_then(|v| v.to_u64()), crate::binomial(n, k));
            prop_assert_eq!(factorial(f).to_u64(), crate::factorial(f));
        }

        #[test]
        fn big_identities(a in 1_u64..=u64::MAX, b in 1_u64..=u64::MAX, shift in 64_u64..300) {
            let x = big(a) << shift;
            let y = (big(b) << shift) + 1_u8;
            let g = gcd(&x, &y);
            prop_assert_eq!(&x / &g * &y / &g * &g, lcm(&x, &y));
            let (g2, u, v) = gcdext(&BigInt::from(x.clone()), &BigInt::from(y.clone()));
            prop_assert_eq!(g2, BigInt::from(g.clone()));
            prop_assert_eq!(BigInt::from(x.clone()) * u + BigInt::from(y.clone()) * v, BigInt::from(g));
            if let Some(inverse) = invmod(&x, &y) {
                prop_assert_eq!(mulmod(&x, &inverse, &y), Some(one()));
                prop_assert_eq!(divmod(&one(), &x, &y), Some(inverse));
            }
            let square = &x * &x;
            prop_assert!(is_square(&square) && !is_square(&(&square + 1_u8)));
            prop_assert_eq!(sqrtint(&(&square + 1_u8)), x.clone());
            prop_assert_eq!(rootint(&(&square * &x), 3), x.clone());
            prop_assert_eq!(logint(&x, &big(2)), x.bits() - 1);
            prop_assert_eq!(valuation(&x, &big(2)), Some(u64::from(a.trailing_zeros()) + shift));
            prop_assert!(is_power(&(&x * &x * &x), 3));
        }
    }
}
