//! Sum of powerfree parts.
//! Port of MPU 0.75 `powerfree.c`, `powerfree_part_sum` and `fprod`.

use super::powerful::factor_exponents;
use crate::rootint;

/// Triangular number `T(n) = n*(n+1)/2`.
/// Port of `powerfree.c` `T`.
const fn triangular(n: u64) -> u128 {
    u128::from(n) * (u128::from(n) + 1) / 2
}

/// Sum `powerfree_part(i, k)` for `i` from 1 to `n`.
///
/// Port of MPU 0.75 `powerfree.c`, `powerfree_part_sum` and `fprod`.
/// Each term is `T(n / j^k)` times the product of `(1 - p^k)` over the
/// distinct primes `p` that divide `j`. The sum uses `u128`.
/// For `k < 2` the result is 1 when `n >= 1`, and 0 otherwise.
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::powerfree_part_sum(10, 2), 38);
/// ```
#[must_use]
pub fn powerfree_part_sum(n: u64, k: u32) -> u128 {
    if k < 2 || n <= 1 {
        return u128::from(n >= 1);
    }
    let mut sum = triangular(n);
    let mut subtract = 0_u128;
    for j in 2..=rootint(n, k) {
        let factors = factor_exponents(j);
        let product: u128 = factors
            .iter()
            .map(|&(p, _)| u128::from(p.pow(k)) - 1)
            .product();
        let term = product * triangular(n / j.pow(k));
        if factors.len() % 2 == 0 {
            sum += term;
        } else {
            subtract += term;
        }
    }
    sum - subtract
}
