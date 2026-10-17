//! Approximate nth semiprime from MPU 0.75 `semi_primes.c`.

use super::nth_semiprime::SEMIPRIME_LIST;

/// Largest semiprime that MPU 0.75 returns (`MPU_MAX_SEMI_PRIME`, 64-bit).
const MAX_SEMI_PRIME: u64 = 18_446_744_073_709_551_601;
/// Index of the largest semiprime (`MPU_MAX_SEMI_PRIME_IDX`, 64-bit).
const MAX_SEMI_PRIME_IDX: u64 = 1_701_748_900_850_019_777;

/// Estimates the `n`-th semiprime (1-based), or returns `None` when `n` is 0.
///
/// Port of `nth_semiprime_approx` in MPU `semi_primes.c`: the piecewise
/// error-factor estimate with three blended ranges. The inverse-interpolation
/// refinement of MPU is not ported yet, so values for `n` past the table can
/// differ from MPU.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_semiprime_approx;
///
/// assert_eq!(nth_semiprime_approx(1), Some(4));
/// assert!(nth_semiprime_approx(1000).is_some());
/// assert_eq!(nth_semiprime_approx(0), None);
/// ```
#[must_use]
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
    Some(est as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_and_bounds() {
        assert_eq!(nth_semiprime_approx(0), None);
        assert_eq!(nth_semiprime_approx(2), Some(6));
        assert_eq!(nth_semiprime_approx(MAX_SEMI_PRIME_IDX), Some(MAX_SEMI_PRIME));
        assert_eq!(nth_semiprime_approx(MAX_SEMI_PRIME_IDX + 1), Some(0));
    }

    #[test]
    fn estimate_branches() {
        // Covers the low, blend low-mid, mid, blend mid-high and high ranges.
        for n in [1000u64, 1 << 26, (1 << 26) + 1, 1 << 27, 1 << 40, 1 << 50, 1 << 60] {
            let v = nth_semiprime_approx(n).unwrap();
            assert!(v > n, "estimate for {n} is {v}");
        }
        assert!(nth_semiprime_approx(1 << 60).unwrap() >= 1 << 60);
    }
}
