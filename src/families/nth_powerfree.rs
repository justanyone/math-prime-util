use super::{is_powerfree, powerfree_count};

/// Return the `n`th `k`-powerfree integer, or `None` on overflow or zero rank.
/// Port of `powerfree.c` / `nth_powerfree`.
/// ```
/// assert_eq!(math_prime_util::nth_powerfree(7, 2), Some(10));
/// ```
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn nth_powerfree(n: u64, k: u32) -> Option<u64> {
    if n == 0 || k < 2 {
        return None;
    }
    if k >= 64 || n < 4 {
        return Some(n);
    }
    let max = match k {
        2 => 11_214_275_663_373_200_251,
        3 => 15_345_982_395_028_449_439,
        4 => 17_043_655_258_566_511_333,
        _ => powerfree_count(u64::MAX, k),
    };
    if n > max {
        return None;
    }
    // MPU uses zeta(k) for the density correction.
    let zeta = 1.0 + crate::riemann_zeta(f64::from(k));
    let mut q = (n as f64 * zeta + 0.5) as u64;
    let threshold = match k {
        2 => 200,
        3 => 60,
        4 => 2,
        _ => 1,
    };
    for i in 0..10 {
        let count = powerfree_count(q, k);
        let diff = count.abs_diff(n);
        if diff <= threshold || i == 9 {
            break;
        }
        let delta = (diff as f64 * zeta) as u64;
        q = if count > n {
            q.saturating_sub(delta).max(1)
        } else {
            q.saturating_add(delta)
        };
    }
    let mut count = powerfree_count(q, k);
    while !is_powerfree(q, k) {
        q -= 1;
    }
    while count != n {
        if count < n {
            loop {
                q = q.checked_add(1)?;
                if is_powerfree(q, k) {
                    break;
                }
            }
            count += 1;
        } else {
            loop {
                q -= 1;
                if is_powerfree(q, k) {
                    break;
                }
            }
            count -= 1;
        }
    }
    Some(q)
}
