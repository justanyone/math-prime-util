//! Totients and multiplicative arithmetic from MPU 0.75.
//! Divisor sums, discriminants, and arithmetic in residue rings.

mod sopfr;
pub use sopfr::sopfr;
mod sopf;
pub use sopf::sopf;
mod kronecker;
pub use kronecker::kronecker;
mod chinese;
pub use chinese::chinese;
mod qnr;
pub use qnr::qnr;
mod is_qr;
pub use is_qr::is_qr;
mod is_fundamental;
pub use is_fundamental::is_fundamental;
mod ramanujan_sum;
pub use ramanujan_sum::ramanujan_sum;
mod valuation;
pub use valuation::valuation;

use crate::{gcd, invmod, is_power, is_prime, is_square, powmod, rootint, sqrtint};

fn factors(mut n: u64) -> Vec<(u64, u32)> {
    fn split(n: u64, out: &mut Vec<u64>) {
        if n <= 1 {
            return;
        }
        if is_prime(n) != 0 {
            out.push(n);
            return;
        }
        // MPU factor.c::prho_factor: batches of 64 Floyd steps and backtracking.
        let (mut u, mut v, mut c) = (7_u64, 7_u64, 1_u64);
        loop {
            let step = |x: u64| {
                u64::try_from((u128::from(x) * u128::from(x) + u128::from(c)) % u128::from(n))
                    .unwrap()
            };
            let (old_u, old_v) = (u, v);
            let mut product = 1;
            for _ in 0..64 {
                u = step(u);
                v = step(step(v));
                product =
                    u64::try_from(u128::from(product) * u128::from(u.abs_diff(v)) % u128::from(n))
                        .unwrap();
            }
            let mut d = gcd(product, n);
            if d == 1 {
                continue;
            }
            if d == n {
                u = old_u;
                v = old_v;
                for _ in 0..=64 {
                    u = step(u);
                    v = step(step(v));
                    d = gcd(u.abs_diff(v), n);
                    if d != 1 {
                        break;
                    }
                }
            }
            if d > 1 && d < n {
                split(d, out);
                split(n / d, out);
                return;
            }
            u = u64::try_from((u128::from(u) + 2) % u128::from(n)).unwrap();
            v = u;
            c += 2;
        }
    }
    if n < 2 {
        return Vec::new();
    }
    let mut flat = Vec::new();
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        while n % p == 0 {
            flat.push(p);
            n /= p;
        }
    }
    split(n, &mut flat);
    flat.sort_unstable();
    let mut result: Vec<(u64, u32)> = Vec::new();
    for p in flat {
        if let Some((q, e)) = result.last_mut() {
            if *q == p {
                *e += 1;
                continue;
            }
        }
        result.push((p, 1));
    }
    result
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43] {
        while n > 1 && n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    let mut pending = vec![n];
    while let Some(n) = pending.pop() {
        if n < 2 {
            continue;
        }
        if is_prime(n) != 0 {
            factors.push(n);
            continue;
        }
        let mut a = 1;
        let divisor = loop {
            if let Some(d) = brent(n, a) {
                break d;
            }
            a += 1;
        };
        pending.push(divisor);
        pending.push(n / divisor);
    }
    factors.sort_unstable();
    factors
}

