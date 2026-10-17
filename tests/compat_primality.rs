//! Native proving checks against MPU 0.75.
mod common;
use math_prime_util::*;
#[test]
fn is_aks_prime_oracle() {
    common::each("primality", "is_aks_prime", |c| {
        // Signed inputs such as -1 and -2 are outside the u64 API.
        if let Ok(n) = c.args[0].parse::<u64>() { c.check(Some(u8::from(is_aks_prime(n)))); }
    });
}
#[test]
fn is_provable_prime_oracle() {
    common::each("primality", "is_provable_prime", |c| {
        // Signed and bigint arguments are outside the native u64 API.
        if let Ok(n) = c.args[0].parse::<u64>() { c.check(Some(is_provable_prime(n))); }
    });
}
// The committed primality fixture has no rows for these three APIs.
// The expected text follows MPU PrimalityProving.pm::_small_cert.
#[test]
fn prime_certificate_compat() {
    assert_eq!(prime_certificate(101), "[MPU - Primality Certificate]\nVersion 1.0\n\nProof for:\nN 101\n\nType Small\nN 101\n");
    assert!(prime_certificate(1).is_empty());
}
#[test]
fn is_provable_prime_with_cert_compat() {
    assert_eq!(is_provable_prime_with_cert(15), (0, String::new()));
    assert_eq!(is_provable_prime_with_cert(101), (2, prime_certificate(101)));
}
#[test]
fn verify_prime_compat() {
    for n in [2, 3, 101, 4_294_967_311, u64::MAX-58] { assert_eq!(verify_prime(&prime_certificate(n)), Ok(true)); }
    let header = "[MPU - Primality Certificate]\nVersion 1.0\nProof for:\nN 101\n";
    for proof in ["Type BLS5\nN 101\nQ[1] 5\nA[0] 2\nA[1] 2\n----\n", "Type Lucas\nN 101\nQ[1] 2\nQ[2] 5\nA 2\n"] {
        assert_eq!(verify_prime(&format!("{header}{proof}")), Ok(true));
    }
    for proof in ["Type Small\nN 15", "Type BLS5\nN 101\nA[0] 1\n----", "Type BLS5\nN 101\nQ[1] 4\n----"] {
        assert_eq!(verify_prime(&format!("{header}{proof}")), Ok(false));
    }
    for proof in ["Type ECPP\nN 101", "Type BLS5\nN 101\nN 101", "Type Small\nN 18446744073709551616", "Type BLS5\nN 101\nQ[2] 5\n----"] {
        assert!(verify_prime(&format!("{header}{proof}")).is_err());
    }
    assert_eq!(verify_prime(""), Ok(false));
    assert!(verify_prime("garbage").is_err());
}
