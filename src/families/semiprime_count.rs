#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use crate::{next_prime, prev_prime, prime_count, primes, primes_in, sqrtint};

/// Count the semiprimes less than or equal to `n`.
///
/// Port of `semi_primes.c`, `semiprime_count`, in MPU 0.75.
/// Sum `prime_count(n / p) - prime_count(p - 1)` for primes through `sqrt(n)`.
/// Cache small quotients in a prime list, as in the reference.
///
/// ```
/// use math_prime_util::semiprime_count;
/// assert_eq!(semiprime_count(100), 34);
/// ```
pub fn semiprime_count(n: u64) -> u64 {
    let root = sqrtint(n);
    let xlim = if n > 1_000_000 {
        (n as f64).powf(0.70) as u64
    } else {
        0
    };
    let mut cached = Vec::new();
    let mut beginning = 0;
    let mut offset = 0;
    let mut sum = 0;
    for (i, p) in primes_in(2..=root).enumerate() {
        let quotient = n / p;
        let count = if p >= 7 && quotient < xlim {
            if cached.is_empty() || quotient < beginning {
                let lower = (n / root).max(quotient.saturating_sub(200_000_000));
                beginning = prev_prime(lower).unwrap_or(2);
                let end = next_prime(quotient).unwrap_or(quotient);
                offset = prime_count(beginning);
                cached = primes(beginning, end);
            }
            offset + cached.partition_point(|&v| v <= quotient) as u64 - 1
        } else {
            prime_count(quotient)
        };
        sum += count - i as u64;
    }
    sum
}
