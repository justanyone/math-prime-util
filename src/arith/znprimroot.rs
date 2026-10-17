use super::{factor_powers, kronecker_unsigned};
use crate::powmod;

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
