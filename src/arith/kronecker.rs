use super::{kronecker_odd, kronecker_unsigned};

/// Return the signed Kronecker symbol (a/b).
///
/// Port of MPU 0.75 `util.c::kronecker_ss, kronecker_su, kronecker_uu_sign`.
///
/// ```
/// assert_eq!(math_prime_util::kronecker(-1, 3), -1);
/// ```
pub fn kronecker(a: i64, b: i64) -> i32 {
    let mut s = if a < 0 && b < 0 { -1 } else { 1 };
    let mut b = b.unsigned_abs();
    if a >= 0 {
        return s * kronecker_unsigned(a.unsigned_abs(), b);
    }
    if b == 0 {
        return i32::from(a == -1);
    }
    let r = b.trailing_zeros();
    if r > 0 {
        if a % 2 == 0 {
            return 0;
        }
        if r % 2 == 1 && matches!(a.rem_euclid(8), 3 | 5) {
            s = -s;
        }
        b >>= r;
    }
    let rem = a.unsigned_abs() % b;
    kronecker_odd(if rem == 0 { 0 } else { b - rem }, b, s)
}
