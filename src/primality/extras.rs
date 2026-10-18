//! Catalan pseudoprime, Chen prime and delicate prime tests from MPU 0.75.
//!
//! Ports of `is_catalan_pseudoprime` and `is_delicate_prime` in `util.c`
//! and `is_chen_prime` in `almost_primes.c`.
use super::is_prime;
use crate::factor::sorted_factors;
use crate::modular::{mulmod, powmod};

/// Largest 64-bit prime that can be a Chen prime (`MAX_CHEN_PRIME` in `almost_primes.c`).
const MAX_CHEN_PRIME: u64 = 18_446_744_073_709_551_437;

/// Distinct prime factors of `n` with their exponents, ascending.
fn factor_pairs(n: u64) -> Vec<(u64, u32)> {
    let mut pairs: Vec<(u64, u32)> = Vec::new();
    for p in sorted_factors(n) {
        match pairs.last_mut() {
            Some((q, e)) if *q == p => *e += 1,
            _ => pairs.push((p, 1)),
        }
    }
    pairs
}

/// Whether `n` has exactly two prime factors counted with multiplicity.
fn is_semiprime(n: u64) -> bool {
    n >= 4 && sorted_factors(n).len() == 2
}

/// `_catalan_vtest`: true if some digit of `n` in base `p` (below the last) is odd-weighted.
fn catalan_vtest(mut n: u64, p: u64) -> bool {
    loop {
        n /= p;
        if n == 0 {
            return false;
        }
        if n % 2 == 1 {
            return true;
        }
    }
}

/// `_catalan_v`: the sum of `floor(n / p^k) mod 2` over `k >= 1`.
fn catalan_v(mut n: u64, p: u64) -> u32 {
    let mut s = 0;
    loop {
        n /= p;
        if n == 0 {
            return s;
        }
        s += u32::from(n % 2 == 1);
    }
}

/// `_catalan_mult`: multiplies `m` by `p^v` modulo `n`.
fn catalan_mult(m: u64, p: u64, n: u64, a: u64) -> u64 {
    if p > a {
        return mulmod(m, p, n).expect("positive modulus");
    }
    match catalan_v(a << 1, p) {
        0 => m,
        1 => mulmod(m, p, n).expect("positive modulus"),
        e => {
            let pw = powmod(p, u64::from(e), n).expect("positive modulus");
            mulmod(m, pw, n).expect("positive modulus")
        }
    }
}

/// Tests whether `n` is a Catalan pseudoprime: prime, or composite with
/// `(-1)^a * C(2a, a) = 2a+1 (mod n)`-type congruence, `a = (n-1)/2`
/// (Greathouse's algorithm; the only ones below 1e10 are 5907, 1194649 and 12327121).
///
/// Port of `is_catalan_pseudoprime` in MPU `util.c`. All primes pass.
///
/// ```
/// use math_prime_util::is_catalan_pseudoprime;
/// assert!(is_catalan_pseudoprime(13));
/// assert!(is_catalan_pseudoprime(5907));
/// assert!(!is_catalan_pseudoprime(15));
/// ```
#[must_use]
pub fn is_catalan_pseudoprime(n: u64) -> bool {
    if n < 2 || (n % 2 == 0 && n != 2) {
        return false;
    }
    if is_prime(n) != 0 {
        return true;
    }
    let a = n >> 1;
    let nf = factor_pairs(n);
    if nf.len() == 2 && nf[0].1 == 1 && nf[1].1 == 1 {
        // Conditions from Aebi and Cairns (2008).
        if n < 10_000_000_000 || 2 * nf[0].0 + 1 >= nf[1].0 {
            return false;
        }
    }
    if nf.iter().any(|&(p, _)| catalan_vtest(a << 1, p)) {
        return false;
    }
    let mut m = 1;
    for p in [2, 3, 5] {
        m = catalan_mult(m, p, n, a);
    }
    for p in crate::primes_in(7..=n) {
        m = catalan_mult(m, p, n, a);
    }
    if a & 1 == 1 { m == n - 1 } else { m == 1 }
}

/// Tests whether `n` is a Chen prime: `n` is prime and `n+2` is prime or a semiprime.
///
/// Port of `is_chen_prime` in MPU `almost_primes.c`.
///
/// ```
/// use math_prime_util::is_chen_prime;
/// assert!(is_chen_prime(5));
/// assert!(is_chen_prime(23));
/// assert!(!is_chen_prime(43));
/// ```
#[must_use]
pub fn is_chen_prime(n: u64) -> bool {
    if !(2..=MAX_CHEN_PRIME).contains(&n) {
        return false;
    }
    is_prime(n) != 0 && (is_prime(n + 2) != 0 || is_semiprime(n + 2))
}

/// Tests whether `n` is a delicate prime in base `b`: a prime where changing any
/// single digit (in base `b`) gives a composite number.
///
/// Returns `None` where MPU needs a bigint backend because a changed number would
/// overflow 64 bits (base 10 with `n >= 10^19`, or other bases near `u64::MAX`).
///
/// Port of `is_delicate_prime` in MPU `util.c`; the general base uses Stamm's
/// `isWeakly` algorithm (2020).
///
/// # Panics
///
/// Panics if `b < 2` (MPU: "`is_delicate_prime` base must be >= 2").
///
/// ```
/// use math_prime_util::is_delicate_prime;
/// assert_eq!(is_delicate_prime(294_001, 10), Some(true));
/// assert_eq!(is_delicate_prime(13, 10), Some(false));
/// ```
#[must_use]
pub fn is_delicate_prime(n: u64, b: u32) -> Option<bool> {
    assert!(b >= 2, "is_delicate_prime base must be >= 2");
    if b == 10 && n < 100 {
        return Some(false);
    }
    if b == 3 && n == 2 {
        return Some(true);
    }
    if is_prime(n) == 0 {
        return Some(false);
    }
    match b {
        10 => delicate_decimal(n),
        2 => Some(delicate_binary(n)),
        _ => delicate_general(n, u64::from(b)),
    }
}

