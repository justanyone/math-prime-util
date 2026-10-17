use super::random_proven_prime_with_cert;

/// Returns a proven prime with exactly `bits` bits.
///
/// Port of MPU `lib/Math/Prime/Util/RandomPrimes.pm`, `random_proven_prime`.
/// The native path uses `random_prime.c`, `random_nbit_prime`. Native
/// primality testing proves the result and its `Type Small` certificate.
/// Sizes above 64 return `None` until the bigint phase.
///
/// # Panics
/// Panics if `bits < 2`, or if an unseeded thread cannot read OS entropy.
///
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::random_proven_prime(10), Some(619));
/// ```
pub fn random_proven_prime(bits: u32) -> Option<u64> {
    random_proven_prime_with_cert(bits).map(|(prime, _)| prime)
}
