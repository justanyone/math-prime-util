use super::is_provable_prime;

/// Return an MPU certificate, or an empty string for a composite.
///
/// Port of `reference/mpu/lib/Math/Prime/Util/PrimalityProving.pm::_small_cert` for native integers.
///
/// ```
/// let cert = math_prime_util::prime_certificate(101);
/// assert!(math_prime_util::verify_prime(&cert).unwrap());
/// ```
pub fn prime_certificate(n: u64) -> String {
    if is_provable_prime(n) == 0 {
        return String::new();
    }
    format!(
        "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN {n}\n\nType Small\nN {n}\n"
    )
}
