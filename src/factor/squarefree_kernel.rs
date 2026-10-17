//! Squarefree kernel, ported from MPU `squarefree_kernel`.

/// Returns the product of the distinct prime factors of `n`.
///
/// Ported from `squarefree_kernel` in Math::Prime::Util (`powerfree.c`,
/// `squarefree_kernel`, MPU 0.75). MPU factors `n` with `factorint` and
/// multiplies the distinct primes. This port uses trial division instead,
/// because `factorint` is not ported yet. The result is the same, but the
/// cost grows with the square root of the largest prime factor of `n`.
/// `squarefree_kernel(0)` is `0`.
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
    // `d <= m / d` is `d * d <= m` without overflow for d near 2^32.
    while d <= m / d {
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

    #[test]
    fn kernel_native_width() {
        // 2^64 - 1 = 3 * 5 * 17 * 257 * 641 * 65537 * 6700417 is squarefree.
        assert_eq!(squarefree_kernel(u64::MAX), u64::MAX);
        // 2^63 = 2^63 has kernel 2; the largest trial divisor is 2.
        assert_eq!(squarefree_kernel(1 << 63), 2);
        // 3^40 has kernel 3.
        assert_eq!(squarefree_kernel(3_u64.pow(40)), 3);
    }
}