fn delicate_decimal(n: u64) -> Option<bool> {
    const MAXD: u32 = 19;
    if n >= 10_u64.pow(MAXD) {
        return None;
    }
    // The last digit of a multi-digit prime must be 1, 3, 7 or 9.
    let dold = n % 10;
    for d in [1, 3, 7, 9] {
        if dold != d && is_prime(n - dold + d) != 0 {
            return Some(false);
        }
    }
    let mut digpow = 10_u64;
    let mut d = 1;
    while d <= MAXD && n >= digpow {
        let dold = (n / digpow) % 10;
        for dnew in 0..10 {
            if dnew != dold && is_prime(n - dold * digpow + dnew * digpow) != 0 {
                return Some(false);
            }
        }
        digpow *= 10;
        d += 1;
    }
    Some(true)
}

fn delicate_binary(n: u64) -> bool {
    if n < 127 {
        return false;
    }
    (1..=n.ilog2())
        .rev()
        .all(|bit| is_prime(n ^ (1_u64 << bit)) == 0)
}

fn delicate_general(n: u64, b: u64) -> Option<bool> {
    let mut bm = 1_u64;
    while n >= bm {
        let bmb = bm.wrapping_mul(b);
        if u64::MAX / b < bm || u64::MAX - bmb < n {
            return None;
        }
        // Check all n + j * b^m are composite.
        let mut counter = 0_u64;
        let mut current = n + bm;
        while n % bm != current % bmb {
            assert!(counter < b - 1, "is_delicate_prime overflow failure");
            if is_prime(current) != 0 {
                return Some(false);
            }
            counter += 1;
            current += bm;
        }
        // Check all n - j * b^m are composite.
        let mut current = n - bm;
        for _ in 1..(b - counter) {
            if is_prime(current) != 0 {
                return Some(false);
            }
            current = current.wrapping_sub(bm);
        }
        bm = bmb;
    }
    Some(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalan_branches() {
        assert!(!is_catalan_pseudoprime(0));
        assert!(!is_catalan_pseudoprime(1));
        assert!(!is_catalan_pseudoprime(4));
        assert!(is_catalan_pseudoprime(2));
        assert!(is_catalan_pseudoprime(5907));
        assert!(is_catalan_pseudoprime(1_194_649));
        assert!(is_catalan_pseudoprime(12_327_121));
        // Small semiprime and a prime square (a vtest rejection / residue mismatch).
        assert!(!is_catalan_pseudoprime(15));
        assert!(!is_catalan_pseudoprime(9));
        assert!(!is_catalan_pseudoprime(25));
        // Semiprime above 1e10 with 2p+1 >= q.
        assert!(!is_catalan_pseudoprime(100_003 * 100_019));
        // Odd `a` / even `a` result forms.
        assert!(!is_catalan_pseudoprime(21));
        assert!(!is_catalan_pseudoprime(2047));
    }

    #[test]
    fn catalan_helpers() {
        assert_eq!(catalan_v(10, 2), 2);
        assert!(catalan_vtest(6, 2));
        assert!(!catalan_vtest(1, 2));
        assert_eq!(catalan_mult(3, 7, 11, 2), 10);
        assert_eq!(catalan_mult(3, 2, 11, 5), 1);
    }

    #[test]
    fn chen() {
        assert!(!is_chen_prime(1));
        assert!(!is_chen_prime(u64::MAX));
        assert!(!is_chen_prime(9));
        assert!(is_chen_prime(2));
        assert!(is_chen_prime(3));
        assert!(is_chen_prime(5));
        assert!(is_chen_prime(7));
        assert!(is_chen_prime(13));
        assert!(!is_chen_prime(43));
        assert!(is_chen_prime(47));
        assert!(!is_semiprime(3));
        assert!(is_semiprime(15));
    }

    #[test]
    fn delicate() {
        assert_eq!(is_delicate_prime(97, 10), Some(false));
        assert_eq!(is_delicate_prime(2, 3), Some(true));
        assert_eq!(is_delicate_prime(15, 10), Some(false));
        assert_eq!(is_delicate_prime(294_001, 10), Some(true));
        assert_eq!(is_delicate_prime(10_000_000_000_000_000_051, 10), None);
        assert_eq!(is_delicate_prime(113, 10), Some(false));
        assert_eq!(is_delicate_prime(100, 2), Some(false));
        assert_eq!(is_delicate_prime(127, 2), Some(true));
        assert_eq!(is_delicate_prime(u64::MAX - 58, 7), None);
        assert_eq!(is_delicate_prime(163, 5), Some(false));
        assert_eq!(is_delicate_prime(2, 5), Some(false));
        assert_eq!(is_delicate_prime(3, 7), Some(false));
    }

    #[test]
    #[should_panic(expected = "base must be >= 2")]
    fn delicate_bad_base() {
        let _ = is_delicate_prime(7, 1);
    }
}
