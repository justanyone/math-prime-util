use super::factors;

/// Return the Jordan totient, or `None` on overflow.
/// Port of `totients.c::jordan_totient`; checked powers also cover the Perl fallback.
/// ```
/// assert_eq!(math_prime_util::jordan_totient(2, 12), Some(96));
/// ```
pub fn jordan_totient(k: u64, mut n: u64) -> Option<u64> {
    if k == 0 || n <= 1 {
        return Some(u64::from(n == 1));
    }
    let k = u32::try_from(k).ok()?;
    // MPU totients.c::jordan_totient strips powers of two before factoring.
    // Checked u128 powers also handle the Perl fallback beyond the C bounds.
    let two_k = 2_u128.checked_pow(k)?;
    let mut v = 1_u64;
    while n % 4 == 0 {
        n >>= 1;
        v = u64::try_from(u128::from(v).checked_mul(two_k)?).ok()?;
    }
    if n % 2 == 0 {
        n >>= 1;
        v = u64::try_from(u128::from(v).checked_mul(two_k - 1)?).ok()?;
    }
    for (p, e) in factors(n) {
        let pk = p.checked_pow(k)?;
        v = v.checked_mul(pk - 1)?;
        for _ in 1..e {
            v = v.checked_mul(pk)?;
        }
    }
    Some(v)
}
