use crate::{is_prime, next_prime, primes, rootint, sqrtint};

const MAX_CHEN_PRIME: u64 = 18_446_744_073_709_551_437;

/// Return the first Chen prime greater than `n`.
/// A Chen prime is a prime `p` for which `p + 2` is prime or semiprime.
/// Return `None` if the next Chen prime does not fit in `u64`.
///
/// Port of MPU 0.75 `almost_primes.c`, `next_chen_prime`.
/// The semiprime test follows `util.c`, `is_semiprime`.
///
/// ```
/// use math_prime_util::next_chen_prime;
/// assert_eq!(next_chen_prime(167), Some(179));
/// assert_eq!(next_chen_prime(u64::MAX), None);
/// ```
pub fn next_chen_prime(n: u64) -> Option<u64> {
    let mut p = next_prime(n)?;
    while p <= MAX_CHEN_PRIME {
        if is_prime(p + 2) != 0 || is_semiprime(p + 2) {
            return Some(p);
        }
        p = next_prime(p + 2)?;
    }
    None
}

// MPU util.c, is_semiprime: divide small factors, then use the cube-root
// bound. This crate has no factor_one port yet, so the factor search uses
// trial division up to the cube root. That search is exact: a composite with
// no prime factor up to its cube root has exactly two prime factors.
fn is_semiprime(n: u64) -> bool {
    if n < 6 {
        return n == 4;
    }
    for p in [2, 3, 5] {
        if n % p == 0 {
            return is_prime(n / p) != 0;
        }
    }
    let bound = rootint(n, 3);
    for p in primes(7, bound.min(277)) {
        if n % p == 0 {
            return is_prime(n / p) != 0;
        }
    }
    if is_prime(n) != 0 {
        return false;
    }
    if bound <= 277 {
        return true;
    }
    let root = sqrtint(n);
    if root * root == n {
        return is_prime(root) != 0;
    }
    // A composite with no prime factor at most its cube root has two factors.
    for p in primes(281, bound) {
        if n % p == 0 {
            return is_prime(n / p) != 0;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semiprime_matches_factor_multiplicity() {
        for n in 0..10_000 {
            let mut rest = n;
            let mut count = 0;
            let mut p = 2;
            while rest > 1 && p <= rest / p {
                while rest % p == 0 {
                    rest /= p;
                    count += 1;
                }
                p += 1;
            }
            count += u32::from(rest > 1);
            assert_eq!(is_semiprime(n), count == 2, "n={n}");
        }
        assert!(is_semiprime(283 * 293));
        assert!(is_semiprime(1009 * 1009));
        assert!(!is_semiprime(1009_u64.pow(4)));
        assert!(is_semiprime(1009 * 1_000_003));
        assert!(!is_semiprime(283 * 283 * 293));
    }

    #[test]
    fn native_limit() {
        assert_eq!(next_chen_prime(MAX_CHEN_PRIME - 1), Some(MAX_CHEN_PRIME));
        assert_eq!(next_chen_prime(MAX_CHEN_PRIME), None);
        assert_eq!(next_chen_prime(u64::MAX), None);
    }
}
