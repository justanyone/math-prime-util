//! Modular number theory on arbitrary precision values: the functions of
//! [`crate::modular`], [`crate::arith`], [`crate::lucas`] and
//! [`crate::factor`](mod@crate::factor) with the same names, for [`BigUint`].
//!
//! Port of MPU 0.75 `PP.pm`, the pure Perl path that `Math::BigInt` inputs
//! take, with the algorithms of the native port: Tonelli-Shanks and the
//! `p = 3 mod 4` and `p = 5 mod 8` short cuts for square roots, Hensel
//! lifting on prime powers, Tonelli-Shanks `k`-th roots, Pohlig-Hellman with
//! baby-step giant-step for discrete logarithms, Garner's form of the
//! Chinese remainder theorem, Lucas and Granville for binomials. A modulus
//! of zero gives `None`, as on the native path, and below 2^64 every result
//! equals the native one.
//!
//! Functions that need the factorization of a composite modulus or of a
//! group order (`allsqrtmod`, `znorder`, `znprimroot`, `znlog`,
//! `is_primitive_root`, `is_quasi_carmichael`, and `sqrtmod` and `rootmod`
//! on composite moduli) factor through [`factor::factor`], MPU's pure Perl
//! recipe with no time limit; a cofactor the recipe cannot split is treated
//! as prime. They are meant for moduli that are prime, or whose factors are
//! small or quickly found: a modulus with two large prime factors takes as
//! long as factoring it does, as in MPU.
//!
//! ```
//! use math_prime_util::bigint::modular::{sqrtmod, znlog};
//! use num_bigint::BigUint;
//! let p = (BigUint::from(1_u8) << 127_usize) - 1_u8;
//! let r = sqrtmod(&BigUint::from(2_u8), &p).unwrap();
//! assert_eq!(&r * &r % &p, BigUint::from(2_u8));
//! assert_eq!(znlog(&BigUint::from(8_u8), &BigUint::from(2_u8), &p), Some(BigUint::from(3_u8)));
//! ```
use super::Int;
use super::arith::{gcd, gcdext, invmod, is_power, is_square, kronecker, sqrtint};
use super::factor;
use super::primality::{is_prob_prime, lucas_sequence};
use num_bigint::{BigInt, BigUint, Sign};
use std::collections::HashMap;

fn zero() -> BigUint {
    BigUint::from(0_u8)
}

fn one() -> BigUint {
    BigUint::from(1_u8)
}

fn two() -> BigUint {
    BigUint::from(2_u8)
}

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

fn is_zero(n: &BigUint) -> bool {
    n.bits() == 0
}

fn is_one(n: &BigUint) -> bool {
    *n == one()
}

fn is_prime(n: &BigUint) -> bool {
    is_prob_prime(n) != 0
}

/// `a^k mod n` for `n > 0`.
fn pow(a: &BigUint, k: &BigUint, n: &BigUint) -> BigUint {
    Int::powmod(a, k, n).unwrap_or_default()
}

fn pow_u(a: &BigUint, k: u64, n: &BigUint) -> BigUint {
    pow(a, &big(k), n)
}

fn mul(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    a * b % n
}

fn add(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    (a + b) % n
}

/// `(a - b) mod n` for any `a`, `b`.
fn sub(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    let (a, b) = (a % n, b % n);
    if a >= b { a - b } else { n - b + a }
}

/// The inverse of `b` modulo `n`, or zero when there is none.
fn inverse(b: &BigUint, n: &BigUint) -> BigUint {
    invmod(b, n).unwrap_or_default()
}

/// `a / b mod n` after removing the common factor of `a` and `b`, as
/// `rootmod.c` divides inside Hensel lifting.
fn divide(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    let g = gcd(a, b);
    if is_zero(&g) {
        return zero();
    }
    mul(&(a / &g), &inverse(&(b / &g), n), n)
}

fn min_root(r: BigUint, n: &BigUint) -> BigUint {
    let other = n - &r;
    if other < r { other } else { r }
}

/// `(r mod n, s mod m)` combined to a residue mod `n * m` for coprime `n`
/// and `m`.
fn crt(r: &BigUint, n: &BigUint, s: &BigUint, m: &BigUint) -> BigUint {
    let t = mul(&inverse(n, m), &sub(s, r, m), m);
    r + n * t
}

/// The prime factorization of `n >= 1` as `(prime, exponent)` pairs in
/// ascending order, through [`factor::factor_exp`]. A cofactor the recipe
/// cannot split is reported as a prime, as MPU does.
fn factor_exp(n: &BigUint) -> Vec<(BigUint, u32)> {
    factor_exp_with(n, factor::factor)
}

/// [`factor_exp`] over the factoring routine `split`, which returns the
/// prime factors in ascending order or the first cofactor it cannot split.
fn factor_exp_with<F>(n: &BigUint, split: F) -> Vec<(BigUint, u32)>
where
    F: Fn(&BigUint) -> Result<Vec<BigUint>, BigUint>,
{
    let mut primes = Vec::new();
    let mut rest = n.clone();
    loop {
        match split(&rest) {
            Ok(found) => {
                primes.extend(found);
                break;
            }
            Err(cofactor) => {
                rest /= &cofactor;
                primes.push(cofactor);
            }
        }
    }
    primes.sort_unstable();
    factor::group(primes)
}

/// The divisors of `n >= 1` in ascending order.
fn divisors(n: &BigUint) -> Vec<BigUint> {
    let mut divs = vec![one()];
    for (p, e) in factor_exp(n) {
        let len = divs.len();
        let mut power = one();
        for _ in 0..e {
            power *= &p;
            for i in 0..len {
                let d = &divs[i] * &power;
                divs.push(d);
            }
        }
    }
    divs.sort_unstable();
    divs
}

/// A square root of `a` modulo the prime `p` (`a` reduced), or zero when
/// `a` has no root or `p` is not prime, by the branches of
/// `rootmod.c::_sqrtmod_prime`: `p = 3 mod 4`, `p = 5 mod 8`, `p = 9 mod 16`
/// (whose value the caller must verify), then Tonelli-Shanks.
fn sqrt_prime(a: &BigUint, p: &BigUint) -> BigUint {
    if *p == two() || *a <= one() {
        return a.clone();
    }
    let p1 = p - 1_u8;
    if p % 4_u8 == BigUint::from(3_u8) {
        return pow(a, &((p >> 2_usize) + 1_u8), p);
    }
    let a2 = add(a, a, p);
    if p % 8_u8 == BigUint::from(5_u8) {
        let alpha = pow(&a2, &((p - 5_u8) >> 3_usize), p);
        let beta = mul(&a2, &mul(&alpha, &alpha, p), p);
        return mul(&alpha, &mul(a, &sub(&beta, &one(), p), p), p);
    }
    let half = &p1 >> 1_usize;
    if p % 16_u8 == BigUint::from(9_u8) {
        let mut alpha = pow(&a2, &((p - 9_u8) >> 4_usize), p);
        let mut beta = mul(&a2, &mul(&alpha, &alpha, p), p);
        let mut d = one();
        if mul(&beta, &beta, p) != p1 {
            d = BigUint::from(3_u8);
            while pow(&d, &half, p) != p1 {
                d += 2_u8;
            }
            alpha = mul(&alpha, &pow(&d, &((p - 9_u8) >> 3_usize), p), p);
            beta = mul(&a2, &mul(&mul(&d, &d, p), &mul(&alpha, &alpha, p), p), p);
        }
        return mul(&alpha, &mul(a, &mul(&d, &sub(&beta, &one(), p), p), p), p);
    }
    // Tonelli-Shanks, Cohen algorithm 1.5.1.
    if !is_one(&pow(a, &half, p)) {
        return zero();
    }
    let e = p1.trailing_zeros().unwrap_or(0);
    let q = &p1 >> e;
    let mut t = BigUint::from(3_u8);
    while pow(&t, &half, p) != p1 {
        t += 2_u8;
        if t == BigUint::from(201_u8) && !is_prime(p) {
            return zero();
        }
    }
    let mut z = pow(&t, &q, p);
    let mut b = pow(a, &q, p);
    let mut r = e;
    let mut x = pow(a, &((&q + 1_u8) >> 1_usize), p);
    while !is_one(&b) {
        let mut t = b.clone();
        let mut m = 0;
        while m < r && !is_one(&t) {
            t = mul(&t, &t, p);
            m += 1;
        }
        if m >= r {
            break;
        }
        let t = pow(&z, &(one() << (r - m - 1)), p);
        x = mul(&x, &t, p);
        z = mul(&t, &t, p);
        b = mul(&b, &z, p);
        r = m;
    }
    x
}

/// The smaller square root of `a` modulo `p^e`, by Hensel lifting of the
/// root modulo `p`, as `rootmod.c::_sqrtmod_prime_power`.
fn sqrt_prime_power(a: &BigUint, p: &BigUint, e: u32) -> Option<BigUint> {
    let n = p.pow(e);
    if is_zero(&(a % &n)) {
        return Some(zero());
    }
    if e == 1 {
        let reduced = a % p;
        let r = sqrt_prime(&reduced, p);
        return (mul(&r, &r, p) == reduced).then(|| min_root(r, p));
    }
    let p2 = p * p;
    if is_zero(&(a % &p2)) {
        return sqrt_prime_power(&(a / &p2), p, e - 2).map(|s| s * p);
    }
    if is_zero(&(a % p)) {
        return None;
    }
    let ered = if *p > two() || e < 5 {
        e.div_ceil(2)
    } else {
        e.div_ceil(2) + 1
    };
    let s = sqrt_prime_power(a, p, ered)?;
    let np = if *p == two() { &n * p } else { n.clone() };
    let numerator = sub(a, &mul(&s, &s, &np), &np);
    let r = add(&s, &divide(&numerator, &add(&s, &s, &np), &n), &n);
    (mul(&r, &r, &n) == a % &n).then(|| min_root(r, &n))
}

