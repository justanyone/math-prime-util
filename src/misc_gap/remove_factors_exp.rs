/// Remove all exact factors of `k` from `n`. Return the remainder and count.
///
/// Port of `util.c`'s `valuation_remainder`, with the zero and argument
/// checks from `XS.xs`'s `remove_factors` in MPU 0.75.
/// Returns `None` if `n` is zero. Composite factors are allowed.
///
/// # Panics
/// Panics if `k` is less than two.
///
/// # Examples
/// ```
/// use math_prime_util::remove_factors_exp;
/// assert_eq!(remove_factors_exp(72, 6), Some((2, 2)));
/// assert_eq!(remove_factors_exp(0, 6), None);
/// ```
pub fn remove_factors_exp(mut n: u64, k: u64) -> Option<(u64, u32)> {
    assert!(k > 1, "remove_factors_exp: k must be > 1");
    if n == 0 {
        return None;
    }
    if k == 2 {
        let e = n.trailing_zeros();
        return Some((n >> e, e));
    }
    let mut e = 0;
    while n % k == 0 {
        n /= k;
        e += 1;
    }
    Some((n, e))
}
