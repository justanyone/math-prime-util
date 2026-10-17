use super::factors;

/// Return the Jordan totient, or `None` on overflow.
/// Port of `totients.c::jordan_totient`; checked powers also cover the Perl fallback.
/// ```
/// assert_eq!(math_prime_util::jordan_totient(2, 12), Some(96));
/// ```
pub fn jordan_totient(k: u64, n: u64) -> Option<u64> {
    if k == 0 || n <= 1 {
        return Some(u64::from(n == 1));
    }
    if n == 2 {
        // 2^k - 1 is computed in u128 so that k = 64 (result u64::MAX) does not overflow.
        let v = 2_u128.checked_pow(u32::try_from(k).ok()?)? - 1;
        return u64::try_from(v).ok();
    }
    let k = u32::try_from(k).ok()?;
    let mut v = 1_u64;
    for (p, e) in factors(n) {
        let pk = p.checked_pow(k)?;
        v = v.checked_mul(pk - 1)?;
        for _ in 1..e {
            v = v.checked_mul(pk)?;
        }
    }
    Some(v)
}

