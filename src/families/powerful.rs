//! Powerful integers, from MPU 0.75 `powerful.c`.
#![allow(clippy::cast_possible_truncation)] // All indices are bounded by allocated sieve sizes.
use crate::{gcd, int::is_power_exp, is_prime, rootint};

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

/// Test if every prime exponent is at least `k`.
/// Port of `powerful.c` / `is_powerful`.
/// ```
/// assert!(math_prime_util::is_powerful(72, 2));
/// ```
pub fn is_powerful(mut n: u64, k: u32) -> bool {
    if n <= 1 {
        return n == 1;
    }
    if k <= 1 {
        return true;
    }
    if n % 2 == 0 {
        if n.trailing_zeros() < k {
            return false;
        }
        n >>= n.trailing_zeros();
    }
    if n == 1 {
        return true;
    }
    if k > 40 {
        return false;
    }
    if is_power_exp(n) >= k {
        return true;
    }
    let mut p = 3_u64;
    while p <= rootint(n, 2 * k) {
        if n % p == 0 {
            let mut exponent = 0;
            while n % p == 0 {
                n /= p;
                exponent += 1;
            }
            if exponent < k {
                return false;
            }
            if n == 1 || is_power_exp(n) >= k {
                return true;
            }
        }
        p += 2;
    }
    false
}

fn count_rec(n: u64, k: u32, flags: &[bool], m: u64, r: u32) -> u64 {
    let limit = rootint(n / m, r);
    if r <= k {
        return limit;
    }
    (1..=limit)
        .filter(|&i| flags[i as usize] && gcd(m, i) == 1)
        .map(|i| count_rec(n, k, flags, m * i.pow(r), r - 1))
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
    count_rec(n, k, &flags, 1, 2 * k - 1)
}

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

const MIN_K: [u32; 21] = [
    0, 0, 1, 2, 4, 6, 7, 9, 11, 12, 14, 16, 18, 19, 21, 23, 24, 26, 28, 30, 31,
];
const MAX_POW: [u64; 11] = [
    0,
    u64::MAX,
    9_330_124_695,
    11_938_035,
    526_402,
    85_014,
    25_017,
    10_251,
    5_137,
    2_903,
    1_796,
];

/// Return the `n`th `k`-powerful integer, or `None` on overflow or zero rank.
/// Port of `powerful.c` / `nth_powerful`; invert `powerful_count`.
/// A zero `k` includes all positive integers, as in MPU's public API.
/// ```
/// assert_eq!(math_prime_util::nth_powerful(4, 2), Some(9));
/// ```
#[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)] // C density estimates only set search bounds.
pub fn nth_powerful(n: u64, k: u32) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if k <= 1 || n == 1 {
        return Some(n);
    }
    if k >= 64 {
        return None;
    }
    let max = if k <= 10 {
        MAX_POW[k as usize]
    } else {
        powerful_count(u64::MAX, k)
    };
    if n > max {
        return None;
    }
    if n <= 20 && k >= MIN_K[n as usize] {
        return 1_u64.checked_shl(k + u32::try_from(n - 2).unwrap_or(0));
    }
    let nf = n as f64;
    let nc = nf.powi(2) / 2.173_254_312_519_554_1_f64.powi(2);
    let (dlo, dhi) = if k == 2 {
        let npow = nf.powf(5.0 / 3.0);
        (
            nc + 0.3 * npow,
            if n < 170 { 8574.0 } else { nc + 0.5 * npow },
        )
    } else if k == 3 {
        let (nest, lower, upper) = if n < 84000 {
            (0.06003 * nf.powf(2.865), 0.96, 1.08)
        } else {
            (0.02209 * nf.powf(2.955), 0.987, 1.025)
        };
        let mut high = upper * (nc + nest);
        if n < 900 {
            high *= 1.3;
        }
        if n < 160 {
            high = 1.3 * high + 600.0;
        }
        (lower * (nc + nest), high)
    } else if k <= 10 {
        let offset = f64::from(k - 4);
        let (nest, lower, upper) = if n < 200 {
            (
                (0.5462 / 1.15_f64.powf(offset)) * nf.powf(3.031 + 0.460 * offset),
                0.51,
                1.86,
            )
        } else {
            (
                (0.01275 / 4.11_f64.powf(offset)) * nf.powf(3.690 + 0.665 * offset),
                0.70,
                4.3,
            )
        };
        (lower * (nc + nest), upper * (nc + nest))
    } else {
        let low = (1_u64.checked_shl(k + 1).unwrap_or(u64::MAX)).saturating_add(1);
        (
            low as f64,
            low as f64 + (nf / max as f64) * (u64::MAX - low) as f64,
        )
    };
    let mut lo = (dlo as u64).max(1);
    let mut hi = (dhi as u64).saturating_add(1);
    // Correct loose density bounds before integer inversion.
    if powerful_count(lo, k) >= n {
        lo = 1;
    }
    while powerful_count(hi, k) < n {
        let next = hi.saturating_mul(2);
        if next == hi {
            return None;
        }
        hi = next;
    }
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if powerful_count(mid, k) < n {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    Some(lo)
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
