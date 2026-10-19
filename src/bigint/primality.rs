//! Primality tests on arbitrary precision values: the functions of
//! [`crate::primality`], [`lucas_sequence`](crate::lucas_sequence) and the
//! prime walkers of [`crate::sieve`] with the same names, for [`BigUint`].
//!
//! Port of MPU 0.75 `PP.pm`, the pure Perl path that `Math::BigInt` inputs
//! take, which follows `primality.c`. Below 2^64 the results equal the native
//! ones; `is_prime`, `is_prob_prime`, `next_prime` and `prev_prime` hand over
//! to the native path there. Above 2^64 `is_prime`, `is_prob_prime` and
//! `is_bpsw_prime` return 1 (probable prime) and never 2, as MPU does.
//!
//! The pseudoprime tests take their bases as a slice, as MPU takes a list;
//! an empty slice means base 2. A base below 2 panics, as on the native path.
//!
//! ```
//! use math_prime_util::bigint::primality::{is_prime, next_prime};
//! use num_bigint::BigUint;
//! let m = (BigUint::from(1_u8) << 521_usize) - 1_u8;
//! assert_eq!(is_prime(&m), 1);
//! assert_eq!(next_prime(&(&m - 1_u8)), m);
//! ```

use super::Int;
use super::arith::{is_square, jacobi};
use num_bigint::{BigInt, BigUint, Sign};

fn zero() -> BigUint {
    BigUint::from(0_u8)
}

fn one() -> BigUint {
    BigUint::from(1_u8)
}

fn two() -> BigUint {
    BigUint::from(2_u8)
}

/// `a mod n` in `0..n` for a signed `a`; `n` is nonzero.
fn reduce(a: &BigInt, n: &BigUint) -> BigUint {
    let r = a.magnitude() % n;
    if a.sign() == Sign::Minus && r != zero() {
        n - r
    } else {
        r
    }
}

/// `a - b mod n` for `a` and `b` below `n`.
fn submod(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    if a >= b { a - b } else { n - (b - a) }
}

/// `a * b - c mod n` for `c` below `n`.
fn mulsubmod(a: &BigUint, b: &BigUint, c: &BigUint, n: &BigUint) -> BigUint {
    submod(&((a * b) % n), c, n)
}

/// `(a + b) / 2 mod n` for odd `n` and `a`, `b` below `n`.
fn halve_sum(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    let sum = (a + b) % n;
    if sum.bit(0) { (sum + n) >> 1 } else { sum >> 1 }
}

/// Splits `m` into `(d, s)` with `m = d * 2^s` and `d` odd. `m` is nonzero.
fn split_twos(m: &BigUint) -> (BigUint, u64) {
    let s = m.trailing_zeros().unwrap_or(0);
    (m >> s, s)
}

/// One strong probable prime round for a reduced base `a` in `2..n-1`.
fn strong_round(n: &BigUint, a: &BigUint, nm1: &BigUint, d: &BigUint, s: u64) -> bool {
    let mut x = a.modpow(d, n);
    if x == one() || x == *nm1 {
        return true;
    }
    for _ in 1..s {
        x = (&x * &x) % n;
        if x == one() {
            return false;
        }
        if x == *nm1 {
            return true;
        }
    }
    false
}

/// Base-two strong probable prime test for odd `n >= 3`
/// (`PP.pm::_miller_rabin_2`).
pub(super) fn strong_base_two(n: &BigUint) -> bool {
    let nm1 = n - 1_u8;
    let (d, s) = split_twos(&nm1);
    strong_round(n, &two(), &nm1, &d, s)
}

/// Reduces the bases MPU-style: a base below 2 is an error, a base at or
/// above `n` is reduced, and a reduced base of 0, 1 or `n - 1` is skipped.
fn reduced_bases(n: &BigUint, bases: &[BigUint]) -> Vec<BigUint> {
    let nm1 = n - 1_u8;
    let mut reduced = Vec::new();
    for a in bases {
        assert!(*a >= two(), "invalid base");
        let a = a % n;
        if a > one() && a != nm1 {
            reduced.push(a);
        }
    }
    reduced
}

/// Tests the Fermat congruence `a^(n-1) = 1 mod n` for every base.
///
/// Port of MPU 0.75 `PP.pm::is_pseudoprime`. An empty slice means base 2.
///
/// # Panics
/// Panics if a base is below 2 and `n` is at least 3.
///
/// ```
/// use math_prime_util::bigint::primality::is_pseudoprime;
/// use num_bigint::BigUint;
/// assert!(is_pseudoprime(&BigUint::from(341_u16), &[BigUint::from(2_u8)]));
/// assert!(!is_pseudoprime(&BigUint::from(341_u16), &[BigUint::from(3_u8)]));
/// ```
pub fn is_pseudoprime(n: &BigUint, bases: &[BigUint]) -> bool {
    if *n < BigUint::from(3_u8) {
        return *n == two();
    }
    let nm1 = n - 1_u8;
    let default = [two()];
    let bases = if bases.is_empty() { &default } else { bases };
    bases.iter().all(|a| {
        assert!(*a >= two(), "invalid base");
        let a = a % n;
        a == one() || a.modpow(&nm1, n) == one()
    })
}