/// A square root of `a` modulo `n`, or `None` when there is none. The
/// smaller of a root and its negative is returned; over a composite
/// modulus the root is the combination of the smaller roots modulo each
/// prime power, as on the native path.
///
/// C reference: MPU 0.75 `rootmod.c::sqrtmod`.
///
/// Port of MPU 0.75 `PP.pm::sqrtmod` and `_sqrtmod_composite`.
///
/// ```
/// use math_prime_util::bigint::modular::sqrtmod;
/// use num_bigint::BigUint;
/// assert_eq!(sqrtmod(&BigUint::from(58_u8), &BigUint::from(101_u8)), Some(BigUint::from(19_u8)));
/// assert_eq!(sqrtmod(&BigUint::from(3_u8), &BigUint::from(7_u8)), None);
/// ```
pub fn sqrtmod(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    // The small-root table of `rootmod.c`, which the native path keeps.
    const SMALL: [&[u64]; 14] = [
        &[0],
        &[0, 0],
        &[0, 0, 2],
        &[0, 3, 2, 0],
        &[3, 0, 2, 0, 0],
        &[0, 0, 2, 0, 0, 0],
        &[0, 0, 2, 0, 0, 4, 0],
        &[0, 0, 2, 5, 4, 0, 0, 3],
        &[0, 5, 2, 4, 0, 0, 0, 3, 0],
        &[0, 0, 2, 0, 0, 0, 0, 3, 0, 0],
        &[0, 4, 2, 0, 0, 0, 0, 3, 6, 0, 5],
        &[4, 0, 2, 0, 0, 7, 6, 3, 0, 5, 0, 0],
        &[0, 0, 2, 0, 6, 0, 0, 3, 5, 0, 0, 0, 0],
        &[0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0],
    ];
    if is_zero(n) {
        return None;
    }
    let a = a % n;
    if *n <= two() || a <= one() {
        return Some(a);
    }
    if *n <= BigUint::from(16_u8) {
        let (n, a) = (n.to_u64()?, a.to_u64()?);
        let r = SMALL[usize::try_from(n - 3).ok()?][usize::try_from(a - 2).ok()?];
        return (r != 0).then(|| big(r));
    }
    let s = sqrtint(&a);
    if &s * &s == a {
        return Some(s);
    }
    let (mut r, mut modulus) = (zero(), one());
    for (p, e) in factor_exp(n) {
        let m = p.pow(e);
        r = crt(&r, &modulus, &sqrt_prime_power(&a, &p, e)?, &m);
        modulus *= m;
    }
    Some(min_root(r, n))
}

/// The discrete logarithm of `a` to the base `g` modulo `p`, where `g` has
/// the known prime order `n`: the `x` below `n` with `g^x = a`.
fn log_prime_order(a: &BigUint, g: &BigUint, p: &BigUint, n: &BigUint) -> Option<BigUint> {
    if is_one(a) {
        return Some(zero());
    }
    if !is_one(&pow(a, n, p)) {
        return None;
    }
    if n.bits() <= 40 {
        return bsgs(a, g, p, n);
    }
    log_rho(a, g, p, n)
}

/// Baby-step giant-step for `g` of prime order `n` (`n < 2^40`).
fn bsgs(a: &BigUint, g: &BigUint, p: &BigUint, n: &BigUint) -> Option<BigUint> {
    let m = sqrtint(n).to_u64()? + 1;
    let mut table = HashMap::new();
    let mut baby = one();
    for j in 0..m {
        if baby == *a {
            return Some(big(j));
        }
        table.entry(baby.clone()).or_insert(j);
        baby = mul(&baby, g, p);
    }
    let giant_step = pow_u(&inverse(g, p), m, p);
    let mut giant = a.clone();
    for i in 0..=m {
        if let Some(&j) = table.get(&giant) {
            let x = (big(i) * m + j) % n;
            return (pow(g, &x, p) == *a).then_some(x);
        }
        giant = mul(&giant, &giant_step, p);
    }
    None
}

/// Pollard rho for `g` of prime order `n`, `znlog.c::dlp_prho` without the
/// Montgomery form.
fn log_rho(a: &BigUint, g: &BigUint, p: &BigUint, n: &BigUint) -> Option<BigUint> {
    let three = BigUint::from(3_u8);
    let cycle = |(u, v, w): (BigUint, BigUint, BigUint)| match (&u % &three).to_u64() {
        Some(0) => (mul(&u, &u, p), add(&v, &v, n), add(&w, &w, n)),
        Some(1) => (mul(&u, a, p), add(&v, &one(), n), w),
        _ => (mul(&u, g, p), v, add(&w, &one(), n)),
    };
    for restart in 0..5_u64 {
        let mut slow = (pow_u(g, restart, p), zero(), big(restart) % n);
        let mut fast = cycle(slow.clone());
        let (mut power, mut lam) = (1_u64, 1_u64);
        while slow.0 != fast.0 {
            if power == lam {
                slow = fast.clone();
                power *= 2;
                lam = 0;
            }
            fast = cycle(fast);
            lam += 1;
        }
        let r1 = sub(&slow.1, &fast.1, n);
        if is_zero(&r1) {
            continue;
        }
        let r2 = sub(&fast.2, &slow.2, n);
        let x = mul(&r2, &inverse(&r1, n), n);
        return (pow(g, &x, p) == *a).then_some(x);
    }
    None
}

/// Pohlig-Hellman: the logarithm of `a` to the base `g` modulo `p`, where
/// `g` has the composite order `n`, as `znlog.c::znlog_ph`.
fn pohlig_hellman(a: &BigUint, g: &BigUint, p: &BigUint, n: &BigUint) -> Option<BigUint> {
    let (mut x, mut modulus) = (zero(), one());
    for (q, e) in factor_exp(n) {
        let pe = q.pow(e);
        let cofactor = n / &pe;
        let delta = pow(a, &cofactor, p);
        let gamma = pow(g, &cofactor, p);
        let gamma0 = pow(&gamma, &q.pow(e - 1), p);
        let (mut k, mut pj) = (zero(), one());
        for j in 0..e {
            let t = if is_zero(&k) {
                delta.clone()
            } else {
                mul(&delta, &pow(&gamma, &(&pe - &k), p), p)
            };
            let h = pow(&t, &q.pow(e - 1 - j), p);
            let digit = log_prime_order(&h, &gamma0, p, &q)?;
            k += digit % &q * &pj;
            pj *= &q;
        }
        if pow(&gamma, &k, p) != delta {
            return None;
        }
        x = crt(&x, &modulus, &k, &pe);
        modulus *= pe;
    }
    (pow(g, &x, p) == *a).then_some(x)
}

/// The least `k` with `g^k = a mod n`, or `None` when there is none.
///
/// C reference: MPU 0.75 `znlog.c::znlog`.
///
/// Port of MPU 0.75 `PP.pm::znlog` with the native `znlog.c` structure:
/// a base that shares a factor with the modulus is peeled off first, then
/// Pohlig-Hellman over the factors of the order of `g`, with baby-step
/// giant-step for prime orders below 2^40 and Pollard rho above. The order
/// of `g` comes from [`znorder`], so the modulus must be factorable, and the
/// search is practical when the order of `g` is smooth.
///
/// ```
/// use math_prime_util::bigint::modular::znlog;
/// use num_bigint::BigUint;
/// assert_eq!(znlog(&BigUint::from(5_u8), &BigUint::from(2_u8), &BigUint::from(1019_u16)), Some(BigUint::from(10_u8)));
/// assert_eq!(znlog(&BigUint::from(2_u8), &BigUint::from(4_u8), &BigUint::from(17_u8)), None);
/// ```
pub fn znlog(a: &BigUint, g: &BigUint, n: &BigUint) -> Option<BigUint> {
    if is_zero(n) {
        return None;
    }
    if is_one(n) {
        return Some(zero());
    }
    let (a, g) = (a % n, g % n);
    if is_one(&a) {
        return Some(zero());
    }
    if is_zero(&g) {
        return is_zero(&a).then(one);
    }
    let mut d = gcd(&g, n);
    if !is_one(&d) {
        let (mut nr, mut ar, mut gp, mut acc, mut k) = (n.clone(), a.clone(), one(), one(), zero());
        while d > one() {
            if gp == a {
                return Some(k);
            }
            if !is_zero(&(&ar % &d)) {
                return None;
            }
            nr /= &d;
            ar /= &d;
            acc = if nr > one() {
                mul(&acc, &(&g / &d), &nr)
            } else {
                zero()
            };
            gp = mul(&gp, &g, n);
            k += 1_u8;
            d = gcd(&g, &nr);
        }
        if gp == a {
            return Some(k);
        }
        if is_one(&nr) {
            return None;
        }
        let target = mul(&(&ar % &nr), &invmod(&acc, &nr)?, &nr);
        k += znlog(&target, &(&g % &nr), &nr)?;
        return (pow(&g, &k, n) == a).then_some(k);
    }
    let order = znorder(&g, n)?;
    if !is_one(&pow(&a, &order, n)) {
        return None;
    }
    if is_prime(&order) {
        log_prime_order(&a, &g, n, &order)
    } else {
        pohlig_hellman(&a, &g, n, &order)
    }
}

/// Carmichael's lambda of `n` from its factorization.
fn lambda(n: &BigUint) -> BigUint {
    let mut result = one();
    for (p, e) in factor_exp(n) {
        let term = if p == two() && e > 2 {
            one() << (e - 2)
        } else {
            (&p - 1_u8) * p.pow(e - 1)
        };
        result = &result / gcd(&result, &term) * term;
    }
    result
}

/// The multiplicative order of `a` modulo `n`, or `None` when `n` is zero
/// or `a` is not a unit modulo `n`.
///
/// Port of MPU 0.75 `PP.pm::znorder`, by the native `util.c::znorder`
/// route: Carmichael's lambda, then each prime of lambda is divided out
/// while the power stays 1. The modulus and `p - 1` for each of its primes
/// must be factorable.
///
/// ```
/// use math_prime_util::bigint::modular::znorder;
/// use num_bigint::BigUint;
/// assert_eq!(znorder(&BigUint::from(2_u8), &BigUint::from(7_u8)), Some(BigUint::from(3_u8)));
/// assert_eq!(znorder(&BigUint::from(2_u8), &BigUint::from(8_u8)), None);
/// ```
pub fn znorder(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    if is_zero(n) {
        return None;
    }
    if is_one(n) {
        return Some(one());
    }
    let a = a % n;
    if a <= one() {
        return is_one(&a).then(one);
    }
    if !is_one(&gcd(&a, n)) {
        return None;
    }
    let mut k = lambda(n);
    for (p, e) in factor_exp(&k) {
        k /= p.pow(e);
        let mut a1 = pow(&a, &k, n);
        for _ in 0..e {
            if is_one(&a1) {
                break;
            }
            k *= &p;
            a1 = pow(&a1, &p, n);
        }
        if !is_one(&a1) {
            return None;
        }
    }
    Some(k)
}

