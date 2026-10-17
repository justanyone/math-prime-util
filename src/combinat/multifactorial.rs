/// Returns `n * (n-k) * (n-2k) * ...`, or `None` on overflow.
///
/// Port of `multifactorial` in MPU 0.75 `util.c`.
/// The positive-step check follows `multifactorial` in `XS.xs`.
/// The empty product at `n == 0` is one.
///
/// # Panics
///
/// Panics if `k == 0`.
///
/// ```
/// use math_prime_util::multifactorial;
/// assert_eq!(multifactorial(9, 2), Some(945));
/// assert_eq!(multifactorial(21, 1), None);
/// ```
pub fn multifactorial(n: u64, k: u64) -> Option<u64> {
    assert!(k > 0, "multifactorial: k must be positive");
    let mut result = 1_u64;
    let mut i = n;
    while i > 0 {
        if result > u64::MAX / i {
            return None;
        }
        result *= i;
        if i <= k {
            break;
        }
        i -= k;
    }
    Some(result)
}
