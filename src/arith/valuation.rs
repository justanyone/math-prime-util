/// Return the exponent of `k` in `n`.
///
/// Return `None` when `n` is zero.
///
/// Port of MPU 0.75 `util.c::valuation` and the checks in `XS.xs::valuation`.
/// Base 2 uses the trailing-zero count. Every other base uses the power loop.
///
/// # Panics
///
/// Panics when `k` is less than two.
///
/// ```
/// assert_eq!(math_prime_util::valuation(72, 3), Some(2));
/// assert_eq!(math_prime_util::valuation(0, 2), None);
/// ```
pub fn valuation(n: u64, k: u64) -> Option<u64> {
    assert!(k >= 2, "valuation: k must be > 1");
    if n == 0 {
        return None;
    }
    if n < 2 {
        return Some(0);
    }
    if k == 2 {
        return Some(u64::from(n.trailing_zeros()));
    }
    let mut v = 0_u64;
    let mut kpower = k;
    while n % kpower == 0 {
        v += 1;
        if kpower > n / k {
            break;
        }
        kpower *= k;
    }
    Some(v)
}
