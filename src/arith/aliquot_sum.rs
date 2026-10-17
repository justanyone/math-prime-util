use super::divisor_sum::sigma;

/// Return the sum of the proper divisors of n.
/// Return `None` if the result exceeds u64.
///
/// Port of MPU 0.75 `factor.c::aliquot_sum`.
///
/// ```
/// assert_eq!(math_prime_util::aliquot_sum(12), Some(16));
/// ```
pub fn aliquot_sum(n: u64) -> Option<u64> {
    if n <= 1 {
        return Some(0);
    }
    u64::try_from(sigma(n, 1)? - u128::from(n)).ok()
}

