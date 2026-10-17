use super::factor_powers;
use crate::{gcd, powmod};

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
    if a <= 1 {
        return (a == 1).then_some(1);
    }
    if gcd(a, n) != 1 {
        return None;
    }
    let mut k = lambda(n);
    for (p, e) in factor_powers(k) {
        k /= p.pow(e);
        let mut a1 = powmod(a, k, n)?;
        for _ in 0..e {
            if a1 == 1 {
                break;
            }
            k *= p;
            a1 = powmod(a1, p, n)?;
        }
        if a1 != 1 {
            return None;
        }
    }
    Some(k)
}
