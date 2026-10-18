/// Return 2 for a proven prime, or 0 for a composite.
///
/// Port of `reference/mpu/XS.xs::is_prime` (the `is_provable_prime` alias).
/// All inputs fit MPU's native certificate bound.
///
/// ```
/// assert_eq!(math_prime_util::is_provable_prime(101), 2);
/// ```
pub fn is_provable_prime(n: u64) -> u8 {
    crate::is_prime(n)
}
