#![allow(clippy::cast_possible_truncation)] // Sieve indices fit the native root.
use super::powerfree::{mobius, squarefree_count};
use crate::rootint;

/// Count positive `k`-powerfree integers up to `n`.
/// Port of `powerfree.c` / `powerfree_count` and `squarefree_count`.
/// ```
/// assert_eq!(math_prime_util::powerfree_count(10, 2), 7);
/// ```
#[allow(clippy::cast_sign_loss)] // The Mobius identity is a count in 0..=n.
pub fn powerfree_count(n: u64, k: u32) -> u64 {
    if k < 2 {
        return u64::from(n >= 1);
    }
    if n < 4 || k >= 64 {
        return n;
    }
    if k == 2 {
        return squarefree_count(n);
    }
    let mu = mobius(rootint(n, k));
    let mut count = i128::from(n);
    for (i, &m) in mu.iter().enumerate().skip(2) {
        if m != 0 {
            count += i128::from(m) * i128::from(n / (i as u64).pow(k));
        }
    }
    count as u64
}

