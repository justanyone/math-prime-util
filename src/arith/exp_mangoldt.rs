/// Return the prime base of a prime power, or one otherwise.
///
/// Port of MPU 0.75 `util.c::exp_mangoldt`, which calls
/// `prime_powers.c::prime_power`.
///
/// ```
/// assert_eq!(math_prime_util::exp_mangoldt(27), 3);
/// assert_eq!(math_prime_util::exp_mangoldt(12), 1);
/// ```
pub fn exp_mangoldt(n: u64) -> u64 {
    let exponent = crate::is_prime_power(n);
    if exponent == 0 { 1 } else { crate::rootint(n, exponent) }
}