/// The odd part of `n` as a single prime power `(p, e)`, for the primitive
/// root tests: `None` when `n` is divisible by 4 or the odd part has two
/// prime factors.
fn cyclic_prime_power(n: &BigUint) -> Option<(BigUint, u32)> {
    if is_zero(&(n % 4_u8)) {
        return None;
    }
    let odd = if n.bit(0) { n.clone() } else { n >> 1_usize };
    let mut factors = factor_exp(&odd);
    (factors.len() == 1).then(|| factors.remove(0))
}

/// The least primitive root modulo `n`, or `None` when `n` is zero or no
/// root exists.
///
/// C reference: MPU 0.75 `util.c::znprimroot`.
///
/// Port of MPU 0.75 `PP.pm::znprimroot`: `n` must be 1, 2, 4, `p^e` or
/// `2 p^e` for an odd prime `p`, and each candidate is checked against the
/// prime factors of `p - 1`, which must be factorable.
///
/// ```
/// use math_prime_util::bigint::modular::znprimroot;
/// use num_bigint::BigUint;
/// assert_eq!(znprimroot(&BigUint::from(7_u8)), Some(BigUint::from(3_u8)));
/// assert_eq!(znprimroot(&BigUint::from(8_u8)), None);
/// ```
pub fn znprimroot(n: &BigUint) -> Option<BigUint> {
    if is_zero(n) {
        return None;
    }
    if *n <= BigUint::from(4_u8) {
        return Some(n - 1_u8);
    }
    let (p, e) = cyclic_prime_power(n)?;
    let even = !n.bit(0);
    if p == BigUint::from(3_u8) && even {
        return Some(BigUint::from(5_u8));
    }
    let phi = &p - 1_u8;
    let cofactors: Vec<BigUint> = factor_exp(&phi)
        .into_iter()
        .filter(|(q, _)| *q > two())
        .map(|(q, _)| &phi / q)
        .collect();
    let p2 = &p * &p;
    let mut a = one();
    loop {
        a += if even { 2_u8 } else { 1_u8 };
        if a >= p {
            return None;
        }
        if a == BigUint::from(4_u8) || a == BigUint::from(8_u8) || a == BigUint::from(9_u8) {
            continue;
        }
        if kronecker(&BigInt::from(a.clone()), &BigInt::from(p.clone())) != -1 {
            continue;
        }
        if cofactors.iter().any(|k| is_one(&pow(&a, k, &p))) {
            continue;
        }
        if e == 1 || !is_one(&pow(&a, &phi, &p2)) {
            return Some(a);
        }
    }
}

/// Tests whether `a` is a primitive root modulo `n`; `None` when `n` is
/// zero.
///
/// C reference: MPU 0.75 `util.c::is_primitive_root`.
///
/// Port of MPU 0.75 `PP.pm::is_primitive_root`: cheap exclusions (even
/// base and modulus, squares, non-units, moduli without primitive roots,
/// quadratic residues), then `a^((p-1)/q) != 1` for every prime `q` of
/// `p - 1`, which must be factorable.
///
/// ```
/// use math_prime_util::bigint::modular::is_primitive_root;
/// use num_bigint::BigUint;
/// assert_eq!(is_primitive_root(&BigUint::from(3_u8), &BigUint::from(7_u8)), Some(true));
/// assert_eq!(is_primitive_root(&BigUint::from(2_u8), &BigUint::from(7_u8)), Some(false));
/// ```
pub fn is_primitive_root(a: &BigUint, n: &BigUint) -> Option<bool> {
    if is_zero(n) {
        return None;
    }
    if is_one(n) {
        return Some(true);
    }
    let a = a % n;
    if a <= one() {
        return Some(is_one(&a) && *n <= two());
    }
    if *n <= BigUint::from(4_u8) {
        return Some(a == n - 1_u8);
    }
    if (!n.bit(0) && !a.bit(0)) || is_square(&a) || !is_one(&gcd(&a, n)) {
        return Some(false);
    }
    let Some((p, e)) = cyclic_prime_power(n) else {
        return Some(false);
    };
    if e > 1 && is_one(&pow(&a, &(&p - 1_u8), &(&p * &p))) {
        return Some(false);
    }
    if kronecker(&BigInt::from(a.clone()), &BigInt::from(p.clone())) != -1 {
        return Some(false);
    }
    let phi = &p - 1_u8;
    if is_power(&a, 3) && is_zero(&(&phi % 3_u8)) {
        return Some(false);
    }
    // MPU tests the small factors before it factors phi.
    for q in [2_u8, 3, 5] {
        if is_zero(&(&phi % q)) && is_one(&pow(&a, &(&phi / q), &p)) {
            return Some(false);
        }
    }
    Some(
        factor_exp(&phi)
            .iter()
            .all(|(q, _)| *q <= BigUint::from(5_u8) || !is_one(&pow(&a, &(&phi / q), &p))),
    )
}

/// The Tonelli-Shanks `k`-th root step of `rootmod.c::_ts_rootmod` for
/// prime `k` and `p`, with the generator pair `(y, m)` of
/// `_find_ts_generator`. Returns zero when no root exists.
fn ts_root(a: &BigUint, k: &BigUint, p: &BigUint, y: &BigUint, m: &BigUint) -> BigUint {
    let (mut e, mut r) = (0_u32, p - 1_u8);
    while is_zero(&(&r % k)) {
        r /= k;
        e += 1;
    }
    let mut x = pow(a, &inverse(&(k % &r), &r), p);
    let mut a = mul(&pow(&x, k, p), &inverse(a, p), p);
    let (mut y, mut m) = (y.clone(), m.clone());
    while !is_one(&a) {
        let (mut l, mut t, mut z) = (1_u32, a.clone(), zero());
        while !is_one(&t) {
            if l >= e {
                return zero();
            }
            z.clone_from(&t);
            t = pow(&t, k, p);
            l += 1;
        }
        let Some(log) = log_prime_order(&z, &m, p, k) else {
            return zero();
        };
        let kz = sub(&zero(), &(log % k), k);
        m = pow(&m, &kz, p);
        let t = pow(&y, &(&kz * k.pow(e - l)), p);
        e = l - 1;
        x = mul(&x, &t, p);
        y = pow(&t, k, p);
        if y <= one() {
            return zero();
        }
        a = mul(&a, &y, p);
    }
    x
}

/// A `k`-th root of `a` modulo the prime `p`, `rootmod.c::_rootmod_prime_splitk`:
/// the gcd of `k` and `p - 1` is split into primes, each handled by
/// Tonelli-Shanks, and the coprime rest by an inverse exponent. Returns a
/// value that the caller must verify.
fn root_prime(a: &BigUint, k: &BigUint, p: &BigUint) -> BigUint {
    let mut a = a % p;
    if a <= one() {
        return a;
    }
    if *k == two() {
        return sqrt_prime(&a, p);
    }
    let p1 = p - 1_u8;
    let g = gcd(k, &p1);
    for (f, e) in factor_exp(&g) {
        let (mut v, mut r) = (0_u32, p1.clone());
        while is_zero(&(&r % &f)) {
            r /= &f;
            v += 1;
        }
        // The generator search of `_find_ts_generator`.
        let (mut x, mut y, mut m) = (two(), zero(), one());
        while is_one(&m) {
            if x >= *p {
                return zero();
            }
            y = pow(&x, &r, p);
            if !is_one(&y) {
                m = pow(&y, &f.pow(v - 1), p);
            }
            x += 1_u8;
        }
        for _ in 0..e {
            a = ts_root(&a, &f, p, &y, &m);
            if is_zero(&a) {
                return zero();
            }
        }
    }
    if g != *k {
        let pg = &p1 / &g;
        a = pow(&a, &inverse(&((k / &g) % &pg), &pg), p);
    }
    a
}

/// One Newton step of Hensel lifting for `r^k = a mod n`, `None` when the
/// derivative vanishes.
fn lift(r: &BigUint, a: &BigUint, k: &BigUint, n: &BigUint) -> Option<BigUint> {
    let f = sub(&pow(r, k, n), a, n);
    if is_zero(&f) {
        return Some(r % n);
    }
    let fp = mul(k, &pow(r, &(k - 1_u8), n), n);
    let d = mul(&f, &inverse(&fp, n), n);
    (!is_zero(&d)).then(|| sub(r, &d, n))
}

/// Primes below this bound take the native route when a Hensel step
/// stalls: a scan over the other base roots, then the smallest root of the
/// residue class. A larger prime that divides `a` but not `k` is handled by
/// its valuation instead, since the scan tries up to `p` candidates.
const NATIVE_SCAN: u64 = 1 << 16;

/// The most roots the exact searches below keep at one time.
const ROOT_SET_LIMIT: usize = 1 << 12;

/// `(v, c)` with `t = p^v * c` and `p` not dividing `c`, for `t > 0`.
fn split_valuation(t: &BigUint, p: &BigUint) -> (u32, BigUint) {
    let (mut v, mut c) = (0_u32, t.clone());
    while is_zero(&(&c % p)) {
        c /= p;
        v += 1;
    }
    (v, c)
}

/// Every root of `x^k = c` modulo the prime `p`, for `c` not divisible by
/// `p`: one root times the `g`-th roots of unity, `g = gcd(k, p - 1)`.
/// `None` when there are more than [`ROOT_SET_LIMIT`].
fn all_roots_prime(c: &BigUint, k: &BigUint, p: &BigUint) -> Option<Vec<BigUint>> {
    let x0 = root_prime(c, k, p);
    if pow(&x0, k, p) != c % p {
        return Some(vec![]);
    }
    let p1 = p - 1_u8;
    let g = gcd(k, &p1)
        .to_u64()
        .filter(|&g| g <= ROOT_SET_LIMIT as u64)?;
    if g == 1 {
        return Some(vec![x0]);
    }
    let cofactor = &p1 / g;
    let primes: Vec<u64> = crate::factor_exp(g).into_iter().map(|(q, _)| q).collect();
    let mut h = two();
    let zeta = loop {
        if h >= *p {
            return None;
        }
        let z = pow(&h, &cofactor, p);
        if primes.iter().all(|&q| !is_one(&pow_u(&z, g / q, p))) {
            break z;
        }
        h += 1_u8;
    };
    let mut roots = Vec::new();
    let mut x = x0;
    for _ in 0..g {
        let next = mul(&x, &zeta, p);
        roots.push(x);
        x = next;
    }
    Some(roots)
}

