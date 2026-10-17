/// Returns a Gordon strong prime, when the size is supported.
///
/// Port of MPU `lib/Math/Prime/Util/RandomPrimes.pm`, `random_strong_prime`.
/// Gordon's method requires at least 128 bits. Sizes above 64 return `None`
/// until the bigint phase, so no valid size has a native result.
///
/// # Panics
/// Panics if `bits < 128`, as in MPU.
///
/// ```
/// assert_eq!(math_prime_util::random_strong_prime(128), None);
/// ```
pub fn random_strong_prime(bits: u32) -> Option<u64> {
    assert!(bits >= 128, "random_strong_prime, bits must be >= 128");
    None
}
