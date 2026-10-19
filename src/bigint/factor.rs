//! Integer factoring on arbitrary precision values: the functions of
//! [`factor`](mod@crate::factor) with the same names, for [`BigUint`].
//!
//! Port of MPU 0.75 `PP.pm` (`factor`, `trial_factor`, `prho_factor`,
//! `pbrent_factor`, `pminus1_factor`, `holf_factor`) and
//! `Math::Prime::Util::ECM` (`ecm_factor`), the pure Perl path that
//! `Math::BigInt` inputs take. `pplus1_factor` and `squfof_factor` are
//! pure Perl stubs (Pollard-Brent) in MPU; here they port `factor.c`'s
//! `pplus1_factor` and `squfof_factor`. Below 2^128 [`factor`] hands over
//! to the native path, as PP hands small cofactors to XS.
//!
//! [`factor`] runs the PP recipe (perfect powers, p-1, ECM, HOLF, Brent) with
//! no time limit; it returns the first cofactor the recipe could not split
//! as the error. [`factor_within`] stops at a deadline and returns what was
//! found. The single-method functions take the method's parameters as the
//! native ones do, pull out 2, 3 and 5 first and return the two parts of one
//! split (either part can be composite), or the input when the method fails.
//!
//! ```
//! use math_prime_util::bigint::factor::{factor, factor_exp};
//! use num_bigint::BigUint;
//! let m = (BigUint::from(1_u8) << 127_usize) - 1_u8;
//! let n = &m * &m * 6_u8;
//! assert_eq!(factor(&n), Ok(vec![BigUint::from(2_u8), BigUint::from(3_u8), m.clone(), m.clone()]));
//! assert_eq!(factor_exp(&n).map(|v| v.len()), Ok(3));
//! ```
use super::arith::{gcd, invmod, is_power_exp, is_square, rootint, sqrtint};
use super::primality::is_prob_prime;
use num_bigint::BigUint;
use std::time::{Duration, Instant};

fn zero() -> BigUint {
    BigUint::from(0_u8)
}

fn one() -> BigUint {
    BigUint::from(1_u8)
}

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

fn is_one(n: &BigUint) -> bool {
    *n == one()
}

/// `a - b mod n` for reduced operands.
fn submod(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    if a >= b { a - b } else { n - (b - a) }
}

fn difference(a: &BigUint, b: &BigUint) -> BigUint {
    if a >= b { a - b } else { b - a }
}

fn checked_sub(a: &BigUint, b: &BigUint) -> Option<BigUint> {
    (a >= b).then(|| a - b)
}

/// `x * x + a mod n`, the Pollard iteration (`PP.pm::Mmuladdmod`).
fn square_add(x: &BigUint, a: &BigUint, n: &BigUint) -> BigUint {
    (x * x + a) % n
}

/// Calls `f` on every prime in `lo..=hi`, in sieve chunks; `f` returns
/// `false` to stop early.
fn for_primes(lo: u64, hi: u64, mut f: impl FnMut(u64) -> bool) {
    const CHUNK: u64 = 500_000;
    let mut start = lo;
    while start <= hi {
        let end = start.saturating_add(CHUNK - 1).min(hi);
        for p in crate::sieve::primes(start, end) {
            if !f(p) {
                return;
            }
        }
        if end == u64::MAX {
            return;
        }
        start = end + 1;
    }
}

/// The largest power of `p` at most `bound`, as the stage 1 exponents of
/// the p-1, p+1 and ECM methods use it; `p` itself when `p` is above the
/// square root of the bound.
fn prime_power(p: u64, bound: u64) -> u64 {
    let mut k = p;
    if p <= bound.isqrt() {
        while k <= bound / p {
            k *= p;
        }
    }
    k
}

/// Flags of the primes up to `limit`, index by value.
fn prime_flags(limit: u64) -> Vec<bool> {
    let size = usize::try_from(limit)
        .unwrap_or(usize::MAX)
        .saturating_add(1);
    let mut flags = vec![true; size];
    flags[0] = false;
    if size > 1 {
        flags[1] = false;
    }
    let mut i = 2;
    while i * i < size {
        if flags[i] {
            for j in (i * i..size).step_by(i) {
                flags[j] = false;
            }
        }
        i += 1;
    }
    flags
}

/// A point in time after which the factoring loops give up.
#[derive(Clone, Copy)]
struct Deadline(Option<Instant>);

impl Deadline {
    const NONE: Self = Self(None);

    fn after(limit: Duration) -> Self {
        Self(Instant::now().checked_add(limit))
    }

    fn expired(self) -> bool {
        self.0.is_some_and(|t| Instant::now() >= t)
    }
}

/// What [`factor_within`] found before its deadline.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Factored {
    /// The prime factors found, with multiplicity, in ascending order.
    pub primes: Vec<BigUint>,
    /// The composite cofactors the recipe did not split, in ascending order.
    pub unsplit: Vec<BigUint>,
}

/// `PP.pm::_found_factor`: the two parts of a split, smaller first, or `n`
/// alone when `f` is trivial.
fn found(f: Option<BigUint>, n: &BigUint) -> Vec<BigUint> {
    match f {
        Some(f) if !is_one(&f) && f != *n && f.bits() != 0 => {
            let other = n / &f;
            if f > other {
                vec![other, f]
            } else {
                vec![f, other]
            }
        }
        _ => vec![n.clone()],
    }
}

/// `PP.pm::_basic_factor`: pulls out 2, 3 and 5.
fn basic(n: &mut BigUint) -> Vec<BigUint> {
    let mut factors = Vec::new();
    for p in [2_u64, 3, 5] {
        while (&*n % p).bits() == 0 {
            factors.push(big(p));
            *n /= p;
        }
    }
    factors
}

/// The wrapper every single-method function shares: small inputs, 2, 3, 5,
/// a primality test, then one split attempt.
fn method(n: &BigUint, split: impl FnOnce(&BigUint) -> Option<BigUint>) -> Vec<BigUint> {
    if *n < big(4) {
        return if is_one(n) {
            Vec::new()
        } else {
            vec![n.clone()]
        };
    }
    let mut m = n.clone();
    let mut factors = basic(&mut m);
    if is_one(&m) {
        return factors;
    }
    if is_prob_prime(&m) != 0 {
        factors.push(m);
        return factors;
    }
    factors.extend(found(split(&m), &m));
    factors
}

/// Trial factors and the remaining cofactor, the bound inclusive.
///
/// Port of MPU 0.75 `PP.pm::trial_factor`. `None` or a bound below 5
/// divides by every prime up to the square root of the remaining cofactor,
/// which on a large prime cofactor is unbounded work, as in PP: give a
/// bound. The last value can be composite. One returns an empty list; zero
/// returns `[0]`.
///
/// ```
/// use math_prime_util::bigint::factor::trial_factor;
/// use num_bigint::BigUint;
/// let m = (BigUint::from(1_u8) << 89_usize) - 1_u8;
/// let got = trial_factor(&(&m * 175_u8), Some(5));
/// assert_eq!(got, vec![BigUint::from(5_u8), BigUint::from(5_u8), &m * 7_u8]);
/// assert_eq!(trial_factor(&(&m * 175_u8), Some(7)), vec![BigUint::from(5_u8), BigUint::from(5_u8), BigUint::from(7_u8), m]);
/// ```
pub fn trial_factor(n: &BigUint, limit: Option<u64>) -> Vec<BigUint> {
    if *n < big(4) {
        return if is_one(n) {
            Vec::new()
        } else {
            vec![n.clone()]
        };
    }
    let mut factors = Vec::new();
    let mut m = n.clone();
    let twos = m.trailing_zeros().unwrap_or(0);
    if twos > 0 {
        factors.extend(std::iter::repeat_n(
            big(2),
            usize::try_from(twos).unwrap_or(usize::MAX),
        ));
        m >>= twos;
    }
    let root = |m: &BigUint| u64::try_from(sqrtint(m)).unwrap_or(u64::MAX);
    let mut limit = match limit {
        Some(l) if l >= 5 => l.min(root(&m)),
        _ => root(&m),
    };
    for_primes(3, limit, |p| {
        if p > limit {
            return false;
        }
        if (&m % p).bits() == 0 {
            loop {
                factors.push(big(p));
                m /= p;
                if (&m % p).bits() != 0 {
                    break;
                }
            }
            limit = limit.min(root(&m));
        }
        true
    });
    if !is_one(&m) {
        factors.push(m);
    }
    factors
}

/// Splits `n` with Pollard rho, both parts of the split or `n` on failure.
///
/// Port of MPU 0.75 `PP.pm::prho_factor` (bigint path): the iteration
/// `x^2 + 3`, seeds 7, batches of 32 differences per gcd. `None` uses
/// 4,194,304 rounds. Factors need not be prime. Zero returns `[0]`; one
/// returns `[]`.
///
/// ```
/// use math_prime_util::bigint::factor::prho_factor;
/// use num_bigint::BigUint;
/// let got = prho_factor(&BigUint::from(403_u16), None);
/// assert_eq!(got, vec![BigUint::from(13_u8), BigUint::from(31_u8)]);
/// ```
pub fn prho_factor(n: &BigUint, rounds: Option<u64>) -> Vec<BigUint> {
    method(n, |m| rho(m, rounds.unwrap_or(4 * 1024 * 1024)))
}