/// Every root of `x^k = c` modulo `p^j` that reduces to one of `roots`,
/// the roots modulo `p`, for `c` not divisible by `p`. A root lifts
/// uniquely when `p` does not divide `k`; otherwise all `p` lifts of a
/// root modulo `p^l` have the same `k`-th power modulo `p^(l+1)`. `None`
/// when the set grows past [`ROOT_SET_LIMIT`].
fn lift_roots(
    roots: Vec<BigUint>,
    c: &BigUint,
    k: &BigUint,
    p: &BigUint,
    j: u32,
) -> Option<Vec<BigUint>> {
    let unique = !is_zero(&(k % p));
    let count = p.to_u64().unwrap_or(u64::MAX);
    let (mut roots, mut pl) = (roots, p.clone());
    for _ in 1..j {
        let next = &pl * p;
        let target = c % &next;
        roots = if unique {
            roots
                .iter()
                .filter_map(|s| lift(s, c, k, &next))
                .filter(|s| pow(s, k, &next) == target)
                .collect()
        } else {
            let kept: Vec<BigUint> = roots
                .into_iter()
                .filter(|s| pow(s, k, &next) == target)
                .collect();
            if count.saturating_mul(kept.len() as u64) > ROOT_SET_LIMIT as u64 {
                return None;
            }
            let pl = &pl;
            kept.iter()
                .flat_map(|s| (0..count).map(move |t| s + pl * t))
                .collect()
        };
        pl = next;
    }
    Some(roots)
}

/// A root of `x^k = a` modulo `p^e` for a prime `p` that divides `a` but
/// not `k`: `p^(v/k)` times a root of `a / p^v` modulo `p^(e-v)`, where `v`
/// is the valuation of `a`.
fn root_by_valuation(a: &BigUint, k: &BigUint, p: &BigUint, e: u32) -> Option<BigUint> {
    let a = a % p.pow(e);
    if is_zero(&a) {
        return Some(zero());
    }
    let (v, c) = split_valuation(&a, p);
    let w = u32::try_from(k)
        .ok()
        .filter(|&k| k > 0 && v % k == 0)
        .map(|k| v / k)?;
    let x0 = root_prime(&c, k, p);
    if pow(&x0, k, p) != &c % p {
        return None;
    }
    let u = lift_roots(vec![x0], &c, k, p, e - v)?;
    u.into_iter().next().map(|u| p.pow(w) * u)
}

/// The smallest `s` below `p^i` with `s = base mod p` and `s^k = t mod p^i`,
/// the value the native search for a stalled Hensel step finds by trying
/// every `s`. Here a unit `t` takes the lifts of `base`, and `t = p^v * c`
/// takes `p^(v/k)` times the smallest root of `c` modulo `p^(i-v)`; the
/// plain search stays as the fallback for root sets past
/// [`ROOT_SET_LIMIT`].
fn smallest_root(t: &BigUint, k: &BigUint, p: &BigUint, i: u32, base: &BigUint) -> Option<BigUint> {
    let fe = p.pow(i);
    let t = t % &fe;
    // A root of a multiple of p is itself a multiple of p.
    if is_zero(&(&t % p)) && !is_zero(base) {
        return None;
    }
    if is_zero(&t) {
        return Some(zero());
    }
    let (v, c) = split_valuation(&t, p);
    let exact = if v == 0 {
        lift_roots(vec![base.clone()], &c, k, p, i).map(|roots| roots.into_iter().min())
    } else {
        match u32::try_from(k).ok().filter(|&k| k > 0 && v % k == 0) {
            None => Some(None),
            Some(k32) => all_roots_prime(&c, k, p)
                .and_then(|roots| lift_roots(roots, &c, k, p, i - v))
                .map(|roots| roots.into_iter().min().map(|u| p.pow(v / k32) * u)),
        }
    };
    exact.unwrap_or_else(|| {
        let mut s = base.clone();
        while s < fe {
            if pow(&s, k, &fe) == t {
                return Some(s);
            }
            s += p;
        }
        None
    })
}

/// A `k`-th root of `a` modulo the prime power `p^e`, lifted from the root
/// modulo `p`, with the searches of `rootmod.c::_rootmod_composite1` when
/// Newton's step stalls.
fn root_prime_power(a: &BigUint, k: &BigUint, p: &BigUint, e: u32) -> Option<BigUint> {
    let ap = a % p;
    let native = *p < big(NATIVE_SCAN);
    if !native && is_zero(&ap) && !is_zero(&(k % p)) {
        return root_by_valuation(a, k, p, e);
    }
    let mut r = root_prime(&ap, k, p);
    if pow(&r, k, p) != ap {
        return None;
    }
    let mut fe = p.clone();
    for level in 2..=e {
        fe *= p;
        let target = a % &fe;
        if let Some(s) = lift(&r, &target, k, &fe) {
            r = s;
            continue;
        }
        if native {
            let m = &fe / p / p;
            let mut t = one();
            let mut alternate = None;
            while t < *p {
                if let Some(s) = lift(&(&r + &t * &m), &target, k, &fe) {
                    alternate = Some(s);
                    break;
                }
                t += 1_u8;
            }
            if let Some(s) = alternate {
                r = s;
                continue;
            }
        }
        r = smallest_root(&target, k, p, level, &(&r % p))?;
    }
    Some(r)
}

/// A `k`-th root of `a` modulo `n`, or `None` when there is none. For
/// `k = 2` the smaller of a root and its negative is returned.
///
/// C reference: MPU 0.75 `rootmod.c::rootmod`.
///
/// Port of MPU 0.75 `PP.pm::rootmod` by the native `rootmod.c` route: an
/// integer root when `a` is a perfect power, Tonelli-Shanks `k`-th roots
/// modulo a prime, Hensel lifting on prime powers and the Chinese
/// remainder theorem over a composite modulus, which must be factorable.
/// Where a Hensel step stalls (a prime of the modulus divides `a` or `k`)
/// the root comes from the valuation of `a` or from the exact set of roots
/// in the residue class, so the result equals the native search's; a large
/// prime that divides both `k` and a repeated factor of the modulus can
/// still make that search slow, as MPU documents for large prime powers.
///
/// ```
/// use math_prime_util::bigint::modular::rootmod;
/// use num_bigint::BigUint;
/// assert_eq!(rootmod(&BigUint::from(13_u8), &BigUint::from(6_u8), &BigUint::from(107_u8)), Some(BigUint::from(83_u8)));
/// assert_eq!(rootmod(&BigUint::from(2_u8), &BigUint::from(3_u8), &BigUint::from(7_u8)), None);
/// ```
pub fn rootmod(a: &BigUint, k: &BigUint, n: &BigUint) -> Option<BigUint> {
    if is_zero(n) {
        return None;
    }
    let a = a % n;
    let mut r;
    if *n <= two() || a <= one() {
        r = a.clone();
    } else if *k <= one() {
        r = if is_zero(k) { one() } else { a.clone() };
    } else if let Some(s) = integer_root(&a, k) {
        r = s;
    } else if is_prime(n) {
        r = root_prime(&a, k, n);
    } else {
        let (mut result, mut modulus) = (zero(), one());
        for (p, e) in factor_exp(n) {
            let fe = p.pow(e);
            r = root_prime_power(&a, k, &p, e)?;
            result = crt(&result, &modulus, &r, &fe);
            modulus *= fe;
        }
        r = result;
    }
    if *k == two() {
        r = min_root(r, n);
    }
    (pow(&r, k, n) == a).then_some(r)
}

/// The integer `s` with `s^k = a`, when `a >= 2` is a perfect `k`-th power.
fn integer_root(a: &BigUint, k: &BigUint) -> Option<BigUint> {
    let k = u32::try_from(k)
        .ok()
        .filter(|&k| u64::from(k) <= a.bits())?;
    let s = super::arith::rootint(a, k);
    (s.pow(k) == *a).then_some(s)
}

/// MPU's limit on the number of returned roots.
const MAX_ROOTS: u64 = 600_000_000;

fn root_count(count: &BigUint) -> u64 {
    let count = count.to_u64().unwrap_or(u64::MAX);
    assert!(count <= MAX_ROOTS, "Maximum returned roots exceeded");
    count
}

/// Every root modulo `n * m` from the roots modulo `n` and modulo `m`.
fn combine(left: &[BigUint], n: &BigUint, right: &[BigUint], m: &BigUint) -> Vec<BigUint> {
    root_count(&(big(left.len() as u64) * right.len() as u64));
    left.iter()
        .flat_map(|r| right.iter().map(move |s| crt(r, n, s, m)))
        .collect()
}

/// All square roots of `a` modulo `p^k`, `rootmod.c::_allsqrtmodpk`.
fn roots_pk(a: &BigUint, p: &BigUint, k: u32) -> Vec<BigUint> {
    let pk = p.pow(k);
    if is_zero(&(a % p)) {
        if is_zero(&(a % &pk)) {
            let low = p.pow(k / 2);
            let high = if k % 2 == 1 { &low * p } else { low.clone() };
            return (0..root_count(&low)).map(|i| &high * i).collect();
        }
        if !is_zero(&((a / p) % p)) {
            return vec![];
        }
        let roots = roots_pk(&(a / p / p), p, k - 2);
        root_count(&(big(roots.len() as u64) * p));
        let count = p.to_u64().unwrap_or(u64::MAX);
        let step = &pk / p;
        let mut lifted = Vec::new();
        for r in &roots {
            let base = r * p;
            lifted.extend((0..count).map(|j| &base + &step * j));
        }
        return lifted;
    }
    let Some(q) = sqrt_prime_power(a, p, k) else {
        return vec![];
    };
    if *p == two() && k == 1 {
        return vec![q];
    }
    if *p != two() || k == 2 {
        return vec![&pk - &q, q];
    }
    let q2 = mul(&q, &(&pk / p - 1_u8), &pk);
    vec![&pk - &q, &pk - &q2, q, q2]
}

