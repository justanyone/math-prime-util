/// Return MPU's lower bound for the count of `k`-almost primes at most `n`.
///
/// Port of MPU 0.75 `almost_primes.c`: `almost_prime_count_lower` and
/// `_almost_prime_count_bounds`. The reference uses empirical bounds for high `k`.
/// ```
/// use math_prime_util::almost_prime_count_lower;
/// assert_eq!(almost_prime_count_lower(2, 100), 34);
/// ```
pub fn almost_prime_count_lower(k: u64, n: u64) -> u64 {
    if k == 2 && n < 256 { crate::semiprime_count(n) }
    else { super::almost_prime_count::bounds(k,n).0 }
}
