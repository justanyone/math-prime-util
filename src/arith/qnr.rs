use super::{factor_powers, kronecker_unsigned};
use crate::{is_prime, next_prime};

/// Return the least quadratic nonresidue modulo `n`.
///
/// Return `None` if `n` is zero or no nonresidue exists.
/// The value is `0` or a prime in MPU. This port returns `None` for `0`.
///
/// Port of MPU 0.75 `util.c::qnr`.
///
/// ```
/// assert_eq!(math_prime_util::qnr(7), Some(3));
/// assert_eq!(math_prime_util::qnr(0), None);
/// ```
pub fn qnr(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if n <= 2 {
        return Some(n);
    }
    if kronecker_unsigned(2, n) == -1 {
        return Some(2);
    }
    if is_prime(n) != 0 {
        return (3..n).step_by(2).find(|&a| kronecker_unsigned(a, n) == -1);
    }
    let mut n = n;
    if n % 2 == 0 {
        let e = n.trailing_zeros();
        n >>= e;
        if e >= 2 || n == 1 {
            return Some(2);
        }
    }
    if [3, 5, 11, 13, 19].iter().any(|p| n % p == 0) {
        return Some(2);
    }
    let factors = factor_powers(n);
    let mut a = 2;
    while a < n {
        if factors
            .iter()
            .any(|&(p, _)| a < p && kronecker_unsigned(a, p) == -1)
        {
            return Some(a);
        }
        a = next_prime(a)?;
    }
    None
}
