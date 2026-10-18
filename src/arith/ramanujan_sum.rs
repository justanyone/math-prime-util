use crate::gcd;

/// Return Ramanujan's sum `c_k(n)`.
///
/// Port of MPU 0.75 `XS.xs::ramanujan_sum`.
/// A zero argument gives zero.
/// The value is `moebius(k / gcd(k, n)) * euler_phi(k) / euler_phi(k / gcd(k, n))`.
/// The product keeps the low 64 bits, as `XSRETURN_IV` does.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_sum(12, 36), 4);
/// assert_eq!(math_prime_util::ramanujan_sum(0, 5), 0);
/// ```
pub fn ramanujan_sum(k: u64, n: u64) -> i64 {
    if k == 0 || n == 0 {
        return 0;
    }
    let g = k / gcd(k, n);
    let m = i64::from(super::moebius(g));
    if m == 0 || k == g {
        return m;
    }
    let ratio = super::euler_phi(k) / super::euler_phi(g);
    m.wrapping_mul(i64::from_ne_bytes(ratio.to_ne_bytes()))
}
