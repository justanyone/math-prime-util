use super::is_practical::factor_into;
use crate::{is_prime, next_prime, sqrtint};

/// Return whether all prime factors of `n` are at least `k`.
/// Zero is rough only for a zero bound. One is always rough.
///
/// Port of `reference/mpu/util.c`, `is_rough`.
///
/// ```
/// assert!(math_prime_util::is_rough(77, 7));
/// assert!(!math_prime_util::is_rough(77, 8));
/// ```
pub fn is_rough(n: u64, k: u64) -> bool {
    if n == 0 {
        return k == 0;
    }
    if n == 1 || k <= 1 {
        return true;
    }
    if k > n {
        return false;
    }
    if k == 2 {
        return true;
    }
    if k == 3 {
        return n & 1 != 0;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    if k <= 5 {
        return true;
    }
    if n % 5 == 0 {
        return false;
    }
    if k > sqrtint(n) {
        return is_prime(n) != 0;
    }
    let limit = if k <= 2500 { k } else { 200 };
    let mut p = 7;
    while p < k && p <= limit && p <= n / p {
        if n % p == 0 {
            return false;
        }
        p = next_prime(p).unwrap();
    }
    if k <= 2500 {
        return true;
    }
    let mut factors = Vec::new();
    factor_into(n, &mut factors);
    factors.into_iter().all(|p| p >= k)
}