fn rho(n: &BigUint, rounds: u64) -> Option<BigUint> {
    const INNER: u64 = 32;
    let a = BigUint::from(3_u8);
    let (mut u, mut v) = (big(7), big(7));
    let mut rounds = rounds.div_ceil(INNER);
    while rounds > 0 {
        rounds -= 1;
        let mut m = one();
        let (old_u, old_v) = (u.clone(), v.clone());
        for _ in 0..INNER {
            u = square_add(&u, &a, n);
            v = square_add(&square_add(&v, &a, n), &a, n);
            m = (m * difference(&u, &v)) % n;
        }
        let mut f = gcd(&m, n);
        if is_one(&f) {
            continue;
        }
        if f == *n {
            u = old_u;
            v = old_v;
            for _ in 0..INNER {
                u = square_add(&u, &a, n);
                v = square_add(&square_add(&v, &a, n), &a, n);
                f = gcd(&difference(&u, &v), n);
                if !is_one(&f) {
                    break;
                }
            }
            if is_one(&f) || f == *n {
                break;
            }
        }
        return Some(f);
    }
    None
}

/// Splits `n` with Brent's variant of Pollard rho.
///
/// Port of MPU 0.75 `PP.pm::pbrent_factor` (bigint path): the iteration
/// `x^2 + a` from 2, batches of 64 differences per gcd, six restarts.
/// `None` uses 4,194,304 rounds and `a = 1`. Factors need not be prime.
/// Zero returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::bigint::factor::pbrent_factor;
/// use num_bigint::BigUint;
/// let n = BigUint::from(1_000_003_u32) * BigUint::from(10_000_019_u32);
/// let got = pbrent_factor(&n, None, None);
/// assert_eq!(got, vec![BigUint::from(1_000_003_u32), BigUint::from(10_000_019_u32)]);
/// ```
pub fn pbrent_factor(n: &BigUint, rounds: Option<u64>, a: Option<u64>) -> Vec<BigUint> {
    method(n, |m| {
        brent(
            m,
            rounds.unwrap_or(4 * 1024 * 1024),
            a.unwrap_or(1),
            Deadline::NONE,
        )
    })
}

/// `PP.pm::_factor_pbrent`, the bigint path.
fn brent(n: &BigUint, mut rounds: u64, a: u64, deadline: Deadline) -> Option<BigUint> {
    const INNER: u64 = 64;
    let mut a = big(a) % n;
    let (mut x, mut anchor) = (BigUint::from(2_u8), BigUint::from(2_u8));
    let (mut r, mut failures) = (1_u64, 6_u8);
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = x.clone();
        let mut f = one();
        while left > 0 {
            let batch = left.min(INNER);
            left -= batch;
            rounds -= batch;
            saved.clone_from(&x);
            let mut product = one();
            for _ in 0..batch {
                x = square_add(&x, &a, n);
                product = (product * difference(&anchor, &x)) % n;
            }
            f = gcd(&product, n);
            if !is_one(&f) {
                break;
            }
            if deadline.expired() {
                return None;
            }
        }
        if is_one(&f) {
            r = r.saturating_mul(2);
            anchor.clone_from(&x);
            continue;
        }
        if f == *n {
            x = saved;
            loop {
                x = square_add(&x, &a, n);
                f = gcd(&submod(&x, &anchor, n), n);
                if !is_one(&f) || r == 0 {
                    break;
                }
                r -= 1;
            }
            if f == *n {
                if failures == 0 {
                    break;
                }
                failures -= 1;
                anchor = (anchor + 11_u8) % n;
                x.clone_from(&anchor);
                a += 1_u8;
                continue;
            }
        }
        return Some(f);
    }
    None
}

/// Splits `n` with Pollard's p-1 method, stage 1 to `b1` and stage 2 to `b2`.
///
/// Port of MPU 0.75 `PP.pm::pminus1_factor` (bigint path). With no first
/// bound the bounds ramp from 1000 to 10^9 with no stage 2, as PP does;
/// a first bound alone sets the second to ten times it; a first bound below
/// 7 fails. Factors need not be prime. Zero returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::bigint::factor::pminus1_factor;
/// use num_bigint::BigUint;
/// let got = pminus1_factor(&BigUint::from(403_u16), Some(20), Some(200));
/// assert_eq!(got, vec![BigUint::from(13_u8), BigUint::from(31_u8)]);
/// ```
pub fn pminus1_factor(n: &BigUint, b1: Option<u64>, b2: Option<u64>) -> Vec<BigUint> {
    method(n, |m| match b1 {
        Some(b1) => pminus1(m, b1, b2.unwrap_or(b1.saturating_mul(10)), Deadline::NONE),
        None => [1_u64, 100, 1000, 10_000, 100_000, 1_000_000]
            .into_iter()
            .find_map(|mul| pminus1(m, 1000 * mul, 1000 * mul, Deadline::NONE)),
    })
}

/// `PP.pm::_factor_pminus1`, the bigint path: batched stage 1 with a
/// restart from the last good state, then a stage 2 over the prime gaps.
#[allow(clippy::too_many_lines)] // Mirrors the Perl.
fn pminus1(n: &BigUint, b1: u64, b2: u64, deadline: Deadline) -> Option<BigUint> {
    if b1 < 7 {
        return None;
    }
    let nm1 = |a: &BigUint| gcd(&(a - 1_u8), n);
    let mut a = BigUint::from(2_u8);
    let (mut saved_a, mut saved_q) = (a.clone(), 1_u64);
    let mut powers = one();
    let mut f = one();
    let mut last = 2_u64;
    let mut j = 32_u64;
    let mut stop = false;
    let mut expired = false;
    for_primes(2, b1, |q| {
        last = q;
        powers *= prime_power(q, b1);
        j += 1;
        if (j - 1) % 64 == 0 {
            a = a.modpow(&powers, n);
            powers = one();
            if a.bits() == 0 {
                stop = true;
                return false;
            }
            f = nm1(&a);
            if !is_one(&f) {
                return false;
            }
            saved_q = q;
            saved_a.clone_from(&a);
            if deadline.expired() {
                expired = true;
                return false;
            }
        }
        true
    });
    if stop || expired {
        return None;
    }
    if is_one(&f) && !is_one(&powers) {
        a = a.modpow(&powers, n);
        if a.bits() == 0 {
            return None;
        }
        f = nm1(&a);
    }
    if f == *n {
        // Backtrack one prime at a time from the last good state.
        a = saved_a;
        f = one();
        let mut q = crate::next_prime(saved_q);
        while let Some(p) = q.filter(|&p| p <= b1) {
            a = a.modpow(&big(prime_power(p, b1)), n);
            f = nm1(&a);
            if f == *n {
                return None;
            }
            if !is_one(&f) {
                break;
            }
            q = crate::next_prime(p);
        }
    }
    if is_one(&f) && b2 > b1 {
        // Stage 2: a^q for each prime q in (b1, b2], by the gap from the
        // previous prime, with the powers of a^2 for the common gaps cached.
        let bm = a.clone();
        let bm2 = (&bm * &bm) % n;
        let mut table: Vec<Option<BigUint>> = vec![None; 20];
        table[0] = Some(bm2.clone());
        for i in 1..20 {
            let prev = table[i - 1].clone()?;
            table[i] = Some((prev * &bm2) % n);
        }
        let mut q = last;
        a = a.modpow(&big(q), n);
        let mut batch = Vec::new();
        let mut j = 1_u64;
        let mut stop = false;
        for_primes(q + 1, b2, |p| {
            let diff = p - q;
            q = p;
            let slot = usize::try_from(diff / 2)
                .unwrap_or(usize::MAX)
                .saturating_sub(1);
            if slot >= table.len() {
                table.resize(slot + 1, None);
            }
            if table[slot].is_none() {
                table[slot] = Some(bm.modpow(&big(diff), n));
            }
            if let Some(step) = &table[slot] {
                a = (&a * step) % n;
            }
            if a.bits() == 0 {
                stop = true;
                return false;
            }
            batch.push(&a - 1_u8);
            if j % 128 == 0 {
                f = gcd(&batch.iter().fold(one(), |acc, v| (acc * v) % n), n);
                batch.clear();
                if !is_one(&f) || deadline.expired() {
                    return false;
                }
            }
            j += 1;
            true
        });
        if stop {
            return None;
        }
        if !batch.is_empty() && is_one(&f) {
            f = gcd(&batch.iter().fold(one(), |acc, v| (acc * v) % n), n);
        }
    }
    (!is_one(&f) && f != *n).then_some(f)
}

