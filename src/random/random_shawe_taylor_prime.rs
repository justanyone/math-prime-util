/// Returns a prime made by the Shawe–Taylor construction.
/// Ports `lib/Math/Prime/Util/RandomPrimes.pm`: `random_shawe_taylor_prime`
/// and `XS.xs`: `random_nbit_prime` for its native dispatch.
/// Returns `None` above 64 bits until bigint support is available.
/// # Panics
/// Panics if `bits < 2`, or if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// let p = math_prime_util::random_shawe_taylor_prime(16).unwrap();
/// assert_eq!(math_prime_util::is_prime(p), 2);
/// ```
pub fn random_shawe_taylor_prime(bits: u32) -> Option<u64> {
    assert!(bits >= 2, "random_shawe_taylor_prime, bits must be >= 2");
    (bits <= 64).then(|| super::random_maurer_prime_with_cert::nbit_prime(bits))
}
