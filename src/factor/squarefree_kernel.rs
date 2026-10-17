//! Squarefree kernel, ported from MPU `squarefree_kernel`.

/// Returns the product of the distinct prime factors of `n`.
///
/// Ported from `squarefree_kernel` in Math::Prime::Util (`powerfree.c`,
/// MPU 0.75). MPU factors `n` with `factorint` and multiplies the distinct
/// primes. This port uses trial division instead, because `factorint` is in
/// another module. The result is the same. `squarefree_kernel(0)` is `0`.
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
    let mut m = n;
    let mut kernel = 1;
    let mut d = 2;
    while d * d <= m {
        if m % d == 0 {
            kernel *= d;
            while m % d == 0 {
                m /= d;
            }
        }
        d += 1;
    }
    if m > 1 {
        kernel *= m;
    }
    kernel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_cases() {
        assert_eq!(squarefree_kernel(0), 0);
        assert_eq!(squarefree_kernel(1), 1);
        assert_eq!(squarefree_kernel(4), 2);
        assert_eq!(squarefree_kernel(12), 6);
        assert_eq!(squarefree_kernel(97), 97);
        assert_eq!(squarefree_kernel(2 * 2 * 97), 2 * 97);
    }
}
