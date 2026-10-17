//! Squarefree kernel, ported from MPU `squarefree_kernel`.

use crate::factor_exp;

/// Returns the product of the distinct prime factors of `n`.
///
/// Ported from `squarefree_kernel` in `Math::Prime::Util` (`powerfree.c`,
/// MPU 0.75): factor `n` and multiply the distinct primes. `squarefree_kernel(0)`
/// is `0` and `squarefree_kernel(1)` is `1`.
///
/// # Examples
///
/// ```
/// use math_prime_util::squarefree_kernel;
///
/// assert_eq!(squarefree_kernel(360), 30); // 2^3 * 3^2 * 5
/// assert_eq!(squarefree_kernel(1), 1);
/// ```
#[must_use]
pub fn squarefree_kernel(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    factor_exp(n).into_iter().map(|(p, _)| p).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_cases() {
        assert_eq!(squarefree_kernel(0), 0);
        assert_eq!(squarefree_kernel(1), 1);
        assert_eq!(squarefree_kernel(12), 6);
        assert_eq!(squarefree_kernel(97), 97);
        assert_eq!(squarefree_kernel(2 * 2 * 97), 2 * 97);
        assert_eq!(squarefree_kernel(u64::MAX), u64::MAX);
        assert_eq!(squarefree_kernel(1 << 63), 2);
        assert_eq!(squarefree_kernel(3_u64.pow(40)), 3);
    }

    #[test]
    fn semiprime_of_two_large_primes() {
        let p = 4_294_967_291_u64;
        let q = 4_294_967_279_u64;
        assert_eq!(squarefree_kernel(p * q), p * q);
        assert_eq!(squarefree_kernel(p * 9), 3 * p);
    }
}
