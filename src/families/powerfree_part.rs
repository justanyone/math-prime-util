//! Powerfree part of one integer.
//! Port of MPU 0.75 `powerfree.c`, `powerfree_part`.

use super::powerful::factor_exponents;

/// Remove every complete `k`th-power factor from `n`.
///
/// Port of MPU 0.75 `powerfree.c`, `powerfree_part`.
/// For `k < 2` the result is 1 when `n` is 1, and 0 otherwise.
/// For `k >= 64` every `u64` is `k`-powerfree, so the result is `n`.
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::powerfree_part(48, 2), 3);
/// ```
#[must_use]
pub fn powerfree_part(n: u64, k: u32) -> u64 {
    if k < 2 || n <= 1 {
        return u64::from(n == 1);
    }
    if k >= 64 {
        return n;
    }
    factor_exponents(n)
        .iter()
        .map(|&(p, e)| p.pow(e % k))
        .product()
}