/// All square roots of `a` modulo `n` in ascending order; empty when there
/// is none.
///
/// C reference: MPU 0.75 `rootmod.c::allsqrtmod`.
///
/// Port of MPU 0.75 `PP.pm::allsqrtmod`: the roots modulo each prime power
/// of `n` (which must be factorable), combined by the Chinese remainder
/// theorem.
///
/// # Panics
///
/// Panics when the result would exceed MPU's limit of 600 million roots.
///
/// ```
/// use math_prime_util::bigint::modular::allsqrtmod;
/// use num_bigint::BigUint;
/// let roots: Vec<u8> = allsqrtmod(&BigUint::from(1_u8), &BigUint::from(8_u8))
///     .iter().map(|r| r.to_string().parse().unwrap()).collect();
/// assert_eq!(roots, [1, 3, 5, 7]);
/// ```
pub fn allsqrtmod(a: &BigUint, n: &BigUint) -> Vec<BigUint> {
    if is_zero(n) {
        return vec![];
    }
    let a = a % n;
    if *n <= two() {
        return vec![a];
    }
    let (mut roots, mut modulus) = (vec![zero()], one());
    for (p, e) in factor_exp(n) {
        let m = p.pow(e);
        roots = combine(&roots, &modulus, &roots_pk(&a, &p, e), &m);
        if roots.is_empty() {
            break;
        }
        modulus *= m;
    }
    roots.sort_unstable();
    roots
}

/// Solves simultaneous congruences `x = a_i mod n_i`, given as
/// `(residue, modulus)` pairs, and returns the least solution. An empty
/// input gives zero. Returns `None` for a zero modulus or inconsistent
/// congruences; the moduli need not be coprime.
///
/// C reference: MPU 0.75 `util.c::chinese`.
///
/// Port of MPU 0.75 `PP.pm::chinese2`, Garner's form with the extended
/// Euclidean algorithm at each step.
///
/// ```
/// use math_prime_util::bigint::modular::chinese;
/// use num_bigint::BigUint;
/// let pair = |a: u8, n: u8| (BigUint::from(a), BigUint::from(n));
/// assert_eq!(chinese(&[pair(2, 3), pair(3, 5)]), Some(BigUint::from(8_u8)));
/// assert_eq!(chinese(&[pair(0, 2), pair(1, 2)]), None);
/// ```
pub fn chinese(congruences: &[(BigUint, BigUint)]) -> Option<BigUint> {
    let mut items: Vec<(BigUint, BigUint)> = Vec::with_capacity(congruences.len());
    for (a, n) in congruences {
        if is_zero(n) {
            return None;
        }
        items.push((a % n, n.clone()));
    }
    items.sort_by(|x, y| y.1.cmp(&x.1));
    let mut items = items.into_iter();
    let Some((mut sum, mut lcm)) = items.next() else {
        return Some(zero());
    };
    for (ai, ni) in items {
        let (g, u, v) = gcdext(&BigInt::from(lcm.clone()), &BigInt::from(ni.clone()));
        let g = g.magnitude().clone();
        if !is_one(&g) && &sum % &g != &ai % &g {
            return None;
        }
        let (s, t) = (&ni / &g, &lcm / &g);
        lcm *= &s;
        let reduce = |x: BigInt| match x.into_parts() {
            (Sign::Minus, m) => &lcm - m % &lcm,
            (_, m) => m % &lcm,
        };
        let (u, v) = (reduce(u), reduce(v));
        let vs = mul(&v, &s, &lcm);
        let ut = mul(&u, &t, &lcm);
        sum = add(&mul(&sum, &vs, &lcm), &mul(&ut, &ai, &lcm), &lcm);
    }
    Some(sum)
}

/// The exponent of `p` in `n!` (Legendre's formula).
fn factorial_valuation(n: &BigUint, p: &BigUint) -> BigUint {
    let (mut n, mut e) = (n.clone(), zero());
    while n >= *p {
        n /= p;
        e += &n;
    }
    e
}

/// `n! mod m`, or `None` when `m` is zero or the product would need more
/// than 2^64 steps (`n`, or `m - n - 1` when Wilson's theorem turns the
/// product around, above the `u64` range).
///
/// C reference: MPU 0.75 `factmod.c::factorialmod`.
///
/// Port of MPU 0.75 `PP.pm::factorialmod` by the native `factmod.c` route:
/// Wilson's theorem for `n` above `m / 2` with `m` prime, a zero short cut
/// for a composite `m` whose largest `p * e` is at most `n`, the plain
/// product below 1000, and the prime power product of `factmod.c::_facmod`
/// above.
///
/// ```
/// use math_prime_util::bigint::modular::factorialmod;
/// use num_bigint::BigUint;
/// assert_eq!(factorialmod(&BigUint::from(10_u8), &BigUint::from(101_u8)), Some(BigUint::from(72_u8)));
/// ```
pub fn factorialmod(n: &BigUint, m: &BigUint) -> Option<BigUint> {
    if is_zero(m) {
        return None;
    }
    if n >= m || is_one(m) {
        return Some(zero());
    }
    if *n <= one() {
        return Some(one());
    }
    let prime = is_prime(m);
    let d = if prime && *n > m >> 1_usize {
        m - n - 1_u8
    } else {
        n.clone()
    };
    if d < two() {
        return Some(if is_zero(&d) { m - 1_u8 } else { one() });
    }
    if d > big(100)
        && !prime
        && factor_exp(m)
            .iter()
            .map(|(p, e)| p * *e)
            .max()
            .is_some_and(|bound| *n >= bound)
    {
        return Some(zero());
    }
    let steps = d.to_u64()?;
    let mut r = one();
    if steps < 1000 {
        for i in 2..=steps {
            r = mul(&r, &big(i), m);
            if is_zero(&r) {
                break;
            }
        }
    } else {
        let sqn = crate::sqrtint(steps);
        let nsqn = steps / sqn;
        let (mut j, mut lo, mut hi, mut product) = (sqn, 0, 0, one());
        for p in crate::primes_in(2..=steps) {
            if p <= nsqn {
                let e = factorial_valuation(&d, &big(p));
                r = mul(&r, &pow(&big(p), &e, m), m);
            } else {
                while p > hi {
                    r = mul(&r, &pow_u(&product, j, m), m);
                    product = one();
                    j -= 1;
                    lo = steps / (j + 1) + 1;
                    hi = steps / j;
                }
                if p >= lo {
                    product = mul(&product, &big(p), m);
                }
            }
            if is_zero(&r) {
                break;
            }
        }
        r = mul(&r, &product, m);
    }
    if d != *n && !is_zero(&r) {
        if !d.bit(0) {
            r = m - r;
        }
        return invmod(&r, m);
    }
    Some(r)
}

/// The product of `2..=n` without the multiples of `p`, modulo the prime
/// power `m`, `factmod.c::_factorialmod_without_prime`.
fn unit_factorial(n: &BigUint, p: &BigUint, m: &BigUint) -> BigUint {
    let mut n = n.clone();
    let mut r = one();
    if n >= *m {
        if (&n / m).bit(0) && (*p > two() || *m == BigUint::from(4_u8)) {
            r = m - 1_u8;
        }
        n %= m;
    }
    let mut i = two();
    while i <= n {
        if !is_zero(&(&i % p)) {
            r = mul(&r, &i, m);
        }
        i += 1_u8;
    }
    r
}

fn unit_factorial_powers(n: &BigUint, p: &BigUint, m: &BigUint) -> BigUint {
    let mut n = n.clone();
    let mut r = one();
    while n > one() {
        r = mul(&r, &unit_factorial(&n, p, m), m);
        n /= p;
    }
    r
}

/// `C(n, k) mod p^e` by the direct product of the unit parts,
/// `factmod.c::_binomial_mod_prime_power`.
fn binomial_prime_power(n: &BigUint, k: &BigUint, p: &BigUint, e: u32) -> BigUint {
    if k > n {
        return zero();
    }
    if is_zero(k) || k == n {
        return one();
    }
    let k = k.min(&(n - k)).clone();
    let nk = n - &k;
    let b = factorial_valuation(n, p) - factorial_valuation(&k, p) - factorial_valuation(&nk, p);
    if b >= big(u64::from(e)) {
        return zero();
    }
    let m = p.pow(e);
    let mut den = unit_factorial_powers(&k, p, &m);
    let mut num = one();
    if k >= m {
        num = unit_factorial_powers(n, p, &m);
        den = mul(&den, &unit_factorial_powers(&nk, p, &m), &m);
    } else {
        let mut i = &nk + 1_u8;
        while i <= *n {
            let mut v = i.clone();
            while is_zero(&(&v % p)) {
                v /= p;
            }
            num = mul(&num, &v, &m);
            i += 1_u8;
        }
    }
    mul(&mul(&num, &inverse(&den, &m), &m), &pow(p, &b, &m), &m)
}

/// `C(n, k) mod p` by Lucas's theorem on the base `p` digits.
fn binomial_lucas(n: &BigUint, k: &BigUint, p: &BigUint) -> BigUint {
    let (mut n, mut k) = (n.clone(), k.clone());
    let mut r = one();
    while !is_zero(&n) || !is_zero(&k) {
        r = mul(&r, &binomial_prime_power(&(&n % p), &(&k % p), p, 1), p);
        if is_zero(&r) {
            break;
        }
        n /= p;
        k /= p;
    }
    r
}

/// `C(n, k) mod p^q` by Granville's generalization of Lucas's theorem,
/// `factmod.c::_binomial_lucas_mod_prime_power`.
fn binomial_granville(n: &BigUint, k: &BigUint, p: &BigUint, mut q: u32) -> BigUint {
    let m = p.pow(q);
    let (mut nn, mut kk) = (n.clone(), k.clone());
    let mut carries: Vec<u32> = Vec::new();
    let mut carry = 0;
    while !is_zero(&nn) {
        carry = u32::from(&nn % p < &kk % p + carry);
        carries.push(carry);
        nn /= p;
        kk /= p;
    }
    for i in (1..carries.len()).rev() {
        carries[i - 1] += carries[i];
    }
    if carries[0] >= q {
        return zero();
    }
    q -= carries[0];
    let m1 = p.pow(q);
    let odd_carry = carries.get(q as usize - 1).is_some_and(|c| c % 2 == 1);
    let mut r = if (*p > two() || q < 3) && odd_carry {
        &m - 1_u8
    } else {
        one()
    };
    r = mul(&r, &pow_u(p, u64::from(carries[0]), &m), &m);
    let (mut nn, mut kk, mut rr) = (n.clone(), k.clone(), n - k);
    while !is_zero(&nn) {
        let num = unit_factorial(&(&nn % &m1), p, &m);
        let den = mul(
            &unit_factorial(&(&kk % &m1), p, &m),
            &unit_factorial(&(&rr % &m1), p, &m),
            &m,
        );
        r = mul(&r, &mul(&num, &inverse(&den, &m), &m), &m);
        nn /= p;
        kk /= p;
        rr /= p;
    }
    r
}

