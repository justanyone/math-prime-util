/// Returns a Catalan number, or `None` on overflow.
///
/// Port of `catalan_number` in MPU 0.75 `util.c`: compute the central
/// binomial coefficient and divide by `n + 1`. Use a wider intermediate
/// so that every Catalan number that fits in `u64` is returned.
///
/// ```
/// assert_eq!(math_prime_util::catalan_number(10), Some(16796));
/// ```
pub fn catalan_number(n: u64) -> Option<u64> {
    if n > 36 {
        return None;
    }
    let mut r = 1_u128;
    for d in 1..=n {
        r = r * u128::from(2 * n - d + 1) / u128::from(d);
    }
    u64::try_from(r / u128::from(n + 1)).ok()
}
