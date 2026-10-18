use super::factor_powers;

/// Test whether `n` is a fundamental discriminant.
///
/// Port of MPU 0.75 `util.c::is_fundamental`.
///
/// ```
/// assert!(math_prime_util::is_fundamental(-4));
/// assert!(!math_prime_util::is_fundamental(4));
/// ```
pub fn is_fundamental(n: i64) -> bool {
    let r = n.rem_euclid(16);
    let m = n.unsigned_abs();
    let square_free = |x| x != 0 && factor_powers(x).iter().all(|&(_, e)| e == 1);
    if r != 0 && r % 4 == 0 && r != 4 {
        square_free(m >> 2)
    } else {
        r % 4 == 1 && square_free(m)
    }
}