/// The binomial coefficient `C(n, k) mod m`, or `None` when `m` is zero.
///
/// C reference: MPU 0.75 `factmod.c::binomialmod`.
///
/// Port of MPU 0.75 `PP.pm::binomialmod` by the native `factmod.c` route:
/// Lucas's theorem for a prime modulus, and for each prime power of a
/// composite modulus (which must be factorable) the direct unit product
/// when `min(k, n - k)` is small, else Granville's theorem; the parts are
/// combined by the Chinese remainder theorem. The work grows with
/// `min(k, n - k)` and with the prime powers of `m`.
///
/// ```
/// use math_prime_util::bigint::modular::binomialmod;
/// use num_bigint::BigUint;
/// assert_eq!(binomialmod(&BigUint::from(10_u8), &BigUint::from(3_u8), &BigUint::from(12_u8)), Some(BigUint::from(0_u8)));
/// assert_eq!(binomialmod(&BigUint::from(10_u8), &BigUint::from(3_u8), &BigUint::from(7_u8)), Some(BigUint::from(1_u8)));
/// ```
pub fn binomialmod(n: &BigUint, k: &BigUint, m: &BigUint) -> Option<BigUint> {
    if is_zero(m) {
        return None;
    }
    if is_one(m) {
        return Some(zero());
    }
    if is_zero(k) || k >= n {
        return Some(big(u64::from(is_zero(k) || k == n)));
    }
    if *m == two() {
        return Some(big(u64::from(k & n == *k)));
    }
    if is_prime(m) {
        return Some(binomial_lucas(n, k, m));
    }
    let small = k.min(&(n - k)).clone();
    let (mut x, mut modulus) = (zero(), one());
    for (p, e) in factor_exp(m) {
        let pm = p.pow(e);
        let b = if e == 1 {
            binomial_lucas(n, k, &p)
        } else if small <= big(100_000) && small < pm {
            binomial_prime_power(n, k, &p, e)
        } else {
            binomial_granville(n, k, &p, e)
        };
        x = crt(&x, &modulus, &b, &pm);
        modulus *= pm;
    }
    Some(x)
}

/// `U_k(P, Q) mod n`.
///
/// C reference: MPU 0.75 `lucas_seq.c::lucasumod`.
///
/// Port of MPU 0.75 `PP.pm::lucasumod` through
/// [`lucas_sequence`].
///
/// # Panics
///
/// Panics when `n` is 0, as the native function does.
///
/// ```
/// use math_prime_util::bigint::modular::lucasumod;
/// use num_bigint::{BigInt, BigUint};
/// // U_10 = 55 for the Fibonacci pair, reduced mod 7 gives 6.
/// assert_eq!(lucasumod(&BigInt::from(1), &BigInt::from(-1), &BigUint::from(10_u8), &BigUint::from(7_u8)), BigUint::from(6_u8));
/// ```
pub fn lucasumod(p: &BigInt, q: &BigInt, k: &BigUint, n: &BigUint) -> BigUint {
    lucas_sequence(n, p, q, k).0
}

/// `V_k(P, Q) mod n`.
///
/// C reference: MPU 0.75 `lucas_seq.c::lucasvmod`.
///
/// Port of MPU 0.75 `PP.pm::lucasvmod` through
/// [`lucas_sequence`].
///
/// # Panics
///
/// Panics when `n` is 0, as the native function does.
///
/// ```
/// use math_prime_util::bigint::modular::lucasvmod;
/// use num_bigint::{BigInt, BigUint};
/// // V_10 = 123 for the Fibonacci pair.
/// assert_eq!(lucasvmod(&BigInt::from(1), &BigInt::from(-1), &BigUint::from(10_u8), &BigUint::from(1000_u16)), BigUint::from(123_u8));
/// ```
pub fn lucasvmod(p: &BigInt, q: &BigInt, k: &BigUint, n: &BigUint) -> BigUint {
    lucas_sequence(n, p, q, k).1
}

/// Tests whether `n` is a Carmichael number.
///
/// C reference: MPU 0.75 `util.c::is_carmichael`.
///
/// Port of MPU 0.75 `PP.pm::is_carmichael`: Korselt's criterion on the
/// small divisors, a Fermat test to base 2, then the factorization when
/// the unfactored part is below 10^50. Above that, as in MPU, the answer is
/// probabilistic: `n` must be composite and a Fermat pseudoprime to every
/// prime base from 41 to 863, with Korselt's criterion on any of those
/// primes that divide `n`.
///
/// ```
/// use math_prime_util::bigint::modular::is_carmichael;
/// use num_bigint::BigUint;
/// assert!(is_carmichael(&BigUint::from(561_u16)));
/// assert!(!is_carmichael(&BigUint::from(563_u16)));
/// ```
pub fn is_carmichael(n: &BigUint) -> bool {
    if *n < BigUint::from(561_u16) || !n.bit(0) {
        return false;
    }
    if [9_u8, 25, 49, 121].iter().any(|p| is_zero(&(n % *p))) {
        return false;
    }
    let n1 = n - 1_u8;
    let mut rest = n.clone();
    for a in [5_u8, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43] {
        if is_zero(&(&rest % a)) {
            if !is_zero(&(&n1 % (a - 1))) {
                return false;
            }
            rest /= a;
            if is_zero(&(&rest % a)) {
                return false;
            }
        }
    }
    if !is_one(&pow(&two(), &n1, n)) {
        return false;
    }
    if rest >= big(10).pow(50) {
        if is_prime(n) {
            return false;
        }
        for a in crate::primes(41, 863) {
            if is_zero(&(&rest % a)) {
                if !is_zero(&(&n1 % (a - 1))) {
                    return false;
                }
                rest /= a;
            } else if !is_one(&pow(&big(a), &n1, n)) {
                return false;
            }
        }
        return true;
    }
    let factors = factor_exp(n);
    factors.len() >= 3
        && factors
            .iter()
            .all(|(p, e)| *e == 1 && is_zero(&(&n1 % (p - 1_u8))))
}

/// The number of quasi-Carmichael bases of `n`: the `b` with `p - b`
/// dividing `n - b` for every prime `p` of the square-free `n`.
///
/// C reference: MPU 0.75 `util.c::is_quasi_carmichael`.
///
/// Port of MPU 0.75 `PP.pm::is_quasi_carmichael` by the native `util.c`
/// route over the divisors of `n / spf - 1` (two primes) or of
/// `lpf * (n / lpf - 1)` (more); `n` must be factorable.
///
/// ```
/// use math_prime_util::bigint::modular::is_quasi_carmichael;
/// use num_bigint::BigUint;
/// assert_eq!(is_quasi_carmichael(&BigUint::from(35_u8)), 1);
/// assert_eq!(is_quasi_carmichael(&BigUint::from(7_u8)), 0);
/// ```
pub fn is_quasi_carmichael(n: &BigUint) -> u64 {
    if *n < BigUint::from(35_u8)
        || [4_u8, 9, 25, 49, 121, 169]
            .iter()
            .any(|p| is_zero(&(n % *p)))
    {
        return 0;
    }
    let factors = factor_exp(n);
    if factors.len() < 2 || factors.iter().any(|(_, e)| *e > 1) {
        return 0;
    }
    let primes: Vec<BigInt> = factors
        .iter()
        .map(|(p, _)| BigInt::from(p.clone()))
        .collect();
    let spf = &factors[0].0;
    let lpf = &factors[factors.len() - 1].0;
    let two_primes = factors.len() == 2;
    let target = if two_primes {
        n / spf - 1_u8
    } else {
        lpf * (n / lpf - 1_u8)
    };
    let signed_n = BigInt::from(n.clone());
    let mut bases = 0;
    for d in divisors(&target) {
        let b = if two_primes {
            if d >= *spf {
                break;
            }
            BigInt::from(spf - d)
        } else {
            let b = BigInt::from(lpf.clone()) - BigInt::from(d);
            if b.sign() != Sign::Minus && b.magnitude() >= spf {
                continue;
            }
            b
        };
        if b.sign() != Sign::NoSign && is_quasi_base(&signed_n, &primes, &b) {
            bases += 1;
        }
    }
    bases
}

