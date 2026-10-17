//! Squarefree kernel, ported from MPU `squarefree_kernel`.

use crate::primality::is_prime;

/// Returns the product of the distinct prime factors of `n`.
///
/// Ported from `squarefree_kernel` in Math::Prime::Util (`powerfree.c`,
/// `squarefree_kernel`, MPU 0.75). MPU factors `n` with `factorint` and
/// multiplies the distinct primes. This port factors `n` with Pollard-Brent
/// rho (`pbrent_factor` in `reference/mpu/factor.c`), using `is_prime` to stop
/// on primes, in place of `factorint`. The cost grows with the square root of
/// the second-largest prime factor of `n`, not with the largest.
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
    let mut primes = Vec::new();
    split_into_primes(n, &mut primes);
    primes.sort_unstable();
    primes.dedup();
    primes.iter().product()
}

/// Appends every prime factor of `n` (with repeats) to `out`.
fn split_into_primes(n: u64, out: &mut Vec<u64>) {
    if n == 1 {
        return;
    }
    if is_prime(n) != 0 {
        out.push(n);
        return;
    }
    let f = pollard_brent(n);
    split_into_primes(f, out);
    split_into_primes(n / f, out);
}

/// Returns a nontrivial factor of the composite `n`.
///
/// Brent's variant of Pollard's rho, as `pbrent_factor` in
/// `reference/mpu/factor.c` (MPU 0.75). The polynomial constant `c` is raised
/// when a run ends with the factor `n`.
fn pollard_brent(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    }
    let mut c = 1;
    loop {
        let f = |x: u64| mulmod(x, x, n).wrapping_add(c) % n;
        let m = 128;
        let mut y = 2;
        let mut x = y;
        let mut ys = y;
        let mut g = 1;
        let mut q = 1;
        let mut r = 1;
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            let mut k = 0;
            while k < r && g == 1 {
                ys = y;
                for _ in 0..m.min(r - k) {
                    y = f(y);
                    q = mulmod(q, x.abs_diff(y), n);
                }
                g = gcd(q, n);
                k += m;
            }
            r *= 2;
        }
        if g == n {
            loop {
                ys = f(ys);
                g = gcd(x.abs_diff(ys), n);
                if g > 1 {
                    break;
                }
            }
        }
        if g != n {
            return g;
        }
        c += 1;
    }
}

/// `a * b mod n` without overflow.
fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    u64::try_from(u128::from(a) * u128::from(b) % u128::from(n)).expect("residue is below n")
}

/// Greatest common divisor by the Euclidean algorithm.
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
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

    #[test]
    fn largest_u64_prime_is_its_own_kernel() {
        // 2^64 - 59 is the largest prime below 2^64. Trial division by odd
        // numbers up to its square root would need about 4.3e9 divisions.
        assert_eq!(squarefree_kernel(18_446_744_073_709_551_557), 18_446_744_073_709_551_557);
    }

    #[test]
    fn semiprime_of_two_large_primes() {
        // 4294967291 and 4294967279 are the two largest primes below 2^32.
        let p = 4_294_967_291_u64;
        let q = 4_294_967_279_u64;
        assert_eq!(squarefree_kernel(p * q), p * q);
        assert_eq!(squarefree_kernel(p * p), p);
        assert_eq!(squarefree_kernel(p * p * q), p * q);
    }

    #[test]
    fn helpers() {
        assert_eq!(mulmod(u64::MAX, u64::MAX, u64::MAX - 1), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(pollard_brent(8051) % 2, 1); // 8051 = 83 * 97
        assert_eq!(pollard_brent(4) , 2);
    }
}