/// Splits `n` with Williams' p+1 method, stage 1 to `bound`.
///
/// Port of MPU 0.75 `factor.c::pplus1_factor`: Lucas sequences from the
/// seeds 7 and 11. MPU's pure Perl `pplus1_factor` is a Pollard-Brent stub;
/// this is the C method on arbitrary precision values. `None` uses bound
/// 200. Factors need not be prime. Zero returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::bigint::factor::pplus1_factor;
/// use num_bigint::BigUint;
/// let got = pplus1_factor(&BigUint::from(403_u16), Some(20));
/// assert_eq!(got, vec![BigUint::from(13_u8), BigUint::from(31_u8)]);
/// ```
pub fn pplus1_factor(n: &BigUint, bound: Option<u64>) -> Vec<BigUint> {
    method(n, |m| pplus1(m, bound.unwrap_or(200)))
}

/// `V_k(x0, 1) mod n` by the Lucas chain of `factor.c::pplus1_factor`.
fn lucas_power(x0: &BigUint, exp: u64, n: &BigUint) -> BigUint {
    let two = BigUint::from(2_u8);
    let mut x = x0.clone();
    if exp < 2 {
        return x;
    }
    let mut y = submod(&((&x * &x) % n), &two, n);
    let mut bit = 1_u64 << (exp.ilog2() - 1);
    while bit != 0 {
        let t = submod(&((&x * &y) % n), x0, n);
        if exp & bit != 0 {
            x = t;
            y = submod(&((&y * &y) % n), &two, n);
        } else {
            y = t;
            x = submod(&((&x * &x) % n), &two, n);
        }
        bit >>= 1;
    }
    x
}

fn pplus1(n: &BigUint, bound: u64) -> Option<BigUint> {
    let two = BigUint::from(2_u8);
    let (mut x1, mut x2) = (big(7) % n, big(11) % n);
    let mut f = one();
    for_primes(2, bound, |p| {
        let k = prime_power(p, bound);
        for x in [&mut x1, &mut x2] {
            *x = lucas_power(x, k, n);
            if *x != two {
                f = gcd(&submod(x, &two, n), n);
                if !is_one(&f) && f != *n {
                    return false;
                }
            }
        }
        true
    });
    (!is_one(&f) && f != *n).then_some(f)
}

/// Splits `n` with Hart's one line factoring method.
///
/// Port of MPU 0.75 `PP.pm::holf_factor` (bigint path): `i` runs from
/// `start` for `rounds` values of `n * i`. `None` uses 67,108,864 rounds
/// from 1. Factors need not be prime. Zero returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::bigint::factor::holf_factor;
/// use num_bigint::BigUint;
/// let got = holf_factor(&BigUint::from(91_u8), None, None);
/// assert_eq!(got, vec![BigUint::from(7_u8), BigUint::from(13_u8)]);
/// ```
pub fn holf_factor(n: &BigUint, rounds: Option<u64>, start: Option<u64>) -> Vec<BigUint> {
    method(n, |m| {
        holf(
            m,
            rounds.unwrap_or(64 * 1024 * 1024),
            start.unwrap_or(1).max(1),
            Deadline::NONE,
        )
    })
}

/// `PP.pm::_factor_holf`, the bigint path. Stops at the first square found,
/// as PP does, even when its gcd is trivial.
fn holf(n: &BigUint, rounds: u64, start: u64, deadline: Deadline) -> Option<BigUint> {
    if rounds == 0 {
        return None;
    }
    let end = start.saturating_add(rounds.saturating_sub(1));
    let mut ni = n * start;
    for i in start..=end {
        if i % 1024 == 0 && deadline.expired() {
            return None;
        }
        let s = sqrtint(&ni);
        if &s * &s == ni {
            let f = gcd(&ni, n);
            return (!is_one(&f) && f != *n).then_some(f);
        }
        let s = s + 1_u8;
        let m = &s * &s - &ni;
        if is_square(&m) {
            let r = sqrtint(&m);
            let f = gcd(n, &difference(&s, &r));
            return (!is_one(&f) && f != *n).then_some(f);
        }
        if i < end {
            ni += n;
        }
    }
    None
}

/// Multiplier table of Gower and Wagstaff (`factor.c::squfof_factor`).
const SQUFOF_MULTIPLIERS: [u64; 38] = [
    3 * 5 * 7 * 11,
    3 * 5 * 7,
    3 * 5 * 7 * 11 * 13,
    3 * 5 * 7 * 13,
    3 * 5 * 7 * 11 * 17,
    3 * 5 * 11,
    3 * 5 * 7 * 17,
    3 * 5,
    3 * 5 * 7 * 11 * 19,
    3 * 5 * 11 * 13,
    3 * 5 * 7 * 19,
    3 * 5 * 7 * 13 * 17,
    3 * 5 * 13,
    3 * 7 * 11,
    3 * 7,
    5 * 7 * 11,
    3 * 7 * 13,
    5 * 7,
    3 * 5 * 17,
    5 * 7 * 13,
    3 * 5 * 19,
    3 * 11,
    3 * 7 * 17,
    3,
    3 * 11 * 13,
    5 * 11,
    3 * 7 * 19,
    3 * 13,
    5,
    5 * 11 * 13,
    5 * 7 * 19,
    5 * 13,
    7 * 11,
    7,
    3 * 17,
    7 * 13,
    11,
    1,
];

/// One multiplier's state in the SQUFOF race (`factor.c::mult_t`).
struct Squfof {
    valid: bool,
    nn: BigUint,
    p: BigUint,
    bn: BigUint,
    qn: BigUint,
    q0: BigUint,
    b0: BigUint,
    it: u64,
    imax: u64,
}

impl Squfof {
    /// The principal cycle of forms for `nn`, or `None` when `nn` is a square.
    fn start(nn: BigUint, imax: u64) -> Option<Self> {
        let root = sqrtint(&nn);
        let qn = &nn - &root * &root;
        if qn.bits() == 0 {
            return None;
        }
        let bn = (&root * 2_u8) / &qn;
        Some(Self {
            valid: true,
            nn,
            p: root.clone(),
            bn,
            qn,
            q0: one(),
            b0: root,
            it: 0,
            imax,
        })
    }

    fn step(&mut self) {
        let old_p = self.p.clone();
        self.p = &self.bn * &self.qn - &self.p;
        let old_qn = self.qn.clone();
        self.qn = if old_p >= self.p {
            &self.q0 + &self.bn * (&old_p - &self.p)
        } else {
            &self.q0 - &self.bn * (&self.p - &old_p)
        };
        self.q0 = old_qn;
        self.bn = (&self.b0 + &self.p) / &self.qn;
        self.it += 1;
    }

    /// `factor.c::squfof_unit`: a factor of `nn` found within `imax` more
    /// steps, or `None`.
    fn unit(&mut self) -> Option<BigUint> {
        let limit = self.it + self.imax;
        loop {
            if self.it & 1 != 0 {
                self.step();
            }
            let root = loop {
                if self.it >= limit {
                    return None;
                }
                self.step();
                if is_square(&self.qn) {
                    break sqrtint(&self.qn);
                }
                self.step();
            };
            let mut s = root;
            let mut ro = &self.p + &s * ((&self.b0 - &self.p) / &s);
            let Some(rest) = checked_sub(&self.nn, &(&ro * &ro)) else {
                self.valid = false;
                return None;
            };
            let mut so = rest / &s;
            if so.bits() == 0 {
                self.valid = false;
                return None;
            }
            let mut bbn = (&self.b0 + &ro) / &so;
            let mut count = 0_u32;
            loop {
                let old_ro = ro.clone();
                let Some(next) = checked_sub(&(&bbn * &so), &ro) else {
                    self.valid = false;
                    return None;
                };
                ro = next;
                let old_so = so.clone();
                so = if old_ro >= ro {
                    &s + &bbn * (&old_ro - &ro)
                } else {
                    let Some(v) = checked_sub(&s, &(&bbn * (&ro - &old_ro))) else {
                        self.valid = false;
                        return None;
                    };
                    v
                };
                s = old_so;
                if so.bits() == 0 {
                    self.valid = false;
                    return None;
                }
                bbn = (&self.b0 + &ro) / &so;
                if ro == old_ro {
                    break;
                }
                count += 1;
                if count == 8_000_008 {
                    self.valid = false;
                    return None;
                }
            }
            let f = gcd(&ro, &self.nn);
            if !is_one(&f) {
                return Some(f);
            }
        }
    }
}

/// Splits `n` with Shanks' square forms factorization and racing multipliers.
///
/// Port of MPU 0.75 `factor.c::squfof_factor` on arbitrary precision values
/// (MPU's pure Perl `squfof_factor` is a Pollard-Brent stub). The work is
/// about the fourth root of `n` steps, so the method suits inputs to about
/// 90 bits. `None` uses 4,000,000 rounds. Factors need not be prime. Zero
/// returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::bigint::factor::squfof_factor;
/// use num_bigint::BigUint;
/// let got = squfof_factor(&BigUint::from(91_u8), None);
/// assert_eq!(got, vec![BigUint::from(7_u8), BigUint::from(13_u8)]);
/// ```
pub fn squfof_factor(n: &BigUint, rounds: Option<u64>) -> Vec<BigUint> {
    method(n, |m| squfof(m, rounds.unwrap_or(4_000_000)))
}