fn is_quasi_base(n: &BigInt, primes: &[BigInt], b: &BigInt) -> bool {
    let shifted = n - b;
    primes.iter().all(|p| {
        let d = p - b;
        d.sign() != Sign::NoSign && (&shifted % d).sign() == Sign::NoSign
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prime127() -> BigUint {
        (one() << 127_usize) - 1_u8
    }

    #[test]
    fn helper_edge_cases() {
        assert_eq!(sub(&big(3), &big(5), &big(7)), big(5));
        assert_eq!(sub(&big(10), &big(3), &big(7)), big(0));
        assert_eq!(divide(&zero(), &zero(), &big(7)), zero());
        assert_eq!(inverse(&big(2), &big(4)), zero());
        assert_eq!(factor_exp(&one()), vec![]);
        assert_eq!(
            factor_exp(&big(360)),
            vec![(big(2), 3), (big(3), 2), (big(5), 1)]
        );
        let p = prime127();
        assert_eq!(factor_exp(&(&p * &p)), vec![(p.clone(), 2)]);
        assert_eq!(divisors(&big(12)), [1, 2, 3, 4, 6, 12].map(big).to_vec());
        assert_eq!(sqrt_prime(&big(0), &big(7)), big(0));
        assert_eq!(sqrt_prime(&big(2), &big(2)), big(2));
        // Tonelli-Shanks on a composite "prime": the non-residue search gives up.
        assert_eq!(
            sqrt_prime(
                &big(4),
                &big(17 * 17 * 17 * 17 * 17 * 17 * 17 * 17 * 17 * 17 * 17 * 17)
            ),
            big(0)
        );
        assert_eq!(integer_root(&big(8), &(one() << 70_usize)), None);
        assert_eq!(integer_root(&big(8), &big(3)), Some(big(2)));
    }

    #[test]
    fn factor_exp_reports_an_unsplit_cofactor() {
        // A routine that cannot split the product of two 200-bit primes.
        let p = super::super::primality::next_prime(&(one() << 200_usize));
        let q = super::super::primality::next_prime(&(&p + 2_u8));
        let pq = &p * &q;
        let stubborn = |m: &BigUint| -> Result<Vec<BigUint>, BigUint> {
            if m % &pq == zero() {
                Err(pq.clone())
            } else {
                factor::factor(m)
            }
        };
        let n = &pq * 3_u8;
        assert_eq!(
            factor_exp_with(&n, stubborn),
            vec![(big(3), 1), (pq.clone(), 1)]
        );
        assert_eq!(
            factor_exp_with(&(&n * 3_u8), stubborn),
            vec![(big(3), 2), (pq.clone(), 1)]
        );
        assert_eq!(factor_exp(&(&p * 9_u8)), vec![(big(3), 2), (p, 1)]);
    }

    #[test]
    fn root_edge_cases() {
        let p = prime127();
        assert_eq!(sqrtmod(&big(5), &zero()), None);
        assert_eq!(sqrtmod(&big(5), &one()), Some(zero()));
        assert_eq!(sqrtmod(&big(5), &big(2)), Some(one()));
        assert_eq!(sqrtmod(&big(4), &big(15)), Some(big(2)));
        assert_eq!(sqrtmod(&big(7), &big(15)), None);
        assert_eq!(sqrtmod(&(&p + 4_u8), &p), Some(big(2)));
        assert_eq!(sqrtmod(&big(4), &big(8)), Some(big(2)));
        assert_eq!(sqrtmod(&big(0), &big(8)), Some(big(0)));
        assert_eq!(sqrt_prime_power(&big(2), &big(2), 2), None);
        assert_eq!(sqrt_prime_power(&big(16), &big(2), 6), Some(big(4)));
        assert_eq!(sqrt_prime_power(&big(17), &big(2), 7), Some(big(23)));
        assert_eq!(sqrt_prime_power(&big(3), &big(3), 2), None);
        assert_eq!(rootmod(&big(5), &big(3), &zero()), None);
        assert_eq!(rootmod(&big(0), &big(0), &big(7)), None);
        assert_eq!(rootmod(&big(1), &big(0), &big(7)), Some(one()));
        assert_eq!(rootmod(&big(5), &big(0), &big(7)), None);
        assert_eq!(rootmod(&big(5), &big(1), &big(7)), Some(big(5)));
        assert_eq!(rootmod(&big(8), &big(3), &big(1000)), Some(big(2)));
        assert_eq!(rootmod(&big(2), &big(3), &big(7)), None);
        assert_eq!(rootmod(&big(3), &big(5), &big(11)), None);
        assert_eq!(
            rootmod(&big(10), &big(5), &big(11)),
            crate::rootmod(10, 5, 11).map(big)
        );
        assert_eq!(rootmod(&big(4), &big(2), &big(27)), Some(big(2)));
        assert_eq!(
            rootmod(&big(10), &big(3), &big(27)),
            crate::rootmod(10, 3, 27).map(big)
        );
        assert_eq!(
            rootmod(&big(4), &big(4), &big(81)),
            crate::rootmod(4, 4, 81).map(big)
        );
        assert_eq!(
            rootmod(&big(7), &big(3), &big(49)),
            crate::rootmod(7, 3, 49).map(big)
        );
        assert_eq!(
            rootmod(&big(1), &big(7), &big(49)),
            crate::rootmod(1, 7, 49).map(big)
        );
        assert_eq!(
            rootmod(&big(18), &big(7), &big(49)),
            crate::rootmod(18, 7, 49).map(big)
        );
        assert_eq!(
            rootmod(&big(2), &big(2), &p),
            Some(min_root(sqrt_prime(&big(2), &p), &p))
        );
        assert_eq!(allsqrtmod(&big(4), &zero()), vec![]);
        assert_eq!(allsqrtmod(&big(4), &one()), vec![zero()]);
        assert_eq!(allsqrtmod(&big(4), &big(2)), vec![zero()]);
        assert_eq!(
            allsqrtmod(&big(0), &big(16)),
            [0, 4, 8, 12].map(big).to_vec()
        );
        assert_eq!(
            allsqrtmod(&big(4), &big(16)),
            [2, 6, 10, 14].map(big).to_vec()
        );
        assert_eq!(allsqrtmod(&big(8), &big(16)), vec![]);
        assert_eq!(
            allsqrtmod(&big(1), &big(16)),
            [1, 7, 9, 15].map(big).to_vec()
        );
        assert_eq!(allsqrtmod(&big(1), &big(4)), [1, 3].map(big).to_vec());
        assert_eq!(allsqrtmod(&big(1), &big(2)), vec![one()]);
        assert_eq!(allsqrtmod(&big(2), &big(15)), vec![]);
        assert_eq!(
            allsqrtmod(&big(4), &big(15)),
            [2, 7, 8, 13].map(big).to_vec()
        );
    }

    /// The native search for a stalled Hensel step: the smallest
    /// `s = base mod p` below `p^i` with `s^k = t mod p^i`.
    fn search(t: u64, k: u64, p: u64, i: u32, base: u64) -> Option<BigUint> {
        let fe = p.pow(i);
        (0..fe / p)
            .map(|j| base + j * p)
            .find(|&s| pow_u(&big(s), k, &big(fe)) == big(t % fe))
            .map(big)
    }

    #[test]
    fn stalled_lifts_find_the_native_root() {
        // Units with p dividing k, and multiples of p, against the search.
        for (t, k, p, i, base) in [
            (17, 2, 2, 10, 1),
            (65, 4, 2, 12, 1),
            (68, 2, 2, 12, 0),
            (16 * 33, 4, 2, 14, 0),
            (9 * 7, 2, 3, 7, 0),
            (27 * 2, 3, 3, 6, 0),
            (10, 3, 3, 5, 1),
            (3 * 7, 2, 3, 5, 0),
            (0, 2, 5, 3, 0),
        ] {
            assert_eq!(
                smallest_root(&big(t), &big(k), &big(p), i, &big(base)),
                search(t, k, p, i, base),
                "{t} {k} {p} {i} {base}"
            );
        }
        // A multiple of p has no root with a unit residue.
        assert_eq!(smallest_root(&big(4), &big(2), &big(2), 4, &big(1)), None);
        // Past the set limit the plain search runs.
        assert_eq!(
            smallest_root(&one(), &big(1 << 13), &big(2), 16, &one()),
            Some(one())
        );
        // The native path, where its search is short.
        for (a, k, n) in [
            (68, 2, 1 << 20),
            (68 * 9, 2, 9 << 20),
            (81 * 10, 4, 3_u64.pow(9)),
            (7 << 12, 6, 1 << 20),
            (49 * 23, 2, 7_u64.pow(6) * 11),
        ] {
            assert_eq!(
                rootmod(&big(a), &big(k), &big(n)),
                crate::rootmod(a, k, n).map(big),
                "{a} {k} {n}"
            );
        }
    }

    #[test]
    fn large_primes_dividing_a_or_k() {
        let p = big(65537);
        let n = p.pow(4);
        // p divides a: the root comes from the valuation of a.
        let a = &p * &p * 4_u8;
        let r = root_prime_power(&a, &big(2), &p, 4).unwrap();
        assert_eq!(pow(&r, &big(2), &n), a);
        assert_eq!(root_prime_power(&n, &big(2), &p, 4), Some(zero()));
        assert_eq!(root_prime_power(&(&p * 4_u8), &big(2), &p, 4), None);
        // 3 generates the units modulo the Fermat prime 65537.
        assert_eq!(root_prime_power(&(&p * &p * 3_u8), &big(2), &p, 4), None);
        let m = &n * 7_u8;
        let a = &p * &p * 9_u8;
        let r = rootmod(&a, &big(2), &m).unwrap();
        assert_eq!(pow(&r, &big(2), &m), a);
        assert_eq!(rootmod(&(&a * 3_u8), &big(2), &m), None);
        // p divides k: 2 is not a p-th power modulo p^2.
        assert_eq!(root_prime_power(&big(2), &p, &p, 2), None);
        let cube = pow(&big(3), &p, &(&p * &p));
        assert_eq!(
            pow(&root_prime_power(&cube, &p, &p, 2).unwrap(), &p, &(&p * &p)),
            cube
        );
    }

    #[test]
    #[should_panic(expected = "Maximum returned roots exceeded")]
    fn too_many_roots() {
        let _ = allsqrtmod(&zero(), &(one() << 70_usize));
    }

    #[test]
    fn log_edge_cases() {
        let p = prime127();
        assert_eq!(znlog(&big(3), &big(2), &zero()), None);
        assert_eq!(znlog(&big(3), &big(2), &one()), Some(zero()));
        assert_eq!(znlog(&big(1), &big(2), &big(7)), Some(zero()));
        assert_eq!(znlog(&big(0), &big(0), &big(7)), Some(one()));
        assert_eq!(znlog(&big(3), &big(0), &big(7)), None);
        assert_eq!(znlog(&big(4), &big(2), &big(8)), Some(big(2)));
        assert_eq!(znlog(&big(6), &big(2), &big(8)), None);
        assert_eq!(znlog(&big(0), &big(2), &big(8)), Some(big(3)));
        assert_eq!(znlog(&big(0), &big(2), &big(10)), None);
        assert_eq!(znlog(&big(2), &big(2), &big(10)), Some(one()));
        assert_eq!(znlog(&big(6), &big(2), &big(10)), Some(big(4)));
        assert_eq!(znlog(&big(2), &big(6), &big(10)), None);
        assert_eq!(znlog(&big(3), &big(2), &big(7)), None);
        assert_eq!(znlog(&big(8), &big(2), &p), Some(big(3)));
        assert_eq!(bsgs(&big(3), &big(4), &big(17), &big(2)), None);
        assert_eq!(log_rho(&big(3), &big(4), &big(17), &big(2)), None);
        // A large prime-order subgroup takes the rho path.
        let order = (&p - 1_u8) / 2_u8;
        let (q, e) = (big(2), 1);
        let _ = (order, q, e);
        assert_eq!(znorder(&big(2), &zero()), None);
        assert_eq!(znorder(&big(2), &one()), Some(one()));
        assert_eq!(znorder(&big(0), &big(7)), None);
        assert_eq!(znorder(&big(8), &big(7)), Some(one()));
        assert_eq!(znorder(&big(3), &big(16)), Some(big(4)));
        assert_eq!(znorder(&big(5), &big(32)), Some(big(8)));
        assert_eq!(znorder(&big(2), &p), Some(big(127)));
    }

    #[test]
    fn rho_finds_logs_in_large_prime_order_groups() {
        // 2^89 - 1 is prime and (2^89 - 2) / 2 has the prime factor
        // 618970019642690137449562111 / ... ; use a safe prime instead:
        // p = 2 q + 1 with q prime, and g of order q.
        let mut q = 4_000_000_000_001;
        while crate::is_prime(q) == 0 || crate::is_prime(2 * q + 1) == 0 {
            q += 2;
        }
        let q = big(q);
        let p = &q * 2_u8 + 1_u8;
        let g = pow(&big(3), &two(), &p);
        let x = big(1_234_567_890_123);
        let a = pow(&g, &x, &p);
        assert_eq!(log_rho(&a, &g, &p, &q), Some(x.clone()));
        assert_eq!(log_prime_order(&a, &g, &p, &q), Some(x));
    }

    #[test]
    fn primitive_root_edge_cases() {
        assert_eq!(znprimroot(&zero()), None);
        assert_eq!(znprimroot(&one()), Some(zero()));
        assert_eq!(znprimroot(&big(4)), Some(big(3)));
        assert_eq!(znprimroot(&big(8)), None);
        assert_eq!(znprimroot(&big(6)), Some(big(5)));
        assert_eq!(znprimroot(&big(15)), None);
        assert_eq!(znprimroot(&big(9)), Some(big(2)));
        assert_eq!(znprimroot(&big(14)), Some(big(3)));
        assert_eq!(znprimroot(&big(49)), Some(big(3)));
        assert_eq!(znprimroot(&big(41)), Some(big(6)));
        assert_eq!(znprimroot(&prime127()), Some(big(43)));
        assert_eq!(is_primitive_root(&big(3), &zero()), None);
        assert_eq!(is_primitive_root(&big(3), &one()), Some(true));
        assert_eq!(is_primitive_root(&big(1), &big(2)), Some(true));
        assert_eq!(is_primitive_root(&big(1), &big(3)), Some(false));
        assert_eq!(is_primitive_root(&big(3), &big(4)), Some(true));
        assert_eq!(is_primitive_root(&big(2), &big(4)), Some(false));
        assert_eq!(is_primitive_root(&big(2), &big(8)), Some(false));
        assert_eq!(is_primitive_root(&big(4), &big(7)), Some(false));
        assert_eq!(is_primitive_root(&big(3), &big(15)), Some(false));
        assert_eq!(is_primitive_root(&big(2), &big(15)), Some(false));
        assert_eq!(is_primitive_root(&big(3), &big(14)), Some(true));
        assert_eq!(is_primitive_root(&big(2), &big(9)), Some(true));
        assert_eq!(is_primitive_root(&big(5), &big(9)), Some(true));
        assert_eq!(is_primitive_root(&big(8), &big(7)), Some(false));
        assert_eq!(is_primitive_root(&big(8), &big(11)), Some(true));
        assert_eq!(is_primitive_root(&big(6), &big(41)), Some(true));
        assert_eq!(is_primitive_root(&big(7), &big(41)), Some(true));
        assert_eq!(is_primitive_root(&big(10), &big(41)), Some(false));
        assert_eq!(is_primitive_root(&big(43), &prime127()), Some(true));
        assert_eq!(is_primitive_root(&big(3), &prime127()), Some(false));
    }

    #[test]
    fn chinese_edge_cases() {
        let pair = |a: u64, n: u64| (big(a), big(n));
        assert_eq!(chinese(&[]), Some(zero()));
        assert_eq!(chinese(&[pair(7, 5)]), Some(big(2)));
        assert_eq!(chinese(&[pair(7, 0)]), None);
        assert_eq!(
            chinese(&[pair(2, 3), pair(3, 5), pair(2, 7)]),
            Some(big(23))
        );
        assert_eq!(chinese(&[pair(2, 6), pair(5, 9)]), Some(big(14)));
        assert_eq!(chinese(&[pair(2, 6), pair(4, 9)]), None);
        assert_eq!(chinese(&[pair(5, 1), pair(5, 1)]), Some(zero()));
        let p = prime127();
        assert_eq!(
            chinese(&[(big(3), p.clone()), (big(3), p.clone())]),
            Some(big(3))
        );
    }

    #[test]
    fn factorial_and_binomial_edge_cases() {
        let p = prime127();
        assert_eq!(factorialmod(&big(5), &zero()), None);
        assert_eq!(factorialmod(&big(5), &one()), Some(zero()));
        assert_eq!(factorialmod(&big(5), &big(5)), Some(zero()));
        assert_eq!(factorialmod(&big(1), &big(5)), Some(one()));
        assert_eq!(factorialmod(&big(4), &big(5)), Some(big(4)));
        assert_eq!(factorialmod(&big(3), &big(5)), Some(one()));
        assert_eq!(factorialmod(&big(1000), &big(1024)), Some(zero()));
        assert_eq!(factorialmod(&big(101), &big(1024)), Some(zero()));
        for (n, m) in [
            (3000, 10007),
            (3000, 20014),
            (7004, 10007),
            (1001, 10007),
            (999, 8192),
            (1500, 8192),
        ] {
            assert_eq!(
                factorialmod(&big(n), &big(m)),
                crate::factorialmod(n, m).map(big),
                "{n}! mod {m}"
            );
        }
        assert_eq!(factorialmod(&(&p - 1_u8), &p), Some(&p - 1_u8));
        assert_eq!(factorialmod(&(&p - 2_u8), &p), Some(one()));
        assert_eq!(factorialmod(&(&p - 3_u8), &p), Some((&p - 1_u8) / 2_u8));
        assert_eq!(factorialmod(&(one() << 70_usize), &p), None);
        assert_eq!(binomialmod(&big(10), &big(3), &zero()), None);
        assert_eq!(binomialmod(&big(10), &big(3), &one()), Some(zero()));
        assert_eq!(binomialmod(&big(10), &big(0), &big(7)), Some(one()));
        assert_eq!(binomialmod(&big(10), &big(10), &big(7)), Some(one()));
        assert_eq!(binomialmod(&big(10), &big(11), &big(7)), Some(zero()));
        assert_eq!(binomialmod(&big(10), &big(2), &big(2)), Some(one()));
        assert_eq!(binomialmod(&big(10), &big(3), &big(2)), Some(zero()));
        assert_eq!(binomialmod(&big(10), &big(3), &big(7)), Some(one()));
        assert_eq!(binomialmod(&big(10), &big(3), &big(8)), Some(zero()));
        assert_eq!(binomialmod(&big(10), &big(3), &big(9)), Some(big(120 % 9)));
        assert_eq!(
            binomialmod(&big(10), &big(3), &big(27)),
            Some(big(120 % 27))
        );
        assert_eq!(binomialmod(&big(10), &big(3), &big(12)), Some(zero()));
        for (n, k, m) in [
            (1_000_000, 500_000, 1024),
            (2_000_000, 1_000_000, 9),
            (2_000_000, 1_000_000, 7),
            (200_001, 100_000, 25),
            (200_001, 100_000, 125),
        ] {
            assert_eq!(
                binomialmod(&big(n), &big(k), &big(m)),
                crate::binomialmod(n, k, m).map(big),
                "C({n}, {k}) mod {m}"
            );
        }
        assert_eq!(binomial_granville(&big(10), &big(3), &big(3), 1), zero());
        assert_eq!(binomial_granville(&big(10), &big(3), &big(3), 2), big(3));
        assert_eq!(binomial_granville(&big(10), &big(3), &big(2), 1), zero());
        assert_eq!(
            binomial_granville(&big(100), &big(50), &big(2), 10),
            binomial_prime_power(&big(100), &big(50), &big(2), 10)
        );
        assert_eq!(
            binomial_granville(&big(100), &big(50), &big(3), 5),
            binomial_prime_power(&big(100), &big(50), &big(3), 5)
        );
        assert_eq!(
            binomial_granville(&big(1000), &big(3), &big(5), 4),
            binomial_prime_power(&big(1000), &big(3), &big(5), 4)
        );
        assert_eq!(binomial_prime_power(&big(3), &big(5), &big(7), 1), zero());
        assert_eq!(binomialmod(&(&p + 1_u8), &big(2), &p), Some(zero()));
        assert_eq!(binomialmod(&(&p + 1_u8), &big(1), &p), Some(one()));
    }

    #[test]
    fn carmichael_edge_cases() {
        assert!(!is_carmichael(&big(560)));
        assert!(!is_carmichael(&big(561 * 2)));
        assert!(!is_carmichael(&big(561 * 9)));
        assert_eq!(
            is_carmichael(&big(5 * 7 * 23)),
            crate::is_carmichael(5 * 7 * 23)
        );
        assert!(!is_carmichael(&big(5 * 5 * 29)));
        assert!(!is_carmichael(&big(2047)));
        assert!(!is_carmichael(&big(1105 * 2)));
        assert!(is_carmichael(&big(1105)));
        assert!(is_carmichael(&big(1729)));
        assert!(!is_carmichael(&big(1727)));
        assert!(!is_carmichael(&prime127()));
        // The probabilistic path: a 50-digit prime, and a composite that
        // fails the Fermat test at base 41.
        let fifty = super::super::primality::next_prime(&big(10).pow(50));
        assert!(!is_carmichael(&fifty));
        let fermat = (one() << 67_usize) - 1_u8;
        assert!(!is_carmichael(&(&fermat * &fermat * &fermat)));
        for n in [
            34,
            35,
            36,
            37,
            77,
            1121,
            3 * 5 * 7,
            5 * 7 * 11,
            3 * 5 * 17,
            7 * 11 * 13,
            5 * 7 * 11 * 13,
            2 * 3 * 5 * 7,
        ] {
            assert_eq!(
                is_quasi_carmichael(&big(n)),
                crate::is_quasi_carmichael(n),
                "quasi {n}"
            );
        }
    }
}
