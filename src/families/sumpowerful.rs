//! Powerful integers from MPU 0.75 `powerful.c`.
#![allow(clippy::cast_possible_truncation)] // Indices fit the allocated sieve.
use super::powerful::squarefree_flags;
use crate::{gcd, rootint};

fn power_sum(n: u64, k: u32) -> u128 {
    let wide = u128::from(n);
    match k {
        2 => wide * (wide + 1) * (2 * wide + 1) / 6,
        3 => (wide * (wide + 1) / 2).pow(2),
        _ => (1..=n).map(|i| u128::from(i).pow(k)).sum(),
    }
}

fn sum_rec(n: u64, k: u32, flags: &[bool], m: u64, r: u32) -> u128 {
    let limit = rootint(n / m, r);
    if r == k {
        return u128::from(m) * power_sum(limit, r);
    }
    (1..=limit)
        .filter(|&i| flags[i as usize] && gcd(m, i) == 1)
        .map(|i| sum_rec(n, k, flags, m * i.pow(r), r - 1))
        .sum()
}

/// Sum the `k`-powerful integers up to `n` with wide arithmetic.
/// Port of `powerful.c` / `sumpowerful` and `_sumpowerful`.
/// ```
/// assert_eq!(math_prime_util::sumpowerful(10, 2), 22);
/// ```
pub fn sumpowerful(n: u64, k: u32) -> u128 {
    if n == 0 {
        return 0;
    }
    if k >= 64 {
        return 1;
    }
    if k <= 1 {
        return u128::from(n) * (u128::from(n) + 1) / 2;
    }
    let flags = squarefree_flags(rootint(n, k + 1));
    sum_rec(n, k, &flags, 1, 2 * k - 1)
}
