/// Return each prime factor with its exponent, in ascending order.
///
/// Port of MPU 0.75 `factor.c::factorintp` (`factor.h::factorint`).
/// One returns an empty list. Zero returns `[(0, 1)]`.
///
/// ```
/// use math_prime_util::factor_exp;
/// assert_eq!(factor_exp(360), vec![(2, 3), (3, 2), (5, 1)]);
/// ```
pub fn factor_exp(n: u64) -> Vec<(u64, u32)> {
    let mut pairs: Vec<(u64, u32)> = Vec::new();
    for p in super::factor(n) {
        if let Some((last, exponent)) = pairs.last_mut() {
            if *last == p {
                *exponent += 1;
                continue;
            }
        }
        pairs.push((p, 1));
    }
    pairs
}
