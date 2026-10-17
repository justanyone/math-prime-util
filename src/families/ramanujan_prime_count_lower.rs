use super::ramanujan_primes::{SMALL, count_bound, nth_bound};

/// Return a lower bound on the number of Ramanujan primes at most `n`.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `ramanujan_prime_count_lower`.
/// It inverts the upper bound on the nth Ramanujan prime.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_prime_count_lower(1000), 71);
/// ```
pub fn ramanujan_prime_count_lower(n: u64) -> u64 {
    if n <= 241 {
        return SMALL.partition_point(|&p| p <= n) as u64;
    }
    let mut lo = count_bound(n, false) / 3;
    let mut hi = count_bound(n, true) / 2;
    if nth_bound(lo, true) > n || nth_bound(hi, true) < n {
        return lo;
    }
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        let value = nth_bound(mid, true);
        if value == n {
            return mid;
        }
        if value < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}
