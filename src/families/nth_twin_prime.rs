//! Select a twin prime by its index.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use super::twin_prime_count::table_skip;
use super::twin_primes::{scan, MAX_TWIN};

/// Return the smaller prime of twin pair number `n`.
///
/// Port of `twin_primes.c` (MPU 0.75), `nth_twin_prime`.
/// The index starts at one. Return `None` for zero or an index outside
/// the native range.
///
/// ```
/// assert_eq!(math_prime_util::nth_twin_prime(6), Some(41));
/// assert_eq!(math_prime_util::nth_twin_prime(0), None);
/// ```
#[must_use]
pub fn nth_twin_prime(mut n: u64) -> Option<u64> {
    if n == 0 || n > 12_975_810_317_986_308 {
        return None;
    }
    if n < 6 {
        return Some([0, 3, 5, 11, 17, 29][n as usize]);
    }
    let mut begin = 31;
    let end = estimate(n);
    if n > 58_980 {
        table_skip(|bound, count| {
            if bound > end || n <= count {
                return false;
            }
            n -= count;
            begin = bound + 1;
            true
        });
    }
    if begin == 31 {
        n -= 5;
    }
    let mut result = None;
    scan(begin, end, |p| {
        n -= 1;
        if n == 0 {
            result = Some(p);
            false
        } else {
            true
        }
    });
    result
}

fn estimate(n: u64) -> u64 {
    let x = n as f64;
    let scale = x * x.ln().powi(2);
    let factor = if n >= 10_000_000_000_000_000 {
        1.04
    } else if n >= 10_000_000_000_000 {
        1.10
    } else if n >= 10_000_000 {
        1.31
    } else if n >= 1200 {
        1.70
    } else {
        2.30
    };
    let mut lo = (0.9 * scale) as u64;
    let mut hi = ((factor * scale + if n < 1200 { 5.0 } else { 0.0 }) as u64).min(MAX_TWIN);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if super::twin_prime_count_approx(mid) < n {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    (800.0 + 1.01 * lo as f64).min((u64::MAX - 16) as f64) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimate_covers_each_c_bound() {
        for n in [
            6,
            1200,
            10_000_000,
            10_000_000_000_000,
            10_000_000_000_000_000,
        ] {
            assert!(estimate(n) > n);
        }
        assert!(estimate(100_000) >= 18_409_199);
    }
}