fn squfof(n: &BigUint, rounds: u64) -> Option<BigUint> {
    let mut states: Vec<Option<Squfof>> = Vec::new();
    for mult in SQUFOF_MULTIPLIERS {
        let nn = n * mult;
        let root = sqrtint(&nn);
        let imax = u64::try_from(sqrtint(&root) / 16_u8)
            .unwrap_or(u64::MAX)
            .clamp(20, rounds.max(20));
        let state = Squfof::start(nn, imax);
        if state.is_none() {
            let f = gcd(&root, n);
            if !is_one(&f) && f != *n {
                return Some(f);
            }
        }
        states.push(state);
    }
    let mut racing = states.iter().filter(|s| s.is_some()).count();
    let mut done = 0_u64;
    while racing > 0 && done < rounds {
        for (slot, mult) in states.iter_mut().zip(SQUFOF_MULTIPLIERS) {
            if done >= rounds {
                break;
            }
            let Some(state) = slot.as_mut() else {
                continue;
            };
            if racing == 1 {
                state.imax = rounds - done;
            }
            if let Some(f) = state.unit() {
                let reduced = &f / gcd(&f, &big(mult));
                let factor = gcd(&reduced, n);
                if !is_one(&factor) && factor != *n {
                    return Some(factor);
                }
                state.valid = false;
            }
            done += state.imax;
            if !state.valid {
                *slot = None;
                racing -= 1;
            }
        }
    }
    None
}

/// Suyama sigma values of the pure Perl ECM (`Math::Prime::Util::ECM`).
const ECM_SIGMAS: [u16; 300] = [
    11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 103, 127, 139,
    149, 151, 157, 163, 181, 191, 197, 199, 211, 223, 227, 233, 239, 257, 271, 293, 313, 331, 337,
    347, 359, 379, 389, 397, 401, 409, 421, 443, 449, 457, 479, 487, 509, 521, 523, 547, 557, 587,
    641, 653, 659, 673, 677, 683, 691, 719, 727, 739, 751, 769, 797, 809, 853, 919, 929, 941, 997,
    1049, 1051, 1063, 1091, 1093, 1109, 1117, 1129, 1153, 1201, 1217, 1229, 1283, 1327, 1361, 1381,
    1427, 1447, 1459, 1471, 1481, 1489, 1543, 1549, 1571, 1621, 1709, 1723, 1753, 1759, 1801, 1811,
    1867, 1987, 2039, 2099, 2113, 2131, 2251, 2309, 2347, 2381, 2399, 2447, 2473, 2551, 2557, 2663,
    2677, 2689, 2713, 2719, 2749, 2857, 2879, 2887, 2939, 3001, 3061, 3067, 3121, 3137, 3187, 3251,
    3259, 3271, 3307, 3359, 3371, 3373, 3467, 3593, 3607, 3623, 3643, 3709, 3733, 3793, 3851, 3923,
    3989, 4019, 4049, 4129, 4231, 4253, 4283, 4339, 4349, 4441, 4523, 4649, 4787, 4987, 4999, 5171,
    5237, 5273, 5297, 5333, 5387, 5471, 5479, 5647, 5749, 5791, 6101, 6163, 6257, 6299, 6337, 6451,
    6491, 6659, 6793, 6823, 6967, 7013, 7229, 7253, 7333, 7369, 7477, 7621, 7793, 7817, 8059, 8167,
    8209, 8263, 8311, 8377, 8573, 8641, 8741, 8837, 8863, 8963, 9001, 9151, 9203, 9433, 9697, 9743,
    9781, 9883, 10007, 10069, 10099, 10139, 10163, 10193, 10267, 10429, 10457, 10487, 10691, 10837,
    10949, 11087, 11243, 11321, 11411, 11681, 11813, 11903, 12011, 12263, 12277, 12401, 12409,
    12437, 12479, 12569, 12619, 12739, 12911, 13331, 13367, 13537, 13721, 13789, 13841, 13873,
    14051, 14149, 14221, 14419, 14431, 14827, 14887, 15077, 15289, 15467, 15511, 15649, 15773,
    15797, 15859, 15901, 16057, 16141, 16217, 16529, 16547, 16553, 16619, 17299, 17393, 17419,
    17449, 17737, 17921, 18049, 18223, 19073, 19183, 19477, 20021, 20323,
];

/// Projective `(X:Z)` point on a Montgomery curve.
#[derive(Clone)]
struct Point {
    x: BigUint,
    z: BigUint,
}

/// `2P` on `By^2 = x^3 + Ax^2 + x` with `a24 = (A + 2) / 4`.
fn ecm_double(p: &Point, a24: &BigUint, n: &BigUint) -> Point {
    let u = submod(&p.x, &p.z, n);
    let u = (&u * &u) % n;
    let v = (&p.x + &p.z) % n;
    let v = (&v * &v) % n;
    let w = submod(&v, &u, n);
    Point {
        x: (&u * &v) % n,
        z: (&w * ((&u + (a24 * &w) % n) % n)) % n,
    }
}

/// Differential addition `P + Q` given `pm = P - Q`.
fn ecm_dadd(p: &Point, q: &Point, pm: &Point, n: &BigUint) -> Point {
    let u = (submod(&p.x, &p.z, n) * ((&q.x + &q.z) % n)) % n;
    let v = (((&p.x + &p.z) % n) * submod(&q.x, &q.z, n)) % n;
    let s = (&u + &v) % n;
    let d = submod(&u, &v, n);
    Point {
        x: (((&s * &s) % n) * &pm.z) % n,
        z: (((&d * &d) % n) * &pm.x) % n,
    }
}

/// Montgomery ladder `k * P`.
fn ecm_mul(p: &Point, k: u64, a24: &BigUint, n: &BigUint) -> Point {
    if k == 1 {
        return p.clone();
    }
    let mut r0 = p.clone();
    let mut r1 = ecm_double(&r0, a24, n);
    if k == 2 {
        return r1;
    }
    for bit in (0..k.ilog2()).rev() {
        if (k >> bit) & 1 == 1 {
            r0 = ecm_dadd(&r0, &r1, p, n);
            r1 = ecm_double(&r1, a24, n);
        } else {
            r1 = ecm_dadd(&r0, &r1, p, n);
            r0 = ecm_double(&r0, a24, n);
        }
    }
    r0
}

/// Batch conversion to affine x (`ECM.pm::_tiny_batch_normalize_x`).
/// `Err(Some(f))` carries a factor; `Err(None)` abandons the curve.
fn ecm_normalize(points: &[Point], n: &BigUint) -> Result<Vec<BigUint>, Option<BigUint>> {
    let mut acc = one();
    let prefix: Vec<BigUint> = points
        .iter()
        .map(|pt| {
            acc = (&acc * &pt.z) % n;
            acc.clone()
        })
        .collect();
    let Some(mut inv) = invmod(&acc, n) else {
        let g = gcd(&acc, n);
        if !is_one(&g) && g != *n {
            return Err(Some(g));
        }
        // The product can hold different factors whose gcd together is n.
        return Err(points
            .iter()
            .map(|pt| gcd(&pt.z, n))
            .find(|g| !is_one(g) && g != n));
    };
    let mut xs = vec![zero(); points.len()];
    for j in (0..points.len()).rev() {
        let prev = if j == 0 { one() } else { prefix[j - 1].clone() };
        let zinv = (&inv * prev) % n;
        inv = (inv * &points[j].z) % n;
        xs[j] = (&points[j].x * zinv) % n;
    }
    Ok(xs)
}

/// The stage 2 plan of `ECM.pm::_tiny_stage2_plan`: the window centres and
/// the prime offsets each window covers.
struct Stage2Plan {
    d: u64,
    windows: Vec<Vec<usize>>,
}

fn stage2_plan(b1: u64, b2: u64) -> Stage2Plan {
    let mut d = (b2 >> 1).isqrt();
    if d % 2 == 1 {
        d += 1;
    }
    let flags = prime_flags(b2);
    let is_prime = |v: u64| usize::try_from(v).ok().and_then(|i| flags.get(i)) == Some(&true);
    let b2primes: Vec<u64> = (b1 + 1..=b2).filter(|&p| is_prime(p)).collect();
    let mut windows = Vec::new();
    let mut m = 1_u64;
    while m < b2 + d {
        let mut offsets = Vec::new();
        if m + d > b1 {
            let (lo, hi) = (m.saturating_sub(d), m + d);
            let first = b2primes.partition_point(|&p| p < lo);
            for &p in b2primes[first..].iter().take_while(|&&p| p <= hi) {
                if p < m {
                    offsets.push(m - p);
                } else if p > m {
                    if p <= 2 * m {
                        let mirror = 2 * m - p;
                        if mirror > b1 && mirror >= lo && is_prime(mirror) {
                            continue;
                        }
                    }
                    offsets.push(p - m);
                }
            }
        }
        windows.push(
            offsets
                .into_iter()
                .map(|v| usize::try_from(v).unwrap_or(usize::MAX))
                .collect(),
        );
        m += 2 * d;
    }
    Stage2Plan { d, windows }
}

