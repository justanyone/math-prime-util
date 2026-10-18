//! Powerful integers, from MPU 0.75 `powerful.c`.
#![allow(clippy::cast_possible_truncation)] // All indices are bounded by allocated sieve sizes.
use crate::{is_prime, rootint};

pub(super) fn squarefree_flags(limit: u64) -> Vec<bool> {
    let size = usize::try_from(limit).expect("sieve size fits usize");
    let mut flags = vec![true; size + 1];
    flags[0] = false;
    for p in 2..=rootint(limit, 2) {
        if flags[p as usize] {
            let square = (p * p) as usize;
            for j in (square..=size).step_by(square) {
                flags[j] = false;
            }
        }
    }
    flags
}

pub(super) fn factor_exponents(mut n: u64) -> Vec<(u64, u32)> {
    let mut result = Vec::new();
    let mut p = 2;
    while p <= n / p {
        if is_prime(n) != 0 {
            break;
        }
        if n % p == 0 {
            let mut e = 0;
            while n % p == 0 {
                n /= p;
                e += 1;
            }
            result.push((p, e));
        }
        p = if p == 2 { 3 } else { p + 2 };
    }
    if n > 1 {
        result.push((n, 1));
    }
    result
}
