//! Sebah and Gourdon twin prime estimate.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

/// Estimate the count of twin pairs up to `n`.
///
/// Port of `twin_primes.c` (MPU 0.75), `twin_prime_count_approx`.
/// Use the exact count below 2000 and the C correction intervals above it.
///
/// ```
/// assert_eq!(math_prime_util::twin_prime_count_approx(100), 8);
/// ```
#[must_use]
pub fn twin_prime_count_approx(n: u64) -> u64 {
    if n < 2000 {
        return super::twin_prime_count(n);
    }
    let x = n as f64;
    let logn = x.ln();
    let mut li2 = crate::exponential_integral(logn) + 2.0 / 2.0_f64.ln() - x / logn;
    if n < 32_000_000 {
        let fm = match n {
            0..4000 => 0.2952,
            4000..8000 => 0.3102,
            8000..16000 => 0.3090,
            16000..32000 => 0.3096,
            32000..64000 => 0.3097,
            64000..128000 => 0.3094,
            128000..256000 => 0.3099,
            256000..600000 => 0.3098 + (x - 256000.0) * (0.3056 - 0.3098) / 344000.0,
            600000..1000000 => 0.3062 + (x - 600000.0) * (0.3042 - 0.3062) / 400000.0,
            1000000..4000000 => 0.3067 + (x - 1000000.0) * (0.3041 - 0.3067) / 3000000.0,
            4000000..16000000 => 0.3041 + (x - 4000000.0) * (0.2983 - 0.3041) / 12000000.0,
            _ => 0.2983 + (x - 16000000.0) * (0.2961 - 0.2983) / 16000000.0,
        };
        li2 *= fm * (12.0 + logn).ln();
    }
    (1.320_323_631_693_739 * li2 + 0.5) as u64
}