/// Stage 2 result of one curve.
enum Stage2 {
    Found(BigUint),
    Abandon,
    Nothing,
}

/// `ECM.pm::_tiny_stage2`: the baby-step giant-step continuation.
fn ecm_stage2(q: &Point, a24: &BigUint, n: &BigUint, plan: &Stage2Plan) -> Stage2 {
    let f = gcd(&q.z, n);
    if f == *n {
        return Stage2::Abandon;
    }
    if !is_one(&f) {
        return Stage2::Found(f);
    }
    let two_d = usize::try_from(2 * plan.d).unwrap_or(usize::MAX);
    let mut nq: Vec<Point> = Vec::with_capacity(two_d + 1);
    nq.push(Point {
        x: zero(),
        z: zero(),
    });
    nq.push(q.clone());
    for i in 2..=two_d {
        let next = if i % 2 == 1 {
            ecm_dadd(&nq[i / 2], &nq[i.div_ceil(2)], q, n)
        } else {
            ecm_double(&nq[i / 2], a24, n)
        };
        nq.push(next);
    }
    let mut s = vec![q.clone()];
    let mut xm = nq[two_d - 1].clone();
    for w in 1..plan.windows.len() {
        let old = s[w - 1].clone();
        s.push(ecm_dadd(&nq[two_d], &old, &xm, n));
        xm = old;
    }
    let mut batch: Vec<Point> = nq[1..=two_d].to_vec();
    batch.extend(s.iter().skip(1).cloned());
    let xs = match ecm_normalize(&batch, n) {
        Ok(xs) => xs,
        Err(Some(f)) => return Stage2::Found(f),
        Err(None) => return Stage2::Abandon,
    };
    let (nqx, sx) = xs.split_at(two_d);
    let mut g = one();
    let mut f = one();
    for (w, offsets) in plan.windows.iter().enumerate() {
        if offsets.is_empty() {
            continue;
        }
        let swx = if w == 0 { &nqx[0] } else { &sx[w - 1] };
        for &i in offsets {
            g = (g * submod(swx, &nqx[i - 1], n)) % n;
        }
        f = gcd(&g, n);
        if !is_one(&f) {
            break;
        }
    }
    if f == *n {
        Stage2::Abandon
    } else if is_one(&f) {
        Stage2::Nothing
    } else {
        Stage2::Found(f)
    }
}

/// `ECM.pm::_factor_tiny`: up to `curves` Montgomery curves from the sigma
/// table starting at `sigma_offset`, stage 1 to `b1` and stage 2 to `b2`.
fn ecm(
    n: &BigUint,
    b1: u64,
    b2: u64,
    curves: u64,
    sigma_offset: u64,
    deadline: Deadline,
) -> Option<BigUint> {
    let primes: Vec<u64> = crate::sieve::primes(2, b1)
        .into_iter()
        .map(|p| prime_power(p, b1))
        .collect();
    let plan = (b2 > b1).then(|| stage2_plan(b1, b2));
    for ci in 0..curves {
        if deadline.expired() {
            return None;
        }
        let curve = usize::try_from((sigma_offset + ci) % 300).unwrap_or(0);
        let sigma = u64::from(ECM_SIGMAS[curve]);
        let u = big(sigma * sigma - 5);
        let v = big(4 * sigma);
        let umv = &u - &v;
        let t3uv = &u * 3_u8 + &v;
        let u3 = (&u * &u * &u) % n;
        let v3 = (&v * &v * &v) % n;
        let umv3 = (&umv * &umv * &umv) % n;
        let num = submod(&zero(), &((umv3 * t3uv) % n), n);
        let den = (u3.clone() * &v * 16_u8) % n;
        let g = gcd(&den, n);
        if g == *n {
            continue;
        }
        if !is_one(&g) {
            return Some(g);
        }
        let Some(den_inv) = invmod(&den, n) else {
            continue;
        };
        let a24 = (num * den_inv) % n;
        let mut pt = Point { x: u3, z: v3 };
        let mut abandoned = false;
        for (j, &k) in primes.iter().enumerate() {
            pt = ecm_mul(&pt, k, &a24, n);
            if j % 64 == 0 {
                let g = gcd(&pt.z, n);
                if g == *n {
                    abandoned = true;
                    break;
                }
                if !is_one(&g) {
                    return Some(g);
                }
                if deadline.expired() {
                    return None;
                }
            }
        }
        if abandoned {
            continue;
        }
        let g = gcd(&pt.z, n);
        if g == *n {
            continue;
        }
        if !is_one(&g) {
            return Some(g);
        }
        if let Some(plan) = &plan {
            match ecm_stage2(&pt, &a24, n, plan) {
                Stage2::Found(f) => return Some(f),
                Stage2::Abandon | Stage2::Nothing => {}
            }
        }
    }
    None
}

/// Splits `n` with the elliptic curve method.
///
/// Port of MPU 0.75 `Math::Prime::Util::ECM::ecm_factor_pp`: Montgomery
/// curves with Suyama's parameterization from a fixed sigma table, a stage 1
/// to `b1` and a baby-step giant-step stage 2 to `b2`. With no first bound
/// the bounds ramp from 500 by factors of 4 over seven steps of 40 curves,
/// as PP does. A first bound alone sets the second to 20 times it and the
/// curve count to 10. Factors need not be prime. Zero returns `[0]`; one
/// returns `[]`.
///
/// ```
/// use math_prime_util::bigint::factor::ecm_factor;
/// use num_bigint::BigUint;
/// let n = BigUint::from(1_000_003_u32) * BigUint::from(10_000_019_u32);
/// let got = ecm_factor(&n, Some(500), None, None);
/// assert_eq!(got, vec![BigUint::from(1_000_003_u32), BigUint::from(10_000_019_u32)]);
/// ```
pub fn ecm_factor(
    n: &BigUint,
    b1: Option<u64>,
    b2: Option<u64>,
    curves: Option<u64>,
) -> Vec<BigUint> {
    method(n, |m| match b1 {
        Some(b1) => {
            let b2 = match b2 {
                Some(b2) if b2 > 0 => b2,
                _ => b1.saturating_mul(20),
            };
            ecm(m, b1, b2, curves.unwrap_or(10), 0, Deadline::NONE)
        }
        None => (0..7_u64).find_map(|step| {
            let b1 = 500_u64 << (2 * step);
            ecm(m, b1, 20 * b1, 40, 40 * step, Deadline::NONE)
        }),
    })
}

/// One stage of the `PP.pm::factor` recipe (`@_fsublist`).
#[derive(Clone, Copy)]
enum Stage {
    Pminus1(u64, u64),
    Ecm(u64, u64, u64, u64),
    Holf(u64),
    Brent(u64, u64),
}

/// The recipe after the perfect-power check, in PP's order.
const STAGES: [Stage; 13] = [
    Stage::Pminus1(65_536, 65_536),
    Stage::Ecm(500, 25_000, 20, 0),
    Stage::Ecm(2_000, 100_000, 20, 20),
    Stage::Ecm(8_000, 160_000, 20, 40),
    Stage::Holf(256 * 1024),
    Stage::Ecm(40_000, 800_000, 10, 60),
    Stage::Ecm(80_000, 1_600_000, 10, 70),
    Stage::Ecm(160_000, 3_200_000, 10, 80),
    Stage::Brent(512 * 1024, 7),
    Stage::Holf(1024 * 1024),
    Stage::Ecm(320_000, 6_400_000, 20, 90),
    Stage::Ecm(1_000_000, 20_000_000, 20, 110),
    Stage::Holf(2048 * 1024),
];

/// The factoring recipe of `PP.pm::factor` on one composite cofactor with
/// no factor below 5000: a factor, or `None` when every stage failed or the
/// deadline passed.
fn recipe(n: &BigUint, deadline: Deadline) -> Option<BigUint> {
    let k = is_power_exp(n);
    if k > 1 {
        return Some(rootint(n, k));
    }
    let mut holf_pos = 1_u64;
    for stage in STAGES {
        if deadline.expired() {
            return None;
        }
        let f = match stage {
            Stage::Pminus1(b1, b2) => pminus1(n, b1, b2, deadline),
            Stage::Ecm(b1, b2, curves, offset) => ecm(n, b1, b2, curves, offset, deadline),
            Stage::Holf(rounds) => {
                let f = holf(n, rounds, holf_pos, deadline);
                holf_pos += rounds;
                f
            }
            Stage::Brent(rounds, a) => brent(n, rounds, a, deadline),
        };
        if f.is_some() {
            return f;
        }
    }
    None
}

