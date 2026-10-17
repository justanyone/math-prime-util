use super::{factor_powers, kronecker_unsigned};
use crate::{gcd, is_power, is_square, powmod};

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
    // MPU tests small factors before it factors phi.
    for q in [2, 3, 5] {
        if phi % q == 0 && powmod(a, phi / q, p) == Some(1) {
            return Some(false);
        }
    }
    Some(
        factor_powers(phi)
            .iter()
            .all(|&(q, _)| q <= 5 || powmod(a, phi / q, p) != Some(1)),
    )
}

