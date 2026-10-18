#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use super::fermat_factor::{attempt_list, pair};
use crate::{gcd, is_prime, is_square_root, rootint};

/// Tries Lehman's factorization method.
///
/// Port of MPU 0.75 `factor.c::lehman_factor` and its `XS.xs` wrapper.
/// Trial division defaults to true. The C search stops at 8,796,393,022,207.
/// A failed attempt retains the cofactor. Factors can be composite.
/// Zero returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::lehman_factor;
/// assert_eq!(lehman_factor(91, None), vec![7, 13]);
/// ```
pub fn lehman_factor(n: u64, do_trial: Option<bool>) -> Vec<u64> {
    attempt_list(n, |n| split(n, do_trial.unwrap_or(true)))
}

fn split(n: u64, do_trial: bool) -> Vec<u64> {
    let tune: f64 = if n >> 36 != 0 { 3.5 } else { 5.0 };
    let bound = (tune * (1 + rootint(n, 3)) as f64) as u64;
    let mut trial_start = 3;
    if do_trial {
        let first_cut = ((0.1 * bound as f64) as u64).clamp(84, 65535);
        // primes_small in MPU contains primes through 2011.
        for p in (3..=2011).step_by(2).filter(|&p| is_prime(p) != 0) {
            trial_start = p;
            if p >= first_cut {
                break;
            }
            if n % p == 0 {
                return pair(n, p);
            }
        }
    }
    if n >= 8_796_393_022_207 {
        return vec![n];
    }
    let reduced = (bound as f64 / (tune * tune * tune)) as u64;
    // B is uint32_t in the C expression B*B, before promotion to UV.
    let bound_squared = u64::from((bound as u32).wrapping_mul(bound as u32));
    let mut kn = 0;
    for k in 1..=reduced {
        let (inc, r) = if k & 1 != 0 { (4, (k + n) % 4) } else { (2, 1) };
        kn += n;
        if kn >= 1_152_921_504_606_846_976 {
            return vec![n];
        }
        let mut x = (kn as f64).sqrt();
        let a = x as u64;
        if a * a == kn {
            return pair(n, gcd(a, n));
        }
        x *= 2.0;
        let a = (x + 0.999_999_966_5) as u64;
        let b = a + (inc + r - a % inc) % inc;
        let mut c = b * b - 4 * kn;
        let upper = (x + bound_squared as f64 / (2.0 * x)) as u64;
        let mut a = b;
        while a <= upper {
            if let Some(b) = is_square_root(c) {
                return pair(n, gcd(a + b, n));
            }
            c += inc * (a + a + inc);
            a += inc;
        }
    }
    if do_trial {
        let mut factors = Vec::new();
        let mut remaining = n;
        for p in (trial_start..=bound.min(65535)).step_by(2) {
            if p > remaining / p {
                break;
            }
            while remaining % p == 0 {
                factors.push(p);
                remaining /= p;
            }
        }
        if remaining > 1 {
            factors.push(remaining);
        }
        return factors;
    }
    vec![n]
}

#[cfg(test)]
mod tests {
    use super::split;

    #[test]
    fn odd_multiplier_and_square_paths() {
        // k = 1 is odd. The search must use the odd residue class.
        assert_eq!(split(47 * 53, false), [47, 53]);
        // An exact square reaches the a*a == kn return before the interval search.
        assert_eq!(split(101 * 101, false), [101, 101]);
    }

    #[test]
    fn trial_fallback_keeps_multiplicity() {
        assert_eq!(split(89_u64.pow(3), true), vec![89, 89, 89]);
        assert_eq!(split(1009, true), vec![1009]);
        assert_eq!(split(1009, false), vec![1009]);
    }

    #[test]
    fn exact_results_across_sizes_and_trial() {
        use crate::factor::test_support::{fold, wide_inputs};
        let mut h = 0;
        for n in wide_inputs() {
            for do_trial in [true, false] {
                let got = split(n, do_trial);
                assert_eq!(got.iter().product::<u64>(), n, "split({n},{do_trial})");
                for f in got {
                    h = fold(h, f);
                }
                h = fold(h, 1);
            }
        }
        assert_eq!(h, 7_218_862_803_161_328_452);
    }
}
