use super::nth_ramanujan_prime_approx::inverse;
use super::nth_ramanujan_prime_lower::{SMALL, count_lower, count_upper};

/// Return an upper bound for the count of Ramanujan primes at most `n`.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `ramanujan_prime_count_upper`.
/// Invert the lower bound for the nth Ramanujan prime.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_prime_count_upper(1000), 76);
/// assert_eq!(math_prime_util::ramanujan_prime_count_upper(0), 0);
/// ```
pub fn ramanujan_prime_count_upper(n: u64) -> u64 {
    if n <= 241 {
        return SMALL.partition_point(|&p| p <= n) as u64;
    }
    let lo = count_lower(n) / 3;
    let hi = count_upper(n) / 2;
    let bound = |k| super::nth_ramanujan_prime_lower(k).unwrap_or(0);
    if bound(lo) > n {
        return lo;
    }
    if bound(hi) < n {
        return hi;
    }
    inverse(lo, hi, n, bound)
}