/// Tests the Euler–Jacobi congruence `a^((n-1)/2) = (a/n) mod n` for
/// every base.
///
/// Port of MPU 0.75 `PP.pm::is_euler_pseudoprime`. An empty slice means
/// base 2.
///
/// # Panics
/// Panics if a base is below 2 and `n` is an odd number at least 3.
///
/// ```
/// use math_prime_util::bigint::primality::is_euler_pseudoprime;
/// use num_bigint::BigUint;
/// assert!(is_euler_pseudoprime(&BigUint::from(561_u16), &[BigUint::from(2_u8)]));
/// assert!(!is_euler_pseudoprime(&BigUint::from(341_u16), &[BigUint::from(2_u8)]));
/// ```
pub fn is_euler_pseudoprime(n: &BigUint, bases: &[BigUint]) -> bool {
    if *n < BigUint::from(3_u8) {
        return *n == two();
    }
    if !n.bit(0) {
        return false;
    }
    let half = (n - 1_u8) >> 1;
    let default = [two()];
    let bases = if bases.is_empty() { &default } else { bases };
    bases.iter().all(|a| {
        assert!(*a >= two(), "invalid base");
        let a = a % n;
        let expected = match jacobi(&BigInt::from(a.clone()), n) {
            1 => one(),
            -1 => n - 1_u8,
            _ => return false,
        };
        a.modpow(&half, n) == expected
    })
}

/// Tests the Euler criterion for base 2 in Plumb's form.
///
/// Port of MPU 0.75 `PP.pm::is_euler_plumb_pseudoprime`.
///
/// ```
/// use math_prime_util::bigint::primality::is_euler_plumb_pseudoprime;
/// use num_bigint::BigUint;
/// assert!(is_euler_plumb_pseudoprime(&BigUint::from(101_u8)));
/// assert!(!is_euler_plumb_pseudoprime(&BigUint::from(341_u16)));
/// ```
pub fn is_euler_plumb_pseudoprime(n: &BigUint) -> bool {
    if *n < BigUint::from(4_u8) {
        return *n >= two();
    }
    if !n.bit(0) {
        return false;
    }
    let residue = n % 8_u8;
    let shift = 1 + u64::from(residue == one());
    let x = two().modpow(&((n - 1_u8) >> shift), n);
    if x == one() {
        return residue == one() || residue == BigUint::from(7_u8);
    }
    if x == n - 1_u8 {
        return residue.bit(0) && residue != BigUint::from(7_u8);
    }
    false
}

/// Tests the strong (Miller–Rabin) congruence for every base.
///
/// Port of MPU 0.75 `PP.pm::is_strong_pseudoprime`. An empty slice means
/// base 2.
///
/// # Panics
/// Panics if a base is below 2 and `n` is an odd number at least 3.
///
/// ```
/// use math_prime_util::bigint::primality::is_strong_pseudoprime;
/// use num_bigint::BigUint;
/// assert!(is_strong_pseudoprime(&BigUint::from(2047_u16), &[BigUint::from(2_u8)]));
/// assert!(!is_strong_pseudoprime(&BigUint::from(2047_u16), &[BigUint::from(3_u8)]));
/// ```
pub fn is_strong_pseudoprime(n: &BigUint, bases: &[BigUint]) -> bool {
    if *n < BigUint::from(3_u8) {
        return *n == two();
    }
    if !n.bit(0) {
        return false;
    }
    if bases.is_empty() {
        return strong_base_two(n);
    }
    let nm1 = n - 1_u8;
    let (d, s) = split_twos(&nm1);
    reduced_bases(n, bases)
        .iter()
        .all(|a| strong_round(n, a, &nm1, &d, s))
}

/// `(U_k, V_k) mod n` for reduced `p`, `q` below `n`, `k >= 1`, `n >= 2`,
/// by the generic doubling ladder of `PP.pm::lucasuvmod`.
fn lucas_uv(p: &BigUint, q: &BigUint, k: &BigUint, n: &BigUint) -> (BigUint, BigUint) {
    let (odd, s) = split_twos(k);
    let (mut uh, mut vl, mut vh, mut ql, mut qh) = (one(), two() % n, p.clone(), one(), one());
    for bit in (1..odd.bits()).rev() {
        ql = (&ql * &qh) % n;
        if odd.bit(bit) {
            qh = (&ql * q) % n;
            uh = (&uh * &vh) % n;
            vl = mulsubmod(&vh, &vl, &((p * &ql) % n), n);
            vh = mulsubmod(&vh, &vh, &((&qh + &qh) % n), n);
        } else {
            qh.clone_from(&ql);
            uh = mulsubmod(&uh, &vl, &ql, n);
            vh = mulsubmod(&vh, &vl, &((p * &ql) % n), n);
            vl = mulsubmod(&vl, &vl, &((&ql + &ql) % n), n);
        }
    }
    ql = (&ql * &qh) % n;
    qh = (&ql * q) % n;
    uh = mulsubmod(&uh, &vl, &ql, n);
    vl = mulsubmod(&vh, &vl, &((p * &ql) % n), n);
    ql = (&ql * &qh) % n;
    for _ in 0..s {
        uh = (&uh * &vl) % n;
        vl = mulsubmod(&vl, &vl, &((&ql + &ql) % n), n);
        ql = (&ql * &ql) % n;
    }
    (uh, vl)
}

