use super::prime_certificate;

/// Return the primality status and its MPU certificate.
///
/// Port of `reference/mpu/lib/Math/Prime/Util/PP.pm::is_provable_prime_with_cert`
/// and `PrimalityProving.pm::_small_cert` for native integers.
/// A composite has status 0 and an empty certificate. A prime has status 2.
///
/// ```
/// let (status, cert) = math_prime_util::is_provable_prime_with_cert(101);
/// assert_eq!(status, 2);
/// assert!(math_prime_util::verify_prime(&cert).unwrap());
/// ```
pub fn is_provable_prime_with_cert(n: u64) -> (u8, String) {
    let cert = prime_certificate(n);
    (if cert.is_empty() { 0 } else { 2 }, cert)
}
