/// Remove all exact factors of `k` from `n`.
///
/// Port of `XS.xs`'s `remove_factors` and `util.c`'s
/// `valuation_remainder` in MPU 0.75. Returns `None` if `n` is zero.
///
/// # Panics
/// Panics if `k` is less than two.
///
/// # Examples
/// ```
/// use math_prime_util::remove_factors;
/// assert_eq!(remove_factors(72, 12), Some(6));
/// assert_eq!(remove_factors(0, 2), None);
/// ```
pub fn remove_factors(n: u64, k: u64) -> Option<u64> {
    super::remove_factors_exp(n, k).map(|(remainder, _)| remainder)
}
