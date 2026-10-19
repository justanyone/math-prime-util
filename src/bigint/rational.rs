//! Arbitrary precision rational sequences from MPU 0.75 `rational.c`.

use super::arith::{gcd, invmod};
use num_bigint::BigUint;

fn validate(num: &BigUint, den: &BigUint) {
    assert!(
        *num != BigUint::ZERO && *den != BigUint::ZERO && gcd(num, den) == BigUint::from(1u8),
        "rational must be positive and reduced"
    );
}

/// Return the Euclidean continued fraction.
///
/// Port of MPU `rational.c` `contfrac`.
///
/// # Panics
///
/// Panics if the denominator is zero.
///
/// ```
/// use math_prime_util::bigint::rational::contfrac;
/// assert_eq!(contfrac(&415u32.into(), &93u32.into()), vec![4u32.into(), 2u32.into(), 6u32.into(), 7u32.into()]);
/// ```
#[must_use]
pub fn contfrac(num: &BigUint, den: &BigUint) -> Vec<BigUint> {
    assert!(*den != BigUint::ZERO, "denominator must be positive");
    let (mut n, mut d) = (num.clone(), den.clone());
    let mut out = Vec::new();
    while d != BigUint::ZERO {
        out.push(&n / &d);
        (n, d) = (d.clone(), n % d);
    }
    out
}

/// Return the convergents of a continued fraction.
///
/// Port of MPU `rational.c` `convergents`.
///
/// ```
/// use math_prime_util::bigint::rational::convergents;
/// assert_eq!(convergents(&[2u32.into()]), Some(vec![(2u32.into(), 1u32.into())]));
/// ```
#[must_use]
pub fn convergents(cf: &[BigUint]) -> Option<Vec<(BigUint, BigUint)>> {
    let first = cf.first()?;
    let (mut p0, mut p1, mut q0, mut q1) = (
        BigUint::from(1u8),
        first.clone(),
        BigUint::ZERO,
        BigUint::from(1u8),
    );
    let mut out = vec![(p1.clone(), q1.clone())];
    for a in &cf[1..] {
        if *a == BigUint::ZERO {
            return None;
        }
        let (p, q) = (a * &p1 + p0, a * &q1 + q0);
        (p0, p1, q0, q1) = (p1, p, q1, q);
        out.push((p1.clone(), q1.clone()));
    }
    Some(out)
}

/// Return the value of a continued fraction.
///
/// Port of MPU `rational.c` `convergents`.
///
/// # Panics
///
/// Panics if a term after the first is zero.
///
/// ```
/// use math_prime_util::bigint::rational::from_contfrac;
/// assert_eq!(from_contfrac(&[]), Some((0u32.into(), 1u32.into())));
/// ```
#[must_use]
pub fn from_contfrac(cf: &[BigUint]) -> Option<(BigUint, BigUint)> {
    assert!(
        cf.iter().skip(1).all(|a| *a != BigUint::ZERO),
        "terms after the first must be positive"
    );
    if cf.is_empty() {
        return Some((BigUint::ZERO, BigUint::from(1u8)));
    }
    convergents(cf)?.pop()
}

/// Return the next Calkin-Wilf fraction.
///
/// Port of MPU `rational.c` `next_calkin_wilf`.
///
/// # Panics
///
/// Panics if the fraction is not positive and reduced.
///
/// ```
/// use math_prime_util::bigint::rational::next_calkin_wilf;
/// assert_eq!(next_calkin_wilf(&1u32.into(), &1u32.into()), Some((1u32.into(), 2u32.into())));
/// ```
#[must_use]
pub fn next_calkin_wilf(num: &BigUint, den: &BigUint) -> Option<(BigUint, BigUint)> {
    validate(num, den);
    let rem = num % den;
    Some((den.clone(), (num - &rem) + (den - &rem)))
}

/// Return the Calkin-Wilf index.
///
/// Port of MPU `rational.c` `calkin_wilf_n`.
/// Returns `None` if the index bit count exceeds the address space.
///
/// # Panics
///
/// Panics if the fraction is not positive and reduced.
///
/// ```
/// use math_prime_util::bigint::rational::calkin_wilf_n;
/// assert_eq!(calkin_wilf_n(&3u32.into(), &2u32.into()), Some(5u32.into()));
/// ```
#[must_use]
pub fn calkin_wilf_n(num: &BigUint, den: &BigUint) -> Option<BigUint> {
    validate(num, den);
    let mut cf = contfrac(num, den);
    *cf.last_mut()? -= 1u8;
    let (mut n, mut shift, mut on) = (BigUint::ZERO, 0usize, true);
    for term in cf {
        let count = usize::try_from(&term).ok()?;
        let end = shift.checked_add(count)?;
        if on && count != 0 {
            n |= ((BigUint::from(1u8) << count) - 1u8) << shift;
        }
        shift = end;
        on = !on;
    }
    n.set_bit(u64::try_from(shift).ok()?, true);
    Some(n)
}

