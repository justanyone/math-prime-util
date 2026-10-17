use super::random_maurer_prime;

/// Returns a proven prime and its MPU text certificate.
///
/// Port of MPU `lib/Math/Prime/Util/RandomPrimes.pm`,
/// `random_proven_prime_with_cert`, via `random_maurer_prime_with_cert`.
/// Native sizes use `random_prime.c`, `random_nbit_prime`, and
/// `PrimalityProving.pm`, `_small_cert`. A `Type Small` certificate uses
/// deterministic primality testing through 64 bits. Sizes above 64 return
/// `None` until the bigint phase.
///
/// # Panics
/// Panics if `bits < 2`, or if an unseeded thread cannot read OS entropy.
///
/// ```
/// math_prime_util::srand(15);
/// let (p, cert) = math_prime_util::random_proven_prime_with_cert(10).unwrap();
/// assert_eq!(p, 619);
/// assert!(cert.ends_with("Type Small\nN 619\n"));
/// ```
pub fn random_proven_prime_with_cert(bits: u32) -> Option<(u64, String)> {
    let prime = random_maurer_prime(bits)?;
    let certificate = format!(
        "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN {prime}\n\nType Small\nN {prime}\n"
    );
    Some((prime, certificate))
}