/// `(U_k, V_k) mod n` for odd `n >= 3` and reduced `p`, `q`, with
/// `d = p^2 - 4q mod n`, by the halving ladder of `PP.pm::lucasuvmod`.
/// Also returns `Q^k mod n`.
fn lucas_uvq_odd(
    p: &BigUint,
    q: &BigUint,
    d: &BigUint,
    k: &BigUint,
    n: &BigUint,
) -> (BigUint, BigUint, BigUint) {
    let (mut u, mut v, mut qk) = (one(), p.clone(), q.clone());
    for bit in (0..k.bits() - 1).rev() {
        u = (&u * &v) % n;
        v = mulsubmod(&v, &v, &((&qk + &qk) % n), n);
        qk = (&qk * &qk) % n;
        if k.bit(bit) {
            let t = (&u * d) % n;
            u = halve_sum(&((&u * p) % n), &v, n);
            v = halve_sum(&((&v * p) % n), &t, n);
            qk = (&qk * q) % n;
        }
    }
    (u, v, qk)
}

/// Lucas sequences `U_k(P, Q)` and `V_k(P, Q)` mod `n`, and `Q^k mod n`.
///
/// Port of MPU 0.75 `PP.pm::lucas_sequence`. `P` and `Q` are reduced
/// mod `n` first, so the results equal the native
/// [`lucas_sequence`](crate::lucas_sequence) on the `u64` range.
///
/// # Panics
/// Panics when `n` is 0, as the native function does.
///
/// ```
/// use math_prime_util::bigint::primality::lucas_sequence;
/// use num_bigint::{BigInt, BigUint};
/// let (u, v, qk) = lucas_sequence(&BigUint::from(10_000_u16), &BigInt::from(1), &BigInt::from(-1), &BigUint::from(17_u8));
/// assert_eq!((u, v, qk), (BigUint::from(1597_u16), BigUint::from(3571_u16), BigUint::from(9999_u16)));
/// ```
pub fn lucas_sequence(
    n: &BigUint,
    p: &BigInt,
    q: &BigInt,
    k: &BigUint,
) -> (BigUint, BigUint, BigUint) {
    assert!(*n != zero(), "lucas_sequence: modulus n must be > 0");
    if *n == one() {
        return (zero(), zero(), zero());
    }
    let (p, q) = (reduce(p, n), reduce(q, n));
    if *k == zero() {
        return (zero(), two() % n, one());
    }
    let (u, v) = lucas_uv(&p, &q, k, n);
    (u, v, q.modpow(k, n))
}

/// Selfridge parameters: the first `D` in `5, -7, 9, -11, ...` with
/// `(D/n) = -1`, giving `P = 1`, `Q = (1 - D)/4`. Returns `(Q, D)` reduced
/// mod `n`, or `None` when the search finds a factor of `n`. `n` is odd,
/// at least 5 and not a square.
fn selfridge_parameters(n: &BigUint) -> Option<(BigUint, BigUint)> {
    let mut d = 5_i64;
    loop {
        let g = BigUint::from(d.unsigned_abs()).gcd(n);
        if g > one() && g != *n {
            return None;
        }
        if jacobi(&BigInt::from(d), n) == -1 {
            break;
        }
        d = if d > 0 { -d - 2 } else { -d + 2 };
        assert!(d.unsigned_abs() <= 4_000_000_000, "Lucas D exceeded 4e9");
    }
    let q = (1 - d) / 4;
    Some((reduce(&BigInt::from(q), n), reduce(&BigInt::from(d), n)))
}

/// Extra strong parameters: the first `P` in `3, 3 + increment, ...` with
/// `((P^2 - 4)/n) = -1`, for `Q = 1`. Returns `P`, or `None` when the search
/// finds a factor of `n`. `n` is odd, at least 5 and not a square.
fn extra_strong_parameters(n: &BigUint, increment: u64) -> Option<u64> {
    let mut p = 3_u64;
    loop {
        let d = p * p - 4;
        let g = BigUint::from(d).gcd(n);
        if g > one() && g != *n {
            return None;
        }
        if jacobi(&BigInt::from(d), n) == -1 {
            return Some(p);
        }
        p += increment;
        assert!(p <= 65_535, "Lucas parameter exceeded 65_535");
    }
}

