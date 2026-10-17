/// Returns a prime made by the Shawe–Taylor construction.
/// Ports `lib/Math/Prime/Util/RandomPrimes.pm`: `random_shawe_taylor_prime`
/// and `_ST_Random_prime`. Returns `None` above 64 bits until bigint support.
/// # Errors
/// Returns an error if the reference search limit is reached.
/// # Panics
/// Panics if `bits < 2`, or if an unseeded thread cannot read OS entropy.
/// ```
/// math_prime_util::srand(15);
/// let p = math_prime_util::random_shawe_taylor_prime(16).unwrap().unwrap();
/// assert_eq!(math_prime_util::is_prime(p), 2);
/// ```
pub fn random_shawe_taylor_prime(bits: u32) -> Result<Option<u64>, &'static str> {
    super::random_shawe_taylor_prime_with_cert(bits).map(|result| result.map(|(p, _)| p))
}
