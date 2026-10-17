//! Powerful integers from MPU 0.75 `powerful.c`.
use crate::{int::is_power_exp, is_prime, rootint};

/// Test if every prime exponent is at least `k`.
/// Port of `powerful.c` / `is_powerful`.
/// ```
/// assert!(math_prime_util::is_powerful(72, 2));
/// ```
pub fn is_powerful(mut n: u64, k: u32) -> bool {
    if n <= 1 {
        return n == 1;
    }
    if k <= 1 {
        return true;
    }
    if n % 2 == 0 {
        if n.trailing_zeros() < k {
            return false;
        }
        n >>= n.trailing_zeros();
    }
    if n == 1 {
        return true;
    }
    if k > 40 {
        return false;
    }
    // The C quick checks reject small primes with too few factors.
    let primes: &[u64] = if k == 2 { &[3, 5, 7, 11, 13] } else { &[3, 5, 7, 11] };
    let exponent = k.min(4);
    for &p in primes {
        if n % p == 0 && n % p.pow(exponent) != 0 {
            return false;
        }
    }
    if is_power_exp(n) >= k {
        return true;
    }
    let mut p = 3_u64;
    while p <= rootint(n, 2 * k) {
        if is_prime(p) != 0 && n % p == 0 {
            let mut exponent = 0;
            while n % p == 0 {
                n /= p;
                exponent += 1;
            }
            if exponent < k {
                return false;
            }
            if n == 1 || is_power_exp(n) >= k {
                return true;
            }
        }
        p += 2;
    }
    false
}

