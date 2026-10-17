#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use super::fermat_factor::attempt;
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
    attempt(n, |n| split(n, do_trial.unwrap_or(true)))
}

fn split(n: u64, do_trial: bool) -> u64 {
    let tune: f64 = if n >> 36 != 0 { 3.5 } else { 5.0 };
    let bound = (tune * (1 + rootint(n, 3)) as f64) as u64;
    let mut trial_start = 3;
    if do_trial {
        let first_cut = ((0.1 * bound as f64) as u64).clamp(84, 65535);
        // primes_small in MPU contains primes through 2011.
        for p in (3..=2011).step_by(2).filter(|&p| is_prime(p) != 0) {
            trial_start = p;
            if p >= first_cut { break; }
            if n % p == 0 { return p; }
        }
    }
    if n >= 8_796_393_022_207 { return n; }
    let reduced = (bound as f64 / (tune * tune * tune)) as u64;
    let bound_squared = bound * bound;
    let mut kn = 0;
    for k in 1..=reduced {
        let (inc, r) = if k & 1 != 0 { (4, (k + n) % 4) } else { (2, 1) };
        kn += n;
        if kn >= 1_152_921_504_606_846_976 { return n; }
        let mut x = (kn as f64).sqrt();
        let a = x as u64;
        if a * a == kn { return gcd(a, n); }
        x *= 2.0;
        let a = (x + 0.999_999_966_5) as u64;
        let b = a + (inc + r - a % inc) % inc;
        let mut c = b * b - 4 * kn;
        let upper = (x + bound_squared as f64 / (2.0 * x)) as u64;
        let mut a = b;
        while a <= upper {
            if let Some(b) = is_square_root(c) { return gcd(a + b, n); }
            c += inc * (a + a + inc);
            a += inc;
        }
    }
    if do_trial {
        for p in (trial_start..=bound.min(65535)).step_by(2) {
            if n % p == 0 { return p; }
        }
    }
    n
}
