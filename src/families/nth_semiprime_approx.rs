//! Approximate nth semiprime from MPU 0.75 `semi_primes.c`.

use super::nth_semiprime::{MAX_SEMI_PRIME, MAX_SEMI_PRIME_IDX, SEMIPRIME_LIST};
use crate::semiprime_count_approx;

/// Estimates the `n`-th semiprime (1-based), or returns `None` when `n` is 0.
///
/// Port of `nth_semiprime_approx` in MPU `semi_primes.c`: the piecewise
/// error-factor estimate with three blended ranges gives a first value, and
/// inverse interpolation against [`semiprime_count_approx`] (MPU
/// `inverse_interpolate.c`, threshold 0) refines it. Indices below the small
/// table come from the table and are exact.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_semiprime_approx;
///
/// assert_eq!(nth_semiprime_approx(1), Some(4));
/// assert_eq!(nth_semiprime_approx(1234), Some(4489));
/// assert_eq!(nth_semiprime_approx(0), None);
/// ```
#[must_use]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)] // An estimate: f64 rounding is accepted, and `est` is positive and below u64::MAX here.
pub fn nth_semiprime_approx(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if let Ok(idx) = usize::try_from(n) {
        if idx < SEMIPRIME_LIST.len() {
            return Some(u64::from(SEMIPRIME_LIST[idx]));
        }
    }
    if n >= MAX_SEMI_PRIME_IDX {
        return Some(if n == MAX_SEMI_PRIME_IDX {
            MAX_SEMI_PRIME
        } else {
            0
        });
    }
    let nf = n as f64;
    let logn = nf.ln();
    let log2n = logn.ln();
    let log3n = log2n.ln();
    let log4n = log3n.ln();
    let err_lo = 1.000 - 0.000_182_160_88 * logn + 0.180_996_098_86 * log2n
        - 0.519_624_743_56 * log3n
        - 0.011_361_433_81 * log4n;
    let err_md = 0.968 - 0.000_732_979_45 * logn + 0.097_316_903_14 * log2n
        - 0.252_125_007_49 * log3n
        - 0.013_667_953_46 * log4n;
    let err_hi = 0.968 - 0.000_080_341_09 * logn + 0.015_226_283_93 * log2n
        - 0.040_202_573_67 * log3n
        - 0.012_664_471_75 * log4n;
    let err_factor = if n <= 1 << 26 {
        err_lo
    } else if n < 1 << 27 {
        // Linear interpolation of the two in the blend area.
        let x = (nf - 67_108_864.0) / 67_108_864.0;
        (1.0 - x) * err_lo + x * err_md
    } else if logn <= 31.884_770_305_75 {
        err_md
    } else if logn < 32.577_917_486_32 {
        let x = (nf - 70_368_744_177_664.0) / 70_368_744_177_664.0;
        (1.0 - x) * err_md + x * err_hi
    } else {
        err_hi
    };
    let est = err_factor * nf * logn / log2n;
    if est >= MAX_SEMI_PRIME as f64 {
        return Some(MAX_SEMI_PRIME);
    }
    // Inverse interpolation against the count estimate improves the result.
    let lo = (0.979 * est - 5.0) as u64;
    let dhi = 1.03 * est;
    let hi = if dhi >= MAX_SEMI_PRIME as f64 {
        MAX_SEMI_PRIME
    } else {
        dhi as u64
    };
    Some(inverse(lo, hi, n))
}

// inverse_interpolate.c: _linear_interpolate.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn linear(lo: u64, hi: u64, rlo: u64, rhi: u64, n: u64) -> u64 {
    if n == rlo {
        return lo + 1;
    }
    if n == rhi {
        return hi - 1;
    }
    let step = ((n - rlo) as f64 * (hi - lo) as f64 / (rhi - rlo) as f64 + 0.5) as u64;
    lo + step.clamp(1, hi - lo - 1)
}

