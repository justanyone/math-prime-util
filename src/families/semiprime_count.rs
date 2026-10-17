use crate::{prime_count, primes_in, sqrtint};

/// Count the semiprimes less than or equal to `n`.
///
/// Port of `semi_primes.c`, `semiprime_count`, in MPU 0.75.
/// Sum `prime_count(n / p) - prime_count(p - 1)` for primes through `sqrt(n)`.
///
/// ```
/// use math_prime_util::semiprime_count;
/// assert_eq!(semiprime_count(100), 34);
/// ```
pub fn semiprime_count(n: u64) -> u64 {
    primes_in(2..=sqrtint(n))
        .enumerate()
        .map(|(i, p)| prime_count(n / p) - i as u64)
        .sum()
}
