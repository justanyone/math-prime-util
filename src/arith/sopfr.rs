/// Return the sum of prime factors with multiplicity.
///
/// Port of MPU 0.75 `factor.c::sopfr`.
///
/// ```
/// assert_eq!(math_prime_util::sopfr(12), 7);
/// ```
pub fn sopfr(n: u64) -> u64 {
    if n <= 5 {
        return n - u64::from(n == 1);
    }
    super::factor_powers(n).iter().map(|&(p, e)| p * u64::from(e)).sum()
}