/// Tests the Lucas congruence `U_{n+1} = 0 mod n` with Selfridge parameters.
///
/// Port of MPU 0.75 `PP.pm::is_lucas_pseudoprime`.
///
/// ```
/// use math_prime_util::bigint::primality::is_lucas_pseudoprime;
/// use num_bigint::BigUint;
/// assert!(is_lucas_pseudoprime(&BigUint::from(323_u16)));
/// assert!(!is_lucas_pseudoprime(&BigUint::from(341_u16)));
/// ```
pub fn is_lucas_pseudoprime(n: &BigUint) -> bool {
    if *n < BigUint::from(4_u8) {
        return *n >= two();
    }
    if !n.bit(0) || is_square(n) {
        return false;
    }
    let Some((q, d)) = selfridge_parameters(n) else {
        return false;
    };
    let (u, _, _) = lucas_uvq_odd(&one(), &q, &d, &(n + 1_u8), n);
    u == zero()
}

/// Tests the strong Lucas congruence with Selfridge parameters.
///
/// Port of MPU 0.75 `PP.pm::is_strong_lucas_pseudoprime`.
///
/// ```
/// use math_prime_util::bigint::primality::is_strong_lucas_pseudoprime;
/// use num_bigint::BigUint;
/// assert!(is_strong_lucas_pseudoprime(&BigUint::from(5459_u16)));
/// assert!(!is_strong_lucas_pseudoprime(&BigUint::from(323_u16)));
/// ```
pub fn is_strong_lucas_pseudoprime(n: &BigUint) -> bool {
    if *n < BigUint::from(4_u8) {
        return *n >= two();
    }
    if !n.bit(0) || is_square(n) {
        return false;
    }
    let Some((q, d)) = selfridge_parameters(n) else {
        return false;
    };
    let (k, s) = split_twos(&(n + 1_u8));
    let (u, mut v, mut qk) = lucas_uvq_odd(&one(), &q, &d, &k, n);
    if u == zero() {
        return true;
    }
    for r in 0..s {
        if v == zero() {
            return true;
        }
        if r + 1 < s {
            v = mulsubmod(&v, &v, &((&qk + &qk) % n), n);
            qk = (&qk * &qk) % n;
        }
    }
    false
}

/// Tests the extra strong Lucas congruence with `Q = 1` and the first
/// `P >= 3` whose discriminant has Jacobi symbol -1.
///
/// Port of MPU 0.75 `PP.pm::is_extra_strong_lucas_pseudoprime`.
///
/// ```
/// use math_prime_util::bigint::primality::is_extra_strong_lucas_pseudoprime;
/// use num_bigint::BigUint;
/// assert!(is_extra_strong_lucas_pseudoprime(&BigUint::from(989_u16)));
/// assert!(!is_extra_strong_lucas_pseudoprime(&BigUint::from(5459_u16)));
/// ```
pub fn is_extra_strong_lucas_pseudoprime(n: &BigUint) -> bool {
    if *n < BigUint::from(4_u8) {
        return *n >= two();
    }
    if !n.bit(0) || is_square(n) {
        return false;
    }
    let Some(p) = extra_strong_parameters(n, 1) else {
        return false;
    };
    let p = BigUint::from(p) % n;
    let d = mulsubmod(&p, &p, &(BigUint::from(4_u8) % n), n);
    let (k, s) = split_twos(&(n + 1_u8));
    let (u, mut v, _) = lucas_uvq_odd(&p, &one(), &d, &k, n);
    if u == zero() && (v == two() || v == n - 2_u8) {
        return true;
    }
    for _ in 1..s {
        if v == zero() {
            return true;
        }
        v = mulsubmod(&v, &v, &two(), n);
    }
    false
}

