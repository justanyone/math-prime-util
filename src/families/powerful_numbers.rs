//! Powerful integers from MPU 0.75 `powerful.c`.
#![allow(clippy::cast_possible_truncation)] // Indices fit the allocated sieve.
use super::{is_powerful, powerful::squarefree_flags};
use crate::{gcd, rootint};

fn generate(lo: u64, hi: u64, k: u32, m: u64, r: u32, out: &mut Vec<u64>) {
    let end = rootint(hi / m, r);
    if r == k {
        let lower = lo.div_ceil(m);
        let mut beg = rootint(lower, r);
        if beg.pow(r) < lower {
            beg += 1;
        }
        out.extend((beg..=end).map(|v| m * v.pow(r)));
    } else {
        let flags = squarefree_flags(end);
        for v in 1..=end {
            if flags[v as usize] && gcd(m, v) == 1 {
                generate(lo, hi, k, m * v.pow(r), r - 1, out);
            }
        }
    }
}

/// List the `k`-powerful integers in the inclusive interval.
/// Port of `powerful.c` / `powerful_numbers_range` and `_pcg`.
/// ```
/// assert_eq!(math_prime_util::powerful_numbers(1, 10, 2), vec![1,4,8,9]);
/// ```
pub fn powerful_numbers(lo: u64, hi: u64, k: u32) -> Vec<u64> {
    let lo = lo.max(1);
    if lo > hi {
        return vec![];
    }
    if k <= 1 {
        return (lo..=hi).collect();
    }
    if k >= 64 || k > hi.ilog2() {
        return if lo == 1 { vec![1] } else { vec![] };
    }
    let threshold = if lo < 500_000 {
        30
    } else if lo < 400_000_000 {
        160
    } else {
        600
    };
    if hi - lo < threshold * if k <= 2 { 1 } else { 4 } {
        return (lo..=hi).filter(|&n| is_powerful(n, k)).collect();
    }
    let mut result = Vec::new();
    generate(lo, hi, k, 1, 2 * k - 1, &mut result);
    result.sort_unstable();
    result
}
