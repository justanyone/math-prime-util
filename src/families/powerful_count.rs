//! Powerful integers from MPU 0.75 `powerful.c`.
#![allow(clippy::cast_possible_truncation)] // Indices fit the allocated sieve.
use super::powerful::squarefree_flags;
use crate::{gcd, rootint};

fn count_rec(n: u64, k: u32, flags: &[bool], m: u64, r: u32) -> u64 {
    let limit = rootint(n / m, r);
    if r <= k {
        return limit;
    }
    (1..=limit)
        .filter(|&i| flags[i as usize] && gcd(m, i) == 1)
        .map(|i| {
            let quotient = n / (m * i.pow(r));
            if r - 1 == k {
                rootint(quotient, k)
            } else {
                count_rec(n, k, flags, m * i.pow(r), r - 1)
            }
        })
        .sum()
}

/// Count the positive `k`-powerful integers up to `n`.
/// Port of `powerful.c` / `powerful_count` and `_pcr`.
/// ```
/// assert_eq!(math_prime_util::powerful_count(10, 2), 4);
/// ```
pub fn powerful_count(n: u64, k: u32) -> u64 {
    if k <= 1 || n <= 1 {
        return n;
    }
    if k >= 64 {
        return 1;
    }
    let flags = squarefree_flags(rootint(n, k + 1));
    if k == 2 {
        return (1..=rootint(n, 3))
            .filter(|&i| flags[i as usize])
            .map(|i| rootint(n / (i * i * i), 2))
            .sum();
    }
    let r = 2 * k - 1;
    (1..=rootint(n, r))
        .filter(|&i| flags[i as usize])
        .map(|i| count_rec(n, k, &flags, i.pow(r), r - 1))
        .sum()
}