/// `PP.pm::factor`: trial division to 4999, then the recipe on a stack of
/// cofactors. `native` hands cofactors below 2^128 to the native path.
fn run(n: &BigUint, deadline: Deadline, native: bool) -> Factored {
    const LIMIT: u64 = 4999;
    let mut result = Factored::default();
    if *n < big(4) {
        if !is_one(n) {
            result.primes.push(n.clone());
        }
        return result;
    }
    if native {
        if let Ok(v) = u128::try_from(n) {
            if let Ok(factors) = crate::factor::factor_u128(v) {
                result.primes = factors.into_iter().map(BigUint::from).collect();
                return result;
            }
        }
    }
    let mut factors = trial_factor(n, Some(LIMIT));
    let Some(last) = factors.pop() else {
        return result;
    };
    result.primes = factors;
    let mut stack = vec![last];
    while let Some(m) = stack.pop() {
        if m < big(LIMIT * LIMIT) || is_prob_prime(&m) != 0 {
            result.primes.push(m);
            continue;
        }
        if native {
            if let Ok(v) = u128::try_from(&m) {
                if let Ok(factors) = crate::factor::factor_u128(v) {
                    result.primes.extend(factors.into_iter().map(BigUint::from));
                    continue;
                }
            }
        }
        match recipe(&m, deadline) {
            Some(f) => {
                stack.push(&m / &f);
                stack.push(f);
            }
            None => result.unsplit.push(m),
        }
    }
    result.primes.sort_unstable();
    result.unsplit.sort_unstable();
    result
}

/// Prime factors with multiplicity, in ascending order.
///
/// Port of MPU 0.75 `PP.pm::factor`: trial division to 4999, then on each
/// cofactor a primality test and the recipe of perfect-power check, p-1
/// (B1 65536), ECM at rising bounds, HOLF and Pollard-Brent. Cofactors below
/// 2^128 take the native path ([`factor_u128`](crate::factor_u128)), as PP
/// hands them to XS. There is no time limit: a 200-bit input with two
/// 100-bit prime factors runs the whole recipe, which takes minutes;
/// [`factor_within`] stops at a deadline. Zero returns `[0]`; one returns
/// `[]`.
///
/// # Errors
/// Returns the first cofactor the recipe could not split. MPU falls back to
/// full trial division there.
///
/// ```
/// use math_prime_util::bigint::factor::factor;
/// use num_bigint::BigUint;
/// let p = (BigUint::from(1_u8) << 89_usize) - 1_u8;
/// let q = BigUint::from(1_000_000_007_u64);
/// assert_eq!(factor(&(&p * &q * 4_u8)), Ok(vec![BigUint::from(2_u8), BigUint::from(2_u8), q, p]));
/// ```
pub fn factor(n: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    let mut result = run(n, Deadline::NONE, true);
    if result.unsplit.is_empty() {
        Ok(result.primes)
    } else {
        Err(result.unsplit.swap_remove(0))
    }
}

/// Prime factors found within `limit`, and the cofactors left unsplit.
///
/// The same recipe as [`factor`]; the stages check the deadline between
/// batches of work, so the call returns shortly after the limit. The
/// product of all primes and unsplit cofactors is `n`.
///
/// ```
/// use math_prime_util::bigint::factor::factor_within;
/// use num_bigint::BigUint;
/// use std::time::Duration;
/// let n = BigUint::from(360_u16);
/// let got = factor_within(&n, Duration::from_secs(1));
/// assert_eq!(got.primes.len(), 6);
/// assert!(got.unsplit.is_empty());
/// ```
pub fn factor_within(n: &BigUint, limit: Duration) -> Factored {
    run(n, Deadline::after(limit), true)
}

/// Prime factors and their exponents, in ascending order of the prime.
///
/// Port of MPU 0.75 `PP.pm::factor_exp` through [`factor`]. One returns
/// `[]`; zero returns `[(0, 1)]`.
///
/// # Errors
/// Returns the first cofactor the recipe could not split, as [`factor`].
///
/// ```
/// use math_prime_util::bigint::factor::factor_exp;
/// use num_bigint::BigUint;
/// let got = factor_exp(&(BigUint::from(1_u8) << 100_usize)).unwrap();
/// assert_eq!(got, vec![(BigUint::from(2_u8), 100)]);
/// ```
pub fn factor_exp(n: &BigUint) -> Result<Vec<(BigUint, u32)>, BigUint> {
    Ok(group(factor(n)?))
}

