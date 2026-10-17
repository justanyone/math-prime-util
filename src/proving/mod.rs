//! Native primality proofs and MPU certificates.

mod aks;
mod certificate;
pub use aks::is_aks_prime;
pub use certificate::verify_prime;

/// Return 2 for a proven prime, or 0 for a composite.
///
/// Port of `PrimalityProving.pm::_small_cert` and the native primality path.
/// All inputs fit MPU's native certificate bound.
///
/// ```
/// assert_eq!(math_prime_util::is_provable_prime(101), 2);
/// ```
pub fn is_provable_prime(n: u64) -> u8 {
    crate::is_prime(n)
}

/// Return an MPU certificate, or an empty string for a composite.
///
/// Port of `PrimalityProving.pm::_small_cert` for native integers.
///
/// ```
/// let cert = math_prime_util::prime_certificate(101);
/// assert!(math_prime_util::verify_prime(&cert).unwrap());
/// ```
pub fn prime_certificate(n: u64) -> String {
    if is_provable_prime(n) == 0 {
        return String::new();
    }
    format!("[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN {n}\n\nType Small\nN {n}\n")
}

/// Return the primality status and its MPU certificate.
///
/// Port of `PrimalityProving.pm::_small_cert` for native integers.
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