// inverse_interpolate.c: _inverse_interpolate, with known bounds and threshold 0.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn inverse(mut lo: u64, mut hi: u64, n: u64) -> u64 {
    let mut mid = lo + (hi - lo) / 2;
    let rmid = semiprime_count_approx(mid);
    let (mut rlo, mut rhi) = if rmid >= n {
        hi = mid;
        (semiprime_count_approx(lo), rmid)
    } else {
        lo = mid;
        (rmid, semiprime_count_approx(hi))
    };
    if rlo >= n {
        return lo;
    }
    if hi - lo <= 1 {
        return if rhi > n { lo } else { hi };
    }
    // Step 1: linear interpolation until `rhi` is exact.
    mid = linear(lo, hi, rlo, rhi, n);
    let mut iteration = 1;
    while hi - lo > 1 && rhi > n {
        let value = semiprime_count_approx(mid);
        if value >= n {
            hi = mid;
            rhi = value;
        } else {
            lo = mid;
            rlo = value;
        }
        if rhi == n || hi - lo <= 1 {
            break;
        }
        mid = linear(lo, hi, rlo, rhi, n);
        if iteration % 6 == 0 {
            // Pull in the far edge when the search sits close to one edge.
            let close = (0.003 * (hi - lo) as f64 + 1.0) as u64;
            mid = mid.clamp(lo + close, hi - close);
        }
        mid = mid.clamp(lo + 1, hi - 1);
        iteration += 1;
    }
    if rlo == n {
        return lo;
    }
    if hi - lo <= 1 {
        return if rhi > n { lo } else { hi };
    }
    // Step 2: Ridder's method until the bracket is small.
    while hi - lo > 8 {
        let mut x1 = lo + (hi - lo) / 2;
        let mut rx1 = semiprime_count_approx(x1);
        let d0 = (n - rlo) as f64;
        let d1 = rx1.abs_diff(n) as f64;
        let d2 = (rhi - n + 1) as f64;
        let pos = (x1 - lo) as f64 * d1 / (d1 * d1 + d0 * d2).sqrt();
        let step = (pos + 0.5) as u64;
        // A step that leaves the bracket counts as no new point (bisection).
        let x3 = if rx1 >= n {
            x1.checked_sub(step).unwrap_or(hi)
        } else {
            x1.checked_add(step).unwrap_or(hi)
        };
        if x3 >= hi || x3 <= lo || x3 == x1 {
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else {
                lo = x1;
                rlo = rx1;
            }
        } else {
            let mut x3 = x3;
            let mut rx3 = semiprime_count_approx(x3);
            if rx1 > rx3 {
                std::mem::swap(&mut x1, &mut x3);
                std::mem::swap(&mut rx1, &mut rx3);
            }
            if rx1 >= n {
                hi = x1;
                rhi = rx1;
            } else if rx3 >= n {
                lo = x1;
                rlo = rx1;
                hi = x3;
                rhi = rx3;
            } else {
                lo = x3;
                rlo = rx3;
            }
        }
    }
    // Step 3: binary search.
    while hi - lo > 1 {
        mid = lo + (hi - lo) / 2;
        if semiprime_count_approx(mid) < n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_and_bounds() {
        assert_eq!(nth_semiprime_approx(0), None);
        assert_eq!(nth_semiprime_approx(2), Some(6));
        assert_eq!(nth_semiprime_approx(190), Some(623));
        assert_eq!(
            nth_semiprime_approx(MAX_SEMI_PRIME_IDX),
            Some(MAX_SEMI_PRIME)
        );
        assert_eq!(nth_semiprime_approx(MAX_SEMI_PRIME_IDX + 1), Some(0));
    }

    #[test]
    fn refined_values() {
        // MPU 0.75 values (tests/fixtures/families.tsv).
        assert_eq!(nth_semiprime_approx(1234), Some(4489));
        assert_eq!(nth_semiprime_approx(12345), Some(51052));
        assert_eq!(nth_semiprime_approx(123_456), Some(573_476));
    }

    #[test]
    fn estimate_branches() {
        // Covers the low, blend low-mid, mid, blend mid-high and high ranges.
        for n in [
            1000u64,
            1 << 26,
            (1 << 26) + 1,
            1 << 27,
            1 << 40,
            1 << 50,
            1 << 60,
        ] {
            let v = nth_semiprime_approx(n).unwrap();
            assert!(v > n, "estimate for {n} is {v}");
            assert!(
                semiprime_count_approx(v) >= n,
                "refined {v} counts below {n}"
            );
        }
        assert!(nth_semiprime_approx(1 << 60).unwrap() >= 1 << 60);
    }

    #[test]
    fn linear_interpolation() {
        assert_eq!(linear(10, 20, 1, 3, 1), 11);
        assert_eq!(linear(10, 20, 1, 3, 3), 19);
        assert_eq!(linear(10, 20, 0, 10, 5), 15);
    }
}