/// Tests the almost extra strong Lucas congruence: the `V` half of the
/// extra strong test, with the parameter search stepping by `increment`.
///
/// Port of MPU 0.75 `PP.pm::is_almost_extra_strong_lucas_pseudoprime`.
///
/// # Panics
/// Panics if `increment` is outside `1..=256`, or the parameter search
/// exceeds `65_535`.
///
/// ```
/// use math_prime_util::bigint::primality::is_almost_extra_strong_lucas_pseudoprime;
/// use num_bigint::BigUint;
/// assert!(is_almost_extra_strong_lucas_pseudoprime(&BigUint::from(101_u8), 1));
/// assert!(!is_almost_extra_strong_lucas_pseudoprime(&BigUint::from(2047_u16), 1));
/// ```
pub fn is_almost_extra_strong_lucas_pseudoprime(n: &BigUint, increment: u64) -> bool {
    assert!((1..=256).contains(&increment), "invalid increment");
    if *n < BigUint::from(13_u8) {
        return [2_u8, 3, 5, 7, 11].iter().any(|p| *n == BigUint::from(*p));
    }
    if !n.bit(0) || is_square(n) {
        return false;
    }
    if let Some(small) = n.to_u64() {
        if ((increment >= 16 && small <= 331) || (increment > 148 && small <= 631))
            && crate::primality::is_prob_prime(small) != 0
        {
            return true;
        }
    }
    let Some(p) = extra_strong_parameters(n, increment) else {
        return false;
    };
    let p = BigUint::from(p) % n;
    let (k, s) = split_twos(&(n + 1_u8));
    let mut v = p.clone();
    let mut w = mulsubmod(&p, &p, &two(), n);
    for bit in (0..k.bits() - 1).rev() {
        if k.bit(bit) {
            v = mulsubmod(&v, &w, &p, n);
            w = mulsubmod(&w, &w, &two(), n);
        } else {
            w = mulsubmod(&w, &v, &p, n);
            v = mulsubmod(&v, &v, &two(), n);
        }
    }
    if v == two() || v == n - 2_u8 {
        return true;
    }
    for _ in 1..s {
        if v == zero() {
            return true;
        }
        v = mulsubmod(&v, &v, &two(), n);
    }
    false
}

/// BPSW: a base-two strong test, then an extra strong Lucas test. Returns
/// 2 for a prime below 2^64, 1 for a probable prime above it, else 0.
///
/// Port of MPU 0.75 `PP.pm::is_bpsw_prime`. Below 2^64 MPU uses the almost
/// extra strong Lucas test, so the result is the native
/// [`is_bpsw_prime`](crate::is_bpsw_prime).
///
/// ```
/// use math_prime_util::bigint::primality::is_bpsw_prime;
/// use num_bigint::BigUint;
/// assert_eq!(is_bpsw_prime(&BigUint::from(101_u8)), 2);
/// assert_eq!(is_bpsw_prime(&((BigUint::from(1_u8) << 127_usize) - 1_u8)), 1);
/// ```
pub fn is_bpsw_prime(n: &BigUint) -> u8 {
    if let Some(small) = n.to_u64() {
        return crate::primality::is_bpsw_prime(small);
    }
    u8::from(n.bit(0) && strong_base_two(n) && is_extra_strong_lucas_pseudoprime(n))
}

/// The primes to 113, the trial division of `PP.pm::_is_prime7`.
const SMALL_PRIMES: [u8; 30] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113,
];

/// Tests whether `n` is prime: 2 for a proven prime (below 2^64), 1 for a
/// probable prime (BPSW, above 2^64), 0 for a composite.
///
/// Port of MPU 0.75 `PP.pm::is_prob_prime`, which is `is_prime` there:
/// trial division by the primes to 113, then [`is_bpsw_prime`]. Below
/// 2^128 the native path answers.
///
/// ```
/// use math_prime_util::bigint::primality::is_prob_prime;
/// use num_bigint::BigUint;
/// assert_eq!(is_prob_prime(&BigUint::from(97_u8)), 2);
/// assert_eq!(is_prob_prime(&((BigUint::from(1_u8) << 607_usize) - 1_u8)), 1);
/// assert_eq!(is_prob_prime(&(BigUint::from(1_u8) << 600_usize)), 0);
/// ```
pub fn is_prob_prime(n: &BigUint) -> u8 {
    if let Ok(native) = u128::try_from(n) {
        return native.is_prime();
    }
    if SMALL_PRIMES.iter().any(|p| n % *p == zero()) {
        return 0;
    }
    is_bpsw_prime(n)
}

/// Tests whether `n` is prime. The same as [`is_prob_prime`]: MPU proves
/// primality only on the native range.
///
/// Port of MPU 0.75 `PP.pm::is_prime`.
///
/// ```
/// use math_prime_util::bigint::primality::is_prime;
/// use num_bigint::BigUint;
/// assert_eq!(is_prime(&((BigUint::from(1_u8) << 521_usize) - 1_u8)), 1);
/// assert_eq!(is_prime(&((BigUint::from(1_u8) << 523_usize) - 1_u8)), 0);
/// ```
pub fn is_prime(n: &BigUint) -> u8 {
    is_prob_prime(n)
}

/// Lucas–Lehmer test of `2^p - 1`: `V = 4`, then `V = V^2 - 2` taken
/// `p - 2` times, is zero mod `2^p - 1` exactly when the number is prime.
///
/// This is the generic test at the end of MPU 0.75 `PP.pm::is_mersenne_prime`,
/// run for any `p`. Returns false for `p < 2` and for even `p > 2`.
///
/// ```
/// use math_prime_util::bigint::primality::lucas_lehmer;
/// assert!(lucas_lehmer(127));
/// assert!(!lucas_lehmer(11));
/// ```
pub fn lucas_lehmer(p: u64) -> bool {
    if p == 2 {
        return true;
    }
    if p < 2 || p % 2 == 0 {
        return false;
    }
    let m = (one() << p) - 1_u8;
    let mut v = BigUint::from(4_u8);
    for _ in 2..p {
        v = mulsubmod(&v, &v, &two(), &m);
    }
    v == zero()
}