/// Groups sorted factors into `(prime, exponent)` pairs.
pub(super) fn group(factors: Vec<BigUint>) -> Vec<(BigUint, u32)> {
    let mut grouped: Vec<(BigUint, u32)> = Vec::new();
    for f in factors {
        match grouped.last_mut() {
            Some((p, e)) if *p == f => *e += 1,
            _ => grouped.push((f, 1)),
        }
    }
    grouped
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Int;

    fn mersenne(bits: usize) -> BigUint {
        (one() << bits) - 1_u8
    }

    fn product(v: &[BigUint]) -> BigUint {
        v.iter().fold(one(), |acc, f| acc * f)
    }

    /// 2^89 - 1, prime; p - 1 has the factor 2931542417.
    fn p89() -> BigUint {
        mersenne(89)
    }

    /// 2^61 - 1, prime; p - 1 is 1321-smooth.
    fn p61() -> BigUint {
        mersenne(61)
    }

    fn dec(text: &str) -> BigUint {
        text.parse().unwrap()
    }

    /// An 89-bit prime with no smooth p - 1 or p + 1 (largest factors above 10^18).
    fn h89() -> BigUint {
        dec("383939849620258888044469981")
    }

    /// A 107-bit prime with no smooth p - 1 or p + 1.
    fn h107() -> BigUint {
        dec("128630034069733398025664261656771")
    }

    /// A 100-bit prime with no smooth p - 1 or p + 1.
    fn q100() -> BigUint {
        dec("785051509458934440118451889551")
    }

    /// An 84-bit prime with p - 1 = 2 * 7 * 127 * 181 * 229 * 593 * 661 * 691 * 733 * 769 * 1321,
    /// and 1321 in the order of 2.
    fn smooth84() -> BigUint {
        dec("14863228214225451287091383")
    }

    #[test]
    fn helpers() {
        assert_eq!(prime_power(2, 100), 64);
        assert_eq!(prime_power(11, 100), 11);
        assert_eq!(prime_power(7, 48), 7);
        let flags = prime_flags(30);
        let primes: Vec<usize> = (0..=30).filter(|&i| flags[i]).collect();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        assert_eq!(prime_flags(0), vec![false]);
        assert_eq!(prime_flags(1), vec![false, false]);
        let mut seen = Vec::new();
        for_primes(2, 20, |p| {
            seen.push(p);
            p < 11
        });
        assert_eq!(seen, vec![2, 3, 5, 7, 11]);
        let mut count = 0;
        for_primes(2, 1_000_003, |_| {
            count += 1;
            true
        });
        assert_eq!(count, 78_499);
        assert_eq!(found(None, &big(15)), vec![big(15)]);
        assert_eq!(found(Some(big(15)), &big(15)), vec![big(15)]);
        assert_eq!(found(Some(big(1)), &big(15)), vec![big(15)]);
        assert_eq!(found(Some(zero()), &big(15)), vec![big(15)]);
        assert_eq!(found(Some(big(5)), &big(15)), vec![big(3), big(5)]);
        assert_eq!(found(Some(big(3)), &big(15)), vec![big(3), big(5)]);
        assert!(!Deadline::NONE.expired());
        assert!(Deadline::after(Duration::ZERO).expired());
        assert!(!Deadline::after(Duration::from_secs(3600)).expired());
        assert_eq!(
            group(vec![big(2), big(2), big(3)]),
            vec![(big(2), 2), (big(3), 1)]
        );
        assert_eq!(lucas_power(&big(7), 1, &big(101)), big(7));
        assert_eq!(lucas_power(&big(7), 0, &big(101)), big(7));
        assert_eq!(checked_sub(&big(3), &big(4)), None);
        assert_eq!(checked_sub(&big(4), &big(3)), Some(one()));
    }

    #[test]
    fn method_wrapper_edges() {
        let methods: [fn(&BigUint) -> Vec<BigUint>; 8] = [
            |n| prho_factor(n, None),
            |n| pbrent_factor(n, None, None),
            |n| pminus1_factor(n, None, None),
            |n| pplus1_factor(n, None),
            |n| holf_factor(n, None, None),
            |n| squfof_factor(n, None),
            |n| ecm_factor(n, None, None, None),
            |n| trial_factor(n, Some(1000)),
        ];
        for f in methods {
            assert_eq!(f(&zero()), vec![zero()]);
            assert_eq!(f(&one()), vec![]);
            assert_eq!(f(&big(2)), vec![big(2)]);
            assert_eq!(f(&big(3)), vec![big(3)]);
            assert_eq!(f(&big(30)), vec![big(2), big(3), big(5)]);
            assert_eq!(f(&big(60)), vec![big(2), big(2), big(3), big(5)]);
            assert_eq!(f(&(p89() * 10_u8)), vec![big(2), big(5), p89()]);
            let got = f(&big(7 * 11));
            assert_eq!(product(&got), big(77));
            assert!(got.iter().all(|v| *v > one()));
        }
    }

    #[test]
    fn trial_division() {
        assert_eq!(trial_factor(&big(4), None), vec![big(2), big(2)]);
        assert_eq!(trial_factor(&big(49), Some(5)), vec![big(49)]);
        assert_eq!(trial_factor(&big(49), Some(2)), vec![big(7), big(7)]);
        assert_eq!(trial_factor(&big(49), Some(7)), vec![big(7), big(7)]);
        assert_eq!(
            trial_factor(&big(2 * 49 * 101), Some(50)),
            vec![big(2), big(7), big(7), big(101)]
        );
        assert_eq!(trial_factor(&(one() << 70_usize), None), vec![big(2); 70]);
        let n = &p89() * &p89() * 1009_u16 * 1009_u16;
        assert_eq!(
            trial_factor(&n, Some(5000)),
            vec![big(1009), big(1009), &p89() * &p89()]
        );
        assert_eq!(
            trial_factor(&(big(3) * big(1_000_003)), None),
            vec![big(3), big(1_000_003)]
        );
        let m = big(1_000_003) * big(1_000_033);
        assert_eq!(trial_factor(&m, None), vec![big(1_000_003), big(1_000_033)]);
        assert_eq!(trial_factor(&big(1_000_003), None), vec![big(1_000_003)]);
        assert_eq!(
            trial_factor(&(p89() * 7_u8), Some(100)),
            vec![big(7), p89()]
        );
    }

    #[test]
    fn rho_and_brent_split_and_fail() {
        let n = big(1_000_003) * big(10_000_019);
        let split = vec![big(1_000_003), big(10_000_019)];
        assert_eq!(prho_factor(&n, None), split);
        assert_eq!(pbrent_factor(&n, None, Some(3)), split);
        assert_eq!(prho_factor(&n, Some(1)), vec![n.clone()]);
        assert_eq!(pbrent_factor(&n, Some(1), None), vec![n.clone()]);
        assert_eq!(pbrent_factor(&n, Some(5), Some(2)), vec![n.clone()]);
        assert_eq!(brent(&n, 100_000, 1, Deadline::after(Duration::ZERO)), None);
        // Squares of a prime: every gcd is trivial or n, so the restart paths run.
        for p in [7_u64, 11, 13, 101, 1009] {
            let sq = big(p) * big(p);
            let got = prho_factor(&sq, Some(5000));
            assert!(
                got == vec![big(p), big(p)] || got == vec![sq.clone()],
                "{p}"
            );
            let got = pbrent_factor(&sq, Some(5000), None);
            assert!(
                got == vec![big(p), big(p)] || got == vec![sq.clone()],
                "{p}"
            );
        }
        let n = p89() * big(1_000_003);
        assert_eq!(pbrent_factor(&n, None, None), vec![big(1_000_003), p89()]);
        assert_eq!(prho_factor(&n, None), vec![big(1_000_003), p89()]);
    }

    #[test]
    fn pminus1_stages() {
        let n = smooth84() * q100();
        let split = vec![smooth84(), q100()];
        // Stage 1: 1321 is below the bound.
        assert_eq!(pminus1_factor(&n, Some(1500), Some(15_000)), split);
        // Stage 2: 1321 lies between the bounds.
        assert_eq!(pminus1_factor(&n, Some(1000), Some(1400)), split);
        assert_eq!(pminus1_factor(&n, Some(1000), None), split);
        assert_eq!(pminus1_factor(&n, Some(1000), Some(1200)), vec![n.clone()]);
        assert_eq!(pminus1_factor(&n, Some(3), None), vec![n.clone()]);
        // The ramp: 1000 fails (no stage 2), 100000 finds it.
        assert_eq!(pminus1_factor(&n, None, None), split);
        assert_eq!(
            pminus1(&n, 1500, 15_000, Deadline::after(Duration::ZERO)),
            None
        );
        assert_eq!(
            pminus1(&n, 1000, 1400, Deadline::after(Duration::ZERO)),
            None
        );
        // Base 2 has order k modulo the Mersenne prime 2^k - 1: found at the prime k.
        assert_eq!(
            pminus1_factor(&(p61() * p89()), Some(100), Some(100)),
            vec![p61(), p89()]
        );
        assert_eq!(
            pminus1_factor(&(h89() * p89()), Some(100), Some(100)),
            vec![h89(), p89()]
        );
        // Both factors found at the same checkpoint force the backtrack.
        let a = big(1_000_003);
        let b = big(1_000_033);
        let n = &a * &b;
        assert_eq!(
            pminus1_factor(&n, Some(1_000_100), None),
            vec![a.clone(), b.clone()]
        );
        assert_eq!(
            pminus1_factor(&(&a * &a), Some(1_000_100), None),
            vec![a.clone(), a.clone()]
        );
        // A square of a prime: the gcd gives the prime once, the split is p * p.
        let n = p61() * p61();
        assert_eq!(
            pminus1_factor(&n, Some(1500), Some(15_000)),
            vec![p61(), p61()]
        );
        let n = h89() * q100();
        assert_eq!(
            pminus1_factor(&n, Some(10_000), Some(100_000)),
            vec![n.clone()]
        );
    }

    #[test]
    fn pplus1_and_holf() {
        // 4294967357 + 1 = 2 * 3 * 11^2 * 71 * 97 * 859 and seed 7 has order dividing p + 1.
        let p = big(4_294_967_357);
        let n = &p * p61();
        assert_eq!(pplus1_factor(&n, Some(1000)), vec![p.clone(), p61()]);
        assert_eq!(pplus1_factor(&n, Some(2)), vec![n.clone()]);
        assert_eq!(pplus1_factor(&n, Some(60)), vec![n.clone()]);
        let n = big(1_000_003) * big(10_000_019);
        assert_eq!(
            holf_factor(&n, None, None),
            vec![big(1_000_003), big(10_000_019)]
        );
        assert_eq!(holf_factor(&n, Some(1), None), vec![n.clone()]);
        assert_eq!(holf_factor(&n, Some(0), Some(0)), vec![n.clone()]);
        // A square n * i stops the scan with a trivial gcd, as PP does.
        assert_eq!(holf_factor(&big(7 * 7), Some(1), None), vec![big(49)]);
        assert_eq!(
            holf_factor(&big(7 * 11), Some(1000), Some(77)),
            vec![big(77)]
        );
        assert_eq!(
            holf_factor(&big(7 * 11), Some(1000), None),
            vec![big(7), big(11)]
        );
        let n = p89() * big(1_000_003);
        assert_eq!(holf_factor(&n, Some(1000), None), vec![n.clone()]);
    }

    #[test]
    fn squfof_splits() {
        for (a, b) in [
            (1_000_003_u64, 10_000_019_u64),
            (4_294_967_311, 4_294_967_357),
            (1_000_000_007, 1_000_000_009),
            (65_537, 65_539),
            (101, 103),
        ] {
            let n = big(a) * big(b);
            assert_eq!(squfof_factor(&n, None), vec![big(a), big(b)], "{a} {b}");
        }
        let n = big(1_000_003) * big(10_000_019);
        assert_eq!(squfof_factor(&n, Some(1)), vec![n.clone()]);
        assert_eq!(squfof_factor(&big(49), None), vec![big(7), big(7)]);
        assert_eq!(
            squfof_factor(&(p89() * 1_000_003_u32), Some(100)),
            vec![p89() * 1_000_003_u32]
        );
        let mut state = Squfof::start(big(1_000_003) * big(10_000_019) * 3_u8, 5).unwrap();
        assert_eq!(state.unit(), None);
        assert!(state.valid);
        assert!(Squfof::start(big(49), 5).is_none());
    }

    #[test]
    fn ecm_splits() {
        let n = big(1_000_003) * big(10_000_019);
        assert_eq!(
            ecm_factor(&n, Some(500), Some(5000), Some(50)),
            vec![big(1_000_003), big(10_000_019)]
        );
        assert_eq!(ecm_factor(&n, Some(2), Some(0), Some(1)), vec![n.clone()]);
        assert_eq!(ecm_factor(&n, Some(2), Some(3), Some(1)), vec![n.clone()]);
        assert_eq!(
            ecm(&n, 500, 5000, 50, 0, Deadline::after(Duration::ZERO)),
            None
        );
        // Stage 2 finds a factor whose curve order has one prime between the bounds.
        let mut stage2_found = false;
        for (a, b) in [
            (1_000_003_u64, 1_000_033_u64),
            (999_983, 1_000_003),
            (100_003, 1_000_003),
        ] {
            let n = big(a) * big(b);
            for sigma in 0..300 {
                let f = ecm(&n, 20, 20_000, 1, sigma, Deadline::NONE);
                let g = ecm(&n, 20, 20, 1, sigma, Deadline::NONE);
                if f.is_some() && g.is_none() {
                    stage2_found = true;
                }
                if let Some(f) = f {
                    assert_eq!((&n % f).bits(), 0);
                }
            }
        }
        assert!(stage2_found);
        let n = p89() * big(1_000_003);
        assert_eq!(
            ecm_factor(&n, None, None, None),
            vec![big(1_000_003), p89()]
        );
        // The sigma table wraps.
        let f = ecm(&n, 500, 10_000, 2, 299, Deadline::NONE);
        assert!(f.is_none() || f == Some(big(1_000_003)));
    }

    #[test]
    fn ecm_normalize_paths() {
        let n = big(7 * 11);
        let pt = |x: u64, z: u64| Point {
            x: big(x),
            z: big(z),
        };
        assert_eq!(ecm_normalize(&[pt(1, 7)], &n), Err(Some(big(7))));
        assert_eq!(ecm_normalize(&[pt(1, 7), pt(1, 11)], &n), Err(Some(big(7))));
        assert_eq!(ecm_normalize(&[pt(1, 0)], &n), Err(None));
        assert_eq!(
            ecm_normalize(&[pt(3, 2), pt(5, 3)], &n),
            Ok(vec![big(40), big(53)])
        );
        let p = pt(2, 3);
        let a24 = big(5);
        let doubled = ecm_double(&p, &a24, &n);
        let tripled = ecm_dadd(&doubled, &p, &p, &n);
        assert_eq!(ecm_mul(&p, 1, &a24, &n).x, p.x);
        assert_eq!(ecm_mul(&p, 2, &a24, &n).x, doubled.x);
        assert_eq!(ecm_mul(&p, 3, &a24, &n).x, tripled.x);
        let plan = stage2_plan(20, 100);
        assert_eq!(plan.d, 8);
        assert!(plan.windows.iter().any(|w| !w.is_empty()));
        // A stage 2 on a curve point with a trivial z abandons the curve.
        let plan = stage2_plan(2, 50);
        assert!(matches!(
            ecm_stage2(&pt(1, 0), &a24, &n, &plan),
            Stage2::Abandon
        ));
        assert!(matches!(ecm_stage2(&pt(1, 7), &a24, &n, &plan), Stage2::Found(f) if f == big(7)));
    }

    #[test]
    fn recipe_and_run_on_small_inputs() {
        // The recipe without the native hand-off, on inputs the native path would take.
        let n = big(1_000_003) * big(10_000_019) * big(4999) * big(2);
        let got = run(&n, Deadline::NONE, false);
        assert_eq!(
            got.primes,
            vec![big(2), big(4999), big(1_000_003), big(10_000_019)]
        );
        assert_eq!(got.unsplit, Vec::<BigUint>::new());
        let got = run(&(big(1_000_003) * big(1_000_003)), Deadline::NONE, false);
        assert_eq!(got.primes, vec![big(1_000_003); 2]);
        let got = run(&(big(5_003) * big(5_009)), Deadline::NONE, false);
        assert_eq!(got.primes, vec![big(5_003), big(5_009)]);
        for n in [
            1_u64,
            2,
            3,
            4,
            24_990_001,
            25_000_000,
            4_294_967_311 * 65_537,
        ] {
            let got = run(&big(n), Deadline::NONE, false);
            assert_eq!(product(&got.primes), big(n), "{n}");
            assert_eq!(got.unsplit, Vec::<BigUint>::new());
            assert!(got.primes.iter().all(|p| p.is_prime() != 0));
        }
        assert_eq!(run(&one(), Deadline::NONE, false).primes, vec![]);
        assert_eq!(run(&zero(), Deadline::NONE, true).primes, vec![zero()]);
        assert_eq!(run(&one(), Deadline::NONE, true).primes, vec![]);
        // A cube and a Mersenne product over the native range.
        let cube = &p89() * &p89() * &p89();
        assert_eq!(factor(&cube), Ok(vec![p89(); 3]));
        let n = mersenne(107) * big(1_000_003) * big(2_147_483_647);
        assert_eq!(
            factor(&n),
            Ok(vec![big(1_000_003), big(2_147_483_647), mersenne(107)])
        );
        assert_eq!(
            factor_exp(&(&cube * 8_u8)),
            Ok(vec![(big(2), 3), (p89(), 3)])
        );
        assert_eq!(factor_exp(&zero()), Ok(vec![(zero(), 1)]));
        assert_eq!(factor_exp(&one()), Ok(vec![]));
        // Two 89-bit and 107-bit primes: the deadline stops the recipe.
        let hard = h89() * h107();
        let got = factor_within(&hard, Duration::from_millis(50));
        assert_eq!(got.unsplit, vec![hard.clone()]);
        assert_eq!(got.primes, Vec::<BigUint>::new());
        let got = factor_within(&(&hard * 6_u8), Duration::from_millis(50));
        assert_eq!(got.unsplit, vec![hard.clone()]);
        assert_eq!(got.primes, vec![big(2), big(3)]);
        let got = factor_within(&(&hard * &hard), Duration::from_millis(50));
        assert_eq!(got.unsplit, vec![hard.clone(), hard.clone()]);
        assert_eq!(
            factor_within(&(one() << 300_usize), Duration::from_secs(5)).primes,
            vec![big(2); 300]
        );
        let n = &hard * big(1_000_003);
        let got = factor_within(&n, Duration::from_secs(2));
        assert_eq!(product(&got.primes) * product(&got.unsplit), n);
        assert_eq!(
            BigUint::from(360_u16).factor(),
            Ok(vec![big(2), big(2), big(2), big(3), big(3), big(5)])
        );
        assert_eq!(cube.factor(), Ok(vec![p89(); 3]));
    }

    #[test]
    fn deadline_stops_each_stage() {
        let n = h89() * h107();
        let expired = Deadline::after(Duration::ZERO);
        assert_eq!(recipe(&n, expired), None);
        assert_eq!(ecm(&n, 500, 25_000, 20, 0, expired), None);
        assert_eq!(pminus1(&n, 65_536, 65_536, expired), None);
        assert_eq!(brent(&n, 1 << 20, 7, expired), None);
        assert_eq!(holf(&n, 1 << 20, 1, expired), None);
        assert_eq!(recipe(&(&p89() * &p89()), expired), Some(p89()));
    }
}

