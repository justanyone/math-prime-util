use super::nth_prime;
use crate::real::{logarithmic_integral, riemann_r};

/// Return an estimate of the `n`th prime, with the first prime as index 1.
///
/// Port of MPU 0.75 `prime_counts.c`, `nth_prime_approx`, which returns
/// `inverse_R(n)` from `real.c`. Small `n` use the exact table, as MPU does.
/// This port runs the Halley iteration of `real.c`, `ld_inverse_R`, in `f64`.
///
/// Returns `None` for `n = 0`, where MPU returns `undef`.
///
/// ```
/// use math_prime_util::nth_prime_approx;
/// let estimate = nth_prime_approx(1_000).unwrap();
/// assert!(estimate.abs_diff(7_919) < 100);
/// assert_eq!(nth_prime_approx(0), None);
/// ```
#[must_use]
pub fn nth_prime_approx(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    // Same cut-off as MPU: NPRIMES_SMALL is 306.
    if n < 306 {
        return nth_prime(n);
    }
    #[allow(clippy::cast_precision_loss)]
    let lx = n as f64;
    let r = inverse_r(lx).ceil();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let approx = r as u64;
    Some(approx)
}

/// Invert Riemann's R function by Halley iteration, as MPU `real.c` does.
fn inverse_r(lx: f64) -> f64 {
    let mut t = if lx <= 3.5 {
        lx + 2.24 * (lx - 1.0) / 2.0
    } else {
        let mut t = lx * lx.ln();
        if lx < 50.0 {
            t *= 1.2;
        } else if lx < 1000.0 {
            t *= 1.15;
        } else {
            // One inverse-li step gives the first estimate for large lx.
            let dn = logarithmic_integral(t) - lx;
            t -= dn * t.ln() / (1.0 + dn / (2.0 * t));
        }
        t
    };
    let mut old_term = 0.0;
    for i in 0..100 {
        let dn = riemann_r(t) - lx;
        let term = dn * t.ln() / (1.0 + dn / (2.0 * t));
        if i > 0 && term.abs() >= old_term.abs() {
            t -= term / 4.0;
            break;
        }
        old_term = term;
        t -= term;
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_is_exact() {
        assert_eq!(nth_prime_approx(1), Some(2));
        assert_eq!(nth_prime_approx(100), Some(541));
        assert_eq!(nth_prime_approx(0), None);
    }

    #[test]
    fn large_is_close() {
        let approx = nth_prime_approx(1_000_000).unwrap();
        assert!(approx.abs_diff(15_485_863) < 20_000);
    }

    #[test]
    fn inverse_r_branches() {
        assert!(inverse_r(2.0) > 0.0);
        assert!(inverse_r(20.0) > 0.0);
        assert!(inverse_r(500.0) > 0.0);
        assert!(inverse_r(5_000.0) > 0.0);
    }
}
