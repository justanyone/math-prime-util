#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
use super::fermat_factor::attempt;
use crate::{gcd, is_square_root, sqrtint};

/// Tries Hart's one line factorization method.
///
/// Port of MPU 0.75 `factor.c::holf_factor` and its `XS.xs` wrapper.
/// `None` uses eight million rounds. A failed attempt retains the cofactor.
/// The returned factors can be composite. Zero returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::holf_factor;
/// assert_eq!(holf_factor(91, None), vec![7, 13]);
/// ```
pub fn holf_factor(n: u64, rounds: Option<u64>) -> Vec<u64> {
    attempt(n, |n| split(n, rounds.unwrap_or(8_000_000)))
}

fn split(n: u64, mut rounds: u64) -> u64 {
    if let Some(root) = is_square_root(n) { return root; }
    if n <= u64::MAX >> 6 {
        let multiplier = if n <= u64::MAX >> 13 { 720 }
            else if n <= u64::MAX >> 11 { 480 }
            else if n <= u64::MAX >> 10 { 360 }
            else if n <= u64::MAX >> 8 { 60 } else { 30 };
        let npre = n * multiplier;
        let mut ni = npre;
        loop {
            if rounds == 0 { return n; }
            rounds -= 1;
            let s = 1 + sqrtint(ni);
            let m = s.wrapping_mul(s).wrapping_sub(ni);
            if let Some(root) = is_square_root(m) {
                let f = gcd(n, s - root);
                if f > 1 && f < n { return f; }
            }
            if ni >= ni.wrapping_add(npre) { break; }
            ni += npre;
        }
    }
    for i in 1..=rounds {
        let s = ((n as f64 * i as f64).sqrt() as u64).saturating_add(1);
        let m = (u128::from(s) * u128::from(s) % u128::from(n)) as u64;
        if let Some(root) = is_square_root(m) { return gcd(s.abs_diff(root), n); }
    }
    n
}