/// Tests a Mersenne exponent: the MPU 0.75 table of known Mersenne primes,
/// false below the exponent GIMPS has verified, else the Lucas–Lehmer test.
///
/// Port of MPU 0.75 `PP.pm::is_mersenne_prime`. The native
/// [`is_mersenne_prime`](crate::is_mersenne_prime) returns `None` past the
/// table; this one runs [`lucas_lehmer`] there, after the quick rejections
/// of a composite `p` and of `p = 3 mod 4` with `2p + 1` prime, which
/// divides `2^p - 1`. That test squares a number of at least 81 million
/// bits `p` times and is not practical without a GMP backend.
///
/// ```
/// use math_prime_util::bigint::primality::is_mersenne_prime;
/// assert!(is_mersenne_prime(607));
/// assert!(!is_mersenne_prime(608));
/// ```
pub fn is_mersenne_prime(p: u64) -> bool {
    if let Some(known) = crate::primality::is_mersenne_prime(p) {
        return known;
    }
    if crate::primality::is_prob_prime(p) == 0 {
        return false;
    }
    let germain = p
        .checked_mul(2)
        .and_then(|d| d.checked_add(1))
        .is_some_and(|d| p % 4 == 3 && crate::primality::is_prob_prime(d) != 0);
    !germain && lucas_lehmer(p)
}

const ADVANCE: [u8; 30] = [
    1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1, 2,
];
const RETREAT: [u8; 30] = [
    1, 2, 1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 1, 2, 1, 2, 3, 4, 1, 2, 1, 2, 3, 4, 1, 2, 3, 4, 5, 6,
];

/// The remainder mod 30 as an index. The remainder is at most 29.
fn wheel_index(n: &BigUint) -> usize {
    usize::try_from(n % 30_u8).unwrap_or_default()
}

/// Least prime greater than `n`.
///
/// Port of MPU 0.75 `PP.pm::next_prime`: the mod-30 wheel with
/// [`is_prob_prime`] on each candidate. Below the largest `u64` prime the
/// native [`next_prime`](crate::next_prime) answers.
///
/// ```
/// use math_prime_util::bigint::primality::next_prime;
/// use num_bigint::BigUint;
/// assert_eq!(next_prime(&BigUint::from(u64::MAX)), (BigUint::from(1_u8) << 64_usize) + 13_u8);
/// ```
pub fn next_prime(n: &BigUint) -> BigUint {
    if let Some(p) = n.to_u64().and_then(crate::sieve::next_prime) {
        return BigUint::from(p);
    }
    let mut n = n.clone();
    loop {
        n += ADVANCE[wheel_index(&n)];
        if &n % 7_u8 != zero() && is_prob_prime(&n) != 0 {
            return n;
        }
    }
}