/// Return the Stern-Brocot index.
///
/// Port of MPU `rational.c` `stern_brocot_n`.
/// Returns `None` if the index bit count exceeds the address space.
///
/// # Panics
///
/// Panics if the fraction is not positive and reduced.
///
/// ```
/// use math_prime_util::bigint::rational::stern_brocot_n;
/// assert_eq!(stern_brocot_n(&2u32.into(), &3u32.into()), Some(5u32.into()));
/// ```
#[must_use]
pub fn stern_brocot_n(num: &BigUint, den: &BigUint) -> Option<BigUint> {
    let m = calkin_wilf_n(num, den)?;
    let mut n = BigUint::from(1u8);
    for bit in 0..m.bits() - 1 {
        n <<= 1usize;
        if m.bit(bit) {
            n += 1u8;
        }
    }
    Some(n)
}

/// Return the fraction at a Calkin-Wilf index.
///
/// Port of MPU `rational.c` `nth_calkin_wilf`.
///
/// # Panics
///
/// Panics if the index is zero.
///
/// ```
/// use math_prime_util::bigint::rational::nth_calkin_wilf;
/// assert_eq!(nth_calkin_wilf(&4u32.into()), (1u32.into(), 3u32.into()));
/// ```
#[must_use]
pub fn nth_calkin_wilf(n: &BigUint) -> (BigUint, BigUint) {
    assert!(*n != BigUint::ZERO, "index must be positive");
    let (mut p, mut q) = (BigUint::ZERO, BigUint::from(1u8));
    for bit in (0..n.bits()).rev() {
        if n.bit(bit) {
            p += &q;
        } else {
            q += &p;
        }
    }
    (p, q)
}

/// Return the fraction at a Stern-Brocot index.
///
/// Port of MPU `rational.c` `nth_stern_brocot`.
///
/// # Panics
///
/// Panics if the index is zero.
///
/// ```
/// use math_prime_util::bigint::rational::nth_stern_brocot;
/// assert_eq!(nth_stern_brocot(&5u32.into()), (2u32.into(), 3u32.into()));
/// ```
#[must_use]
pub fn nth_stern_brocot(n: &BigUint) -> (BigUint, BigUint) {
    assert!(*n != BigUint::ZERO, "index must be positive");
    let (mut p, mut q) = (BigUint::from(1u8), BigUint::from(1u8));
    for bit in 0..n.bits() - 1 {
        if n.bit(bit) {
            p += &q;
        } else {
            q += &p;
        }
    }
    (p, q)
}

/// Return a Stern diatomic sequence value.
///
/// Port of MPU `rational.c` `nth_stern_diatomic`.
///
/// ```
/// use math_prime_util::bigint::rational::nth_stern_diatomic;
/// assert_eq!(nth_stern_diatomic(&6u32.into()), 2u32.into());
/// ```
#[must_use]
pub fn nth_stern_diatomic(n: &BigUint) -> BigUint {
    let (mut p, mut q) = (BigUint::ZERO, BigUint::from(1u8));
    for bit in 0..n.bits() {
        if n.bit(bit) {
            p += &q;
        } else {
            q += &p;
        }
    }
    p
}

/// Return the next Stern-Brocot fraction.
///
/// Port of MPU `rational.c` `next_stern_brocot`.
///
/// # Panics
///
/// Panics if the fraction is not positive and reduced.
///
/// ```
/// use math_prime_util::bigint::rational::next_stern_brocot;
/// assert_eq!(next_stern_brocot(&1u32.into(), &1u32.into()), Some((1u32.into(), 2u32.into())));
/// ```
#[must_use]
pub fn next_stern_brocot(num: &BigUint, den: &BigUint) -> Option<(BigUint, BigUint)> {
    validate(num, den);
    if *den == BigUint::from(1u8) {
        return Some((BigUint::from(1u8), num + 1u8));
    }
    Some(nth_stern_brocot(&(stern_brocot_n(num, den)? + 1u8)))
}

/// Return the next fraction in a Farey sequence.
///
/// Port of MPU `rational.c` `next_farey`.
/// Returns `None` if the reduced fraction is outside the sequence.
///
/// ```
/// use math_prime_util::bigint::rational::next_farey;
/// assert_eq!(next_farey(&5u32.into(), &1u32.into(), &3u32.into()), Some((2u32.into(), 5u32.into())));
/// ```
#[must_use]
pub fn next_farey(n: &BigUint, p: &BigUint, q: &BigUint) -> Option<(BigUint, BigUint)> {
    if *n == BigUint::ZERO || p >= q {
        return None;
    }
    let g = gcd(p, q);
    let (p, q) = (p / &g, q / &g);
    if q > *n {
        return None;
    }
    if p == BigUint::ZERO {
        return Some((BigUint::from(1u8), n.clone()));
    }
    // The extended-gcd coefficient gives r = -inverse(p) modulo q.
    let r = &q - invmod(&p, &q)?;
    let d = n - ((n - &r) % &q);
    let c = (&d * p + 1u8) / q;
    Some((c, d))
}
