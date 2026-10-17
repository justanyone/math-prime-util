/// Return whether `n` is a perfect number.
///
/// Port of `is_perfect_number` in MPU 0.75 `util.c`. It checks the
/// even perfect number form. No odd perfect number fits in `u64`.
///
/// ```
/// assert!(math_prime_util::is_perfect_number(28));
/// assert!(!math_prime_util::is_perfect_number(27));
/// ```
pub fn is_perfect_number(n: u64) -> bool {
    if n == 0 || n % 2 != 0 {
        return false;
    }
    let v = n.trailing_zeros();
    let m = n >> v;
    if m & (m + 1) != 0 || m >> v != 1 {
        return false;
    }
    // util.c calls is_mersenne_prime(v + 1). Here m = 2^(v + 1) - 1.
    crate::is_prime(m) != 0
}