/// Greatest prime less than `n`, or `None` when `n <= 2`.
///
/// Port of MPU 0.75 `PP.pm::prev_prime`: the mod-30 wheel with
/// [`is_prob_prime`] on each candidate. Below 2^64 the native
/// [`prev_prime`](crate::prev_prime) answers.
///
/// ```
/// use math_prime_util::bigint::primality::prev_prime;
/// use num_bigint::BigUint;
/// assert_eq!(prev_prime(&(BigUint::from(1_u8) << 64_usize)), Some(BigUint::from(u64::MAX - 58)));
/// assert_eq!(prev_prime(&BigUint::from(2_u8)), None);
/// ```
pub fn prev_prime(n: &BigUint) -> Option<BigUint> {
    if let Some(small) = n.to_u64() {
        return crate::sieve::prev_prime(small).map(BigUint::from);
    }
    let mut n = n.clone();
    loop {
        n -= RETREAT[wheel_index(&n)];
        if &n % 7_u8 != zero() && is_prob_prime(&n) != 0 {
            return Some(n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big(text: &str) -> BigUint {
        text.parse().expect("decimal")
    }

    fn mersenne(bits: usize) -> BigUint {
        (one() << bits) - 1_u8
    }

    #[test]
    fn small_inputs_follow_the_native_rules() {
        let bases = vec![two(), one()];
        for n in 0_u8..3 {
            let n = BigUint::from(n);
            assert_eq!(is_strong_pseudoprime(&n, &bases), n == two());
            assert_eq!(is_pseudoprime(&n, &bases), n == two());
            assert_eq!(is_euler_pseudoprime(&n, &bases), n == two());
            assert_eq!(is_euler_plumb_pseudoprime(&n), n == two());
            assert_eq!(is_lucas_pseudoprime(&n), n == two());
            assert_eq!(is_strong_lucas_pseudoprime(&n), n == two());
            assert_eq!(is_extra_strong_lucas_pseudoprime(&n), n == two());
            assert_eq!(is_almost_extra_strong_lucas_pseudoprime(&n, 1), n == two());
        }
        let four = BigUint::from(4_u8);
        assert!(!is_strong_pseudoprime(&four, &bases));
        assert!(!is_euler_pseudoprime(&four, &bases));
        assert!(!is_euler_plumb_pseudoprime(&four));
        assert!(!is_lucas_pseudoprime(&four));
        assert!(!is_strong_lucas_pseudoprime(&four));
        assert!(!is_extra_strong_lucas_pseudoprime(&four));
        assert!(!is_almost_extra_strong_lucas_pseudoprime(&four, 1));
        assert!(!is_almost_extra_strong_lucas_pseudoprime(
            &BigUint::from(25_u8),
            1
        ));
        assert!(is_almost_extra_strong_lucas_pseudoprime(
            &BigUint::from(331_u16),
            16
        ));
        assert!(is_almost_extra_strong_lucas_pseudoprime(
            &BigUint::from(631_u16),
            149
        ));
        assert!(!is_almost_extra_strong_lucas_pseudoprime(
            &BigUint::from(333_u16),
            16
        ));
        // Fermat tests see 4 with base 2 as 2^3 = 0 mod 4: not a pseudoprime.
        assert!(!is_pseudoprime(&four, &[two()]));
        // A base that reduces to 1 passes every test.
        let n = BigUint::from(101_u8);
        assert!(is_pseudoprime(&n, &[BigUint::from(102_u8)]));
        assert!(is_strong_pseudoprime(
            &n,
            &[BigUint::from(102_u8), BigUint::from(100_u8)]
        ));
        assert!(is_euler_pseudoprime(&n, &[BigUint::from(102_u8)]));
        // A base sharing a factor with n fails the Euler test.
        assert!(!is_euler_pseudoprime(
            &BigUint::from(15_u8),
            &[BigUint::from(3_u8)]
        ));
        // Plumb: 2 is a quadratic residue mod 17 (1 mod 8) and 7 (7 mod 8).
        assert!(is_euler_plumb_pseudoprime(&BigUint::from(17_u8)));
        assert!(is_euler_plumb_pseudoprime(&BigUint::from(7_u8)));
        assert!(is_euler_plumb_pseudoprime(&BigUint::from(11_u8)));
        assert!(is_euler_plumb_pseudoprime(&BigUint::from(13_u8)));
        assert!(!is_euler_plumb_pseudoprime(&BigUint::from(15_u8)));
        assert!(!is_euler_plumb_pseudoprime(&BigUint::from(21_u8)));
    }

    #[test]
    #[should_panic(expected = "invalid base")]
    fn strong_base_one_panics() {
        is_strong_pseudoprime(&BigUint::from(101_u8), &[one()]);
    }

    #[test]
    #[should_panic(expected = "invalid base")]
    fn fermat_base_zero_panics() {
        is_pseudoprime(&BigUint::from(101_u8), &[zero()]);
    }

    #[test]
    #[should_panic(expected = "invalid base")]
    fn euler_base_one_panics() {
        is_euler_pseudoprime(&BigUint::from(101_u8), &[one()]);
    }

    #[test]
    #[should_panic(expected = "invalid increment")]
    fn increment_zero_panics() {
        is_almost_extra_strong_lucas_pseudoprime(&BigUint::from(101_u8), 0);
    }

    #[test]
    #[should_panic(expected = "modulus n must be > 0")]
    fn lucas_sequence_modulus_zero_panics() {
        lucas_sequence(&zero(), &BigInt::from(1), &BigInt::from(-1), &one());
    }

    #[test]
    fn lucas_sequence_edges() {
        let (p, q) = (BigInt::from(3), BigInt::from(-1));
        assert_eq!(
            lucas_sequence(&one(), &p, &q, &one()),
            (zero(), zero(), zero())
        );
        let n = BigUint::from(10_u8);
        assert_eq!(lucas_sequence(&n, &p, &q, &zero()), (zero(), two(), one()));
        assert_eq!(
            lucas_sequence(&two(), &p, &q, &zero()),
            (zero(), zero(), one())
        );
        assert_eq!(
            lucas_sequence(&n, &p, &q, &one()),
            (one(), BigUint::from(3_u8), BigUint::from(9_u8))
        );
        // U_2 = P, V_2 = P^2 - 2Q: 3 and 11 for (3, -1).
        assert_eq!(
            lucas_sequence(&n, &p, &q, &two()),
            (BigUint::from(3_u8), one(), one())
        );
        // An even modulus with a square discriminant: P = 2, Q = 1 gives U_k = k, V_k = 2.
        let (u, v, qk) = lucas_sequence(
            &BigUint::from(1000_u16),
            &BigInt::from(2),
            &BigInt::from(1),
            &BigUint::from(1234_u16),
        );
        assert_eq!((u, v, qk), (BigUint::from(234_u8), two(), one()));
        // Negative P reduces mod n.
        let m = mersenne(127);
        assert_eq!(
            lucas_sequence(
                &m,
                &BigInt::from(-3),
                &BigInt::from(-5),
                &BigUint::from(7_u8)
            ),
            (BigUint::from(4229_u16), &m - 22_767_u32, &m - 78_125_u32)
        );
        assert_eq!(
            lucas_sequence(&m, &p, &q, &m),
            (one(), BigUint::from(3_u8), &m - 1_u8)
        );
    }

    #[test]
    fn mersenne_primes_and_lucas_lehmer() {
        for bits in [127, 521, 607] {
            let m = mersenne(bits);
            assert_eq!(is_prime(&m), 1);
            assert_eq!(is_bpsw_prime(&m), 1);
            assert!(is_lucas_pseudoprime(&m));
            assert!(is_strong_lucas_pseudoprime(&m));
            assert!(is_extra_strong_lucas_pseudoprime(&m));
            assert!(is_almost_extra_strong_lucas_pseudoprime(&m, 3));
            assert_eq!(is_prime(&(&m * &m)), 0);
            assert!(!is_strong_lucas_pseudoprime(&(&m * &m)));
        }
        for p in [
            2_u64, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521, 607, 1279,
        ] {
            assert!(lucas_lehmer(p), "{p}");
            assert!(is_mersenne_prime(p));
        }
        for p in [0_u64, 1, 4, 6, 9, 11, 15, 23, 29, 37, 67, 257, 1023] {
            assert!(!lucas_lehmer(p), "{p}");
            assert!(!is_mersenne_prime(p));
        }
        assert!(is_mersenne_prime(136_279_841));
        assert!(!is_mersenne_prime(81_307_408));
        // Above the verified bound: composite exponents and Sophie Germain
        // exponents (2p + 1 divides 2^p - 1) are rejected without the test.
        assert!(!is_mersenne_prime(100_000_000));
        assert!(!is_mersenne_prime(81_307_763));
    }

    #[test]
    fn above_the_native_range() {
        let p = big("170141183460469231731687303715884105757"); // 2^127 + 29
        assert_eq!(is_prob_prime(&p), 1);
        assert_eq!(is_bpsw_prime(&(one() << 130_usize)), 0);
        let q = mersenne(521);
        assert_eq!(is_prob_prime(&(&q * 113_u8)), 0);
        assert_eq!(is_prob_prime(&(&q * &p)), 0);
        assert_eq!(is_bpsw_prime(&(&q * &p)), 0);
        // 2^67 - 1: a strong pseudoprime base 2 that the Lucas tests reject.
        let m67 = mersenne(67);
        assert!(is_strong_pseudoprime(&m67, &[]));
        assert!(is_pseudoprime(&m67, &[]));
        assert!(is_euler_pseudoprime(&m67, &[]));
        assert!(!is_strong_pseudoprime(&m67, &[BigUint::from(3_u8)]));
        assert!(!is_lucas_pseudoprime(&m67));
        assert!(!is_strong_lucas_pseudoprime(&m67));
        assert!(!is_extra_strong_lucas_pseudoprime(&m67));
        assert!(!is_almost_extra_strong_lucas_pseudoprime(&m67, 1));
        assert_eq!(is_bpsw_prime(&m67), 0);
        // (4^p - 1)/3 is a Fermat pseudoprime base 2 but 5 mod 8, so not an Euler one.
        let cipolla = ((one() << 254_usize) - 1_u8) / 3_u8;
        assert!(is_pseudoprime(&cipolla, &[]));
        assert!(!is_euler_pseudoprime(&cipolla, &[]));
        assert!(!is_strong_pseudoprime(&cipolla, &[]));
        // A factor found during parameter selection: 3 divides D = 9 and 5 divides D = 5.
        let by_three = &q * 3_u8;
        assert!(!is_lucas_pseudoprime(&by_three));
        assert!(!is_strong_lucas_pseudoprime(&(&q * 5_u8)));
        assert!(!is_extra_strong_lucas_pseudoprime(&(&q * 5_u8)));
        assert!(!is_almost_extra_strong_lucas_pseudoprime(&(&q * 5_u8), 1));
        assert!(!is_extra_strong_lucas_pseudoprime(&(&q * &q)));
        assert_eq!(next_prime(&(&p - 1_u8)), p);
        assert_eq!(prev_prime(&(&p + 1_u8)), Some(p.clone()));
        assert_eq!(
            next_prime(&BigUint::from(u64::MAX - 59)),
            BigUint::from(u64::MAX - 58)
        );
        assert_eq!(
            next_prime(&BigUint::from(u64::MAX - 58)),
            (one() << 64_usize) + 13_u8
        );
        assert_eq!(
            prev_prime(&((one() << 64_usize) + 13_u8)),
            Some(BigUint::from(u64::MAX - 58))
        );
        assert_eq!(prev_prime(&zero()), None);
        assert_eq!(next_prime(&zero()), two());
    }
}