/// Tries Fermat's difference of squares method.
/// Port of MPU 0.75 `factor.c::fermat_factor` and `PP.pm::_factor_fermat`.
/// `None` uses 64 million rounds. A failed attempt keeps the cofactor.
///
/// ```
/// use math_prime_util::bigint::factor::fermat_factor;
/// assert_eq!(fermat_factor(&91_u8.into(), Some(10)), vec![7_u8.into(), 13_u8.into()]);
/// ```
#[must_use]
pub fn fermat_factor(n: &BigUint, rounds: Option<u64>) -> Vec<BigUint> {
    if let Ok(native) = u64::try_from(n) {
        return crate::fermat_factor(native, rounds).into_iter().map(BigUint::from).collect();
    }
    method(n, |m| {
        let mut a = sqrtint(m);
        if &a * &a == *m { return Some(a); }
        a += 1_u8;
        for _ in 0..rounds.unwrap_or(64_000_000) {
            let b2 = &a * &a - m;
            let b = sqrtint(&b2);
            if &b * &b == b2 { return Some(a - b); }
            a += 1_u8;
        }
        None
    })
}

/// Tries Chebyshev factoring with a prime power ladder.
/// Port of MPU 0.75 `factor.c::cheb_factor` and `PP.pm::_factor_cheb`.
/// Zero selects the default bound or the initial value 72.
///
/// ```
/// use math_prime_util::bigint::factor::cheb_factor;
/// assert_eq!(cheb_factor(&403_u16.into(), 0, 0), vec![13_u8.into(), 31_u8.into()]);
/// ```
#[must_use]
pub fn cheb_factor(n: &BigUint, bound: u64, init_x: u64) -> Vec<BigUint> {
    method(n, |m| {
        let default_bound = 8_u64.saturating_mul((m.bits() - 1).saturating_pow(2));
        let bound = if bound == 0 { default_bound } else { bound };
        let bound = u64::try_from(sqrtint(m)).map_or(bound, |root| bound.min(root));
        let mut x = big(if init_x == 0 { 72 } else { init_x }) % m;
        let inverse = (m + 1_u8) >> 1;
        for p in crate::primes_in(2..=bound) {
            let mut power = p;
            while power <= bound / p { power *= p; }
            let (_, v, _) = super::primality::lucas_sequence(m,
                &num_bigint::BigInt::from((2_u8 * &x) % m), &num_bigint::BigInt::from(1), &big(power));
            x = (v * &inverse) % m;
            let f = gcd(&if x.bits() == 0 { m - 1_u8 } else { &x - 1_u8 }, m);
            if f != one() { return Some(f); }
        }
        None
    })
}
