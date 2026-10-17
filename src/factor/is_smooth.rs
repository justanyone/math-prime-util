use super::is_practical::factor_into;
use crate::{is_prime, next_prime};

/// Return whether all prime factors of `n` are at most `k`.
/// Both zero and one are smooth for every bound.
///
/// Port of `reference/mpu/util.c`, `is_smooth` and `SMOOTH_TEST`.
///
/// ```
/// assert!(math_prime_util::is_smooth(72, 3));
/// assert!(!math_prime_util::is_smooth(70, 5));
/// ```
pub fn is_smooth(mut n: u64, k: u64) -> bool {
    if n <= 1 {
        return true;
    }
    if k <= 1 {
        return false;
    }
    if n <= k {
        return true;
    }
    if k == 2 {
        return n.is_power_of_two();
    }
    n >>= n.trailing_zeros();
    if n <= k {
        return true;
    }
    let mut p = 3;
    loop {
        if n < p * p {
            return n <= k;
        }
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            if n <= p {
                return true;
            }
        }
        let next = next_prime(p).unwrap();
        if k < next {
            return n <= k;
        }
        p = next;
        if p >= 1000 {
            break;
        }
    }
    if n < p * p {
        return n <= k;
    }
    if is_prime(n) != 0 {
        return false;
    }
    let limit = if k <= 290_000 { k } else { 4999 };
    while p <= limit && p <= n / p {
        while n % p == 0 {
            n /= p;
        }
        p = next_prime(p).unwrap();
    }
    if n <= k {
        return true;
    }
    if k <= 290_000 || n < 5003 * 5003 {
        return false;
    }
    let mut factors = Vec::new();
    factor_into(n, &mut factors);
    factors.into_iter().all(|p| p <= k)
}
