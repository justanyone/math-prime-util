//! Sum of k-powerfree integers.
//! Port of MPU 0.75 `powerfree.c`, `powerfree_sum`.

#![allow(clippy::cast_possible_truncation)] // Sieve indexes stay inside the u64 root.

use crate::rootint;

/// Möbius values through `limit`, inclusive.
/// Same linear sieve as `powerfree.c` uses through its segment Möbius.
fn mobius(limit: u64) -> Vec<i8> {
    let size = usize::try_from(limit).expect("sieve size fits usize");
    let mut mu = vec![1_i8; size + 1];
    let mut composite = vec![false; size + 1];
    mu[0] = 0;
    for p in 2..=size {
        if !composite[p] {
            for j in (p..=size).step_by(p) {
                mu[j] = -mu[j];
                composite[j] = true;
            }
            if p <= size / p {
                for j in (p * p..=size).step_by(p * p) {
                    mu[j] = 0;
                }
            }
        }
    }
    mu
}

/// Triangular number `T(n) = n*(n+1)/2`.
/// Port of `powerfree.c` `T`.
fn triangular(n: u64) -> u128 {
    u128::from(n) * (u128::from(n) + 1) / 2
}

/// Sum the positive `k`-powerfree integers up to `n`.
///
/// Port of MPU 0.75 `powerfree.c`, `powerfree_sum`.
/// The sum uses `u128`, so a value past the C word limit stays exact.
/// For `k < 2` the result is 1 when `n >= 1`, and 0 otherwise.
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::powerfree_sum(10, 2), 34);
/// ```
#[must_use]
pub fn powerfree_sum(n: u64, k: u32) -> u128 {
    if k < 2 {
        return u128::from(n >= 1);
    }
    let mu = mobius(rootint(n, k));
    let mut sum = triangular(n);
    let mut subtract = 0_u128;
    for (i, &m) in mu.iter().enumerate().skip(2) {
        if m != 0 {
            let ik = (i as u64).pow(k);
            let term = u128::from(ik) * triangular(n / ik);
            if m > 0 {
                sum += term;
            } else {
                subtract += term;
            }
        }
    }
    sum - subtract
}
