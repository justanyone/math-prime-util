#![allow(clippy::cast_possible_truncation)] // C estimates are clamped to native bounds.
use super::powerful_count;

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

