/// Counts distinct prime factors. Zero has one factor; one has none.
///
/// Port of `prime_omega` in MPU `factor.c`.
///
/// ```
/// assert_eq!(math_prime_util::prime_omega(360), 3);
/// ```
pub fn prime_omega(n: u64) -> u32 {
    super::prime_bigomega::exponents(n)
        .iter()
        .fold(0, |count, _| count + 1)
}
