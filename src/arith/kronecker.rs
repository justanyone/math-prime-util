fn kronecker_unsigned(a: u64, mut b: u64) -> i32 {
    if b == 0 {
        return i32::from(a == 1);
    }
    let mut s = 1;
    let r = b.trailing_zeros();
    if r > 0 {
        if a % 2 == 0 {
            return 0;
        }
        if r % 2 == 1 && matches!(a % 8, 3 | 5) {
            s = -s;
        }
        b >>= r;
    }
    kronecker_odd(a, b, s)
}
fn kronecker_odd(mut a: u64, mut b: u64, mut s: i32) -> i32 {
    while a != 0 {
        let r = a.trailing_zeros();
        if r % 2 == 1 && matches!(b % 8, 3 | 5) {
            s = -s;
        }
        a >>= r;
        if a & b & 2 != 0 {
            s = -s;
        }
        (a, b) = (b % a, a);
    }
    if b == 1 { s } else { 0 }
}

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