#[allow(clippy::cast_possible_truncation)] // Remainders modulo u64 fit u64.
fn brent(n: u64, mut a: u64) -> Option<u64> {
    let step =
        |x: u64, a: u64| ((u128::from(x) * u128::from(x) + u128::from(a)) % u128::from(n)) as u64;
    let inner = if n <= 4_000_000_000 { 32 } else { 160 };
    let (mut r, mut xi, mut xm) = (1_u64, 1, 1);
    let mut rounds = 500_000;
    let mut fails = 6;
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = xi;
        let mut f = 1;
        while left > 0 {
            let count = left.min(inner);
            saved = xi;
            left -= count;
            rounds -= count;
            xi = step(xi, a);
            let mut m = xi.abs_diff(xm);
            for _ in 1..count {
                xi = step(xi, a);
                m = (u128::from(m) * u128::from(xi.abs_diff(xm)) % u128::from(n)) as u64;
            }
            f = gcd(m, n);
            if f != 1 {
                break;
            }
        }
        if f == 1 {
            r *= 2;
            xm = xi;
            continue;
        }
        if f == n {
            xi = saved;
            loop {
                xi = step(xi, a);
                f = gcd(xi.abs_diff(xm), n);
                if f != 1 || r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        if f == n || f == 0 {
            if fails == 0 {
                break;
            }
            fails -= 1;
            xm = ((u128::from(xm) + 11) % u128::from(n)) as u64;
            xi = xm;
            a += 1;
        } else {
            return Some(f);
        }
    }
    None
}

fn factor_powers(n: u64) -> Vec<(u64, u32)> {
    let mut result: Vec<(u64, u32)> = Vec::new();
    for p in prime_factors(n) {
        if let Some(last) = result.last_mut() {
            if last.0 == p {
                last.1 += 1;
                continue;
            }
        }
        result.push((p, 1));
    }
    result
}

fn range_len(lo: u64, hi: u64) -> usize {
    usize::try_from(
        hi.checked_sub(lo)
            .and_then(|v| v.checked_add(1))
            .expect("invalid or oversized range"),
    )
    .expect("range fits usize")
}

mod mertens;
pub use mertens::mertens;
mod liouville;
pub use liouville::liouville;
mod sumliouville;
pub use sumliouville::sumliouville;
mod exp_mangoldt;
pub use exp_mangoldt::exp_mangoldt;

fn divisors(n: u64) -> Vec<u64> {
    let mut d = vec![1];
    for (p, e) in factors(n) {
        let len = d.len();
        let mut power = 1;
        for _ in 0..e {
            power *= p;
            for i in 0..len {
                d.push(d[i] * power);
            }
        }
    }
    d.sort_unstable();
    d
}

fn kronecker_unsigned(a: u64, mut b: u64) -> i32 {
    if b == 0 {
        return i32::from(a == 1);
    }
    let mut s = 1;
    let r = b.trailing_zeros();
    if r > 0 {
        if a % 2 == 0 {
            return 0;
        }
        if r % 2 == 1 && matches!(a % 8, 3 | 5) {
            s = -s;
        }
        b >>= r;
    }
    kronecker_odd(a, b, s)
}
fn kronecker_odd(mut a: u64, mut b: u64, mut s: i32) -> i32 {
    while a != 0 {
        let r = a.trailing_zeros();
        if r % 2 == 1 && matches!(b % 8, 3 | 5) {
            s = -s;
        }
        a >>= r;
        if a & b & 2 != 0 {
            s = -s;
        }
        (a, b) = (b % a, a);
    }
    if b == 1 { s } else { 0 }
}

/// Return the least solution and the combined modulus.
/// Return `None` for a zero modulus, inconsistent inputs, or modulus overflow.
/// An empty input gives `(0, 0)`.
///
/// Port of MPU 0.75 `util.c::chinese and _simple_chinese`.
///
/// ```
/// assert_eq!(math_prime_util::chinese2(&[(2, 3), (3, 5)]), Some((8, 15)));
/// ```
pub fn chinese2(congruences: &[(u64, u64)]) -> Option<(u64, u64)> {
    if congruences.is_empty() {
        return Some((0, 0));
    }
    // With two or more pairs, every input modulus must stay in the signed range (native C path).
    if congruences.len() > 1 && congruences.iter().any(|&(_, m)| m > u64::MAX >> 1) {
        return None;
    }
    let mut pairs = congruences.to_vec();
    pairs.sort_unstable_by_key(|a| std::cmp::Reverse(a.1));
    let (mut r, mut m) = pairs[0];
    if m == 0 {
        return None;
    }
    r %= m;
    for &(a, n) in &pairs[1..] {
        if n == 0 {
            return None;
        }
        let g = gcd(m, n);
        if r % g != a % g {
            return None;
        }
        let ng = n / g;
        let modulus = m.checked_mul(ng)?;
        let delta = (i128::from(a) - i128::from(r)) / i128::from(g);
        let delta = u64::try_from(delta.rem_euclid(i128::from(ng))).ok()?;
        let t = crate::mulmod(delta, invmod(m / g, ng)?, ng)?;
        r = u64::try_from((u128::from(r) + u128::from(m) * u128::from(t)) % u128::from(modulus))
            .ok()?;
        m = modulus;
    }
    Some((r, m))
}

fn lambda(n: u64) -> u64 {
    let mut result = 1;
    for (p, e) in factor_powers(n) {
        let term = if p == 2 && e > 2 {
            1 << (e - 2)
        } else {
            (p - 1) * p.pow(e - 1)
        };
        result = result / gcd(result, term) * term;
    }
    result
}

/// Return the multiplicative order of a modulo n.
/// Return `None` if n is zero or a is not a unit modulo n.
///
/// Port of MPU 0.75 `util.c::znorder`.
///
/// ```
/// assert_eq!(math_prime_util::znorder(2, 7), Some(3));
/// ```
pub fn znorder(a: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(1);
    }
    if gcd(a, n) != 1 {
        return None;
    }
    let mut k = lambda(n);
    for (p, e) in factor_powers(k) {
        k /= p.pow(e);
        let mut a1 = powmod(a, k, n)?;
        while a1 != 1 {
            k *= p;
            a1 = powmod(a1, p, n)?;
        }
    }
    Some(k)
}

