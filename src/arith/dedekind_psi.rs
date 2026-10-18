use super::factors;

/// Return Dedekind's psi, or `None` on overflow.
/// Port of `reference/mpu/totients.c::dedekind_psi`.
/// ```
/// assert_eq!(math_prime_util::dedekind_psi(12), Some(24));
/// ```
pub fn dedekind_psi(n: u64) -> Option<u64> {
    if n < 4 {
        return Some(n + u64::from(n > 1));
    }
    let mut v = 1_u64;
    for (p, e) in factors(n) {
        v = v
            .checked_mul(p.checked_add(1)?)?
            .checked_mul(p.pow(e - 1))?;
    }
    Some(v)
}
