/// Return the sum of distinct prime factors.
///
/// Port of MPU 0.75 `factor.c::sopf`.
///
/// ```
/// assert_eq!(math_prime_util::sopf(12), 5);
/// ```
pub fn sopf(n: u64) -> u64 {
    if n <= 3 {
        return n - u64::from(n == 1);
    }
    super::factor_powers(n).iter().map(|&(p, _)| p).sum()
}