/// Return the least primitive root modulo n.
/// Return `None` if n is zero or no root exists.
///
/// Port of MPU 0.75 `util.c::znprimroot`.
///
/// ```
/// assert_eq!(math_prime_util::znprimroot(7), Some(3));
/// ```
pub fn znprimroot(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 4 {
        return Some(n - 1);
    }
    if n % 4 == 0 {
        return None;
    }
    let even = n % 2 == 0;
    let odd = if even { n / 2 } else { n };
    let factors = factor_powers(odd);
    if factors.len() != 1 {
        return None;
    }
    let (p, e) = factors[0];
    if p == 3 && even {
        return Some(5);
    }
    let phi = p - 1;
    let phif = factor_powers(phi);
    for a in 2..p {
        if (even && a % 2 == 0) || matches!(a, 4 | 8 | 9) {
            continue;
        }
        if kronecker_unsigned(a, p) != -1 {
            continue;
        }
        if phif
            .iter()
            .skip(1)
            .any(|&(q, _)| powmod(a, phi / q, p) == Some(1))
        {
            continue;
        }
        if e == 1 || powmod(a, phi, p * p) != Some(1) {
            return Some(a);
        }
    }
    None
}

/// Test whether a is a primitive root modulo n.
/// Return `None` if n is zero.
///
/// Port of MPU 0.75 `util.c::is_primitive_root`.
///
/// ```
/// assert_eq!(math_prime_util::is_primitive_root(3, 7), Some(true));
/// ```
pub fn is_primitive_root(a: u64, n: u64) -> Option<bool> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(true);
    }
    let a = a % n;
    if a <= 1 {
        return Some(a == 1 && n <= 2);
    }
    if n <= 4 {
        return Some(a == n - 1);
    }
    if n % 4 == 0 || (n % 2 == 0 && a % 2 == 0) || is_square(a) || gcd(a, n) != 1 {
        return Some(false);
    }
    let odd = if n % 2 == 0 { n / 2 } else { n };
    let f = factor_powers(odd);
    if f.len() != 1 {
        return Some(false);
    }
    let (p, e) = f[0];
    if e > 1 && powmod(a, p - 1, p * p) == Some(1) {
        return Some(false);
    }
    if kronecker_unsigned(a, p) != -1 {
        return Some(false);
    }
    let phi = p - 1;
    if is_power(a, 3) && gcd(3, phi) != 1 {
        return Some(false);
    }
    Some(
        factor_powers(phi)
            .iter()
            .all(|&(q, _)| powmod(a, phi / q, p) != Some(1)),
    )
}

mod inverse_sigma0;
pub use inverse_sigma0::*;
mod divisor_sum;
pub use divisor_sum::divisor_sum;
mod ramanujan_tau;
pub use ramanujan_tau::ramanujan_tau;
mod aliquot_sum;
pub use aliquot_sum::aliquot_sum;
mod abundance;
pub use abundance::abundance;
mod euler_phi;
pub use euler_phi::{euler_phi, euler_phi_range};
mod jordan_totient;
pub use jordan_totient::jordan_totient;
mod carmichael_lambda;
pub use carmichael_lambda::carmichael_lambda;
mod moebius;
pub use moebius::{moebius, moebius_range};
mod sumtotient;
pub use sumtotient::sumtotient;
mod dedekind_psi;
pub use dedekind_psi::dedekind_psi;
mod inverse_totient;
pub use inverse_totient::inverse_totient;
mod is_totient;
pub use is_totient::is_totient;
