use super::{factor_powers, kronecker_unsigned};

/// Test whether `a` is a square modulo `n`.
///
/// Return false for modulus zero, as in the MPU wrapper.
///
/// Port of MPU 0.75 `util.c::is_qr` and `rootmod.c::_sqrtmod_composite`.
///
/// ```
/// assert!(math_prime_util::is_qr(2, 7));
/// assert!(!math_prime_util::is_qr(1, 0));
/// ```
pub fn is_qr(a: u64, n: u64) -> bool {
    if n == 0 {
        return false;
    }
    if n <= 2 || a % n <= 1 {
        return true;
    }
    let a = a % n;
    factor_powers(n).iter().all(|&(p, e)| {
        let mut unit = a;
        let mut v = 0;
        while unit % p == 0 && v < e {
            unit /= p;
            v += 1;
        }
        if v == e {
            return true;
        }
        if v % 2 == 1 {
            return false;
        }
        if p != 2 {
            return kronecker_unsigned(unit, p) == 1;
        }
        match e - v {
            1 => true,
            2 => unit % 4 == 1,
            _ => unit % 8 == 1,
        }
    })
}
