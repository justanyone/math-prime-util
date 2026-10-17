//! Bernstein's 2003 AKS variant from `aks.c`.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use crate::modular::powmod_u;

fn log_gamma(x: f64) -> f64 {
    let coefficients = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    let mut sum = 0.0;
    for i in (1..=8).rev() {
        sum += coefficients[i] / (x + i as f64);
    }
    sum += coefficients[0];
    let base = x + 7.5;
    0.918_938_533_204_672_8 + (sum / x).ln() + ((x + 0.5) * base.ln() - base)
}
fn log_binomial(n: u64, k: u64) -> f64 {
    if n < k {
        return -f64::MAX;
    }
    log_gamma((n + 1) as f64) - log_gamma((k + 1) as f64) - log_gamma((n - k + 1) as f64)
}
fn acceptable(n: u64, r: u64, s: u64) -> bool {
    let d = ((r - 1) / 2).min(r - 2);
    let i = ((0.475 * (r - 1) as f64) as u64).min(d);
    let j = ((0.475 * (r - 1) as f64) as u64).min(r - 2 - d);
    log_binomial(2 * s, i)
        + log_binomial(d, i)
        + log_binomial(2 * s - i, j)
        + log_binomial(r - 2 - d, j)
        >= (((r - 1) as f64 / 3.0).sqrt().ceil() * (n as f64).ln()) + 1e-7
}
fn primitive(n: u64, r: u64) -> bool {
    if n % r == 0 {
        return false;
    }
    let mut remaining = r - 1;
    let mut q = 2;
    while q * q <= remaining {
        if remaining % q == 0 {
            if powmod_u(n, (r - 1) / q, r) == 1 {
                return false;
            }
            while remaining % q == 0 {
                remaining /= q;
            }
        }
        q += 1;
    }
    remaining == 1 || powmod_u(n, (r - 1) / remaining, r) != 1
}
// aks.c::poly_mod_mul: cyclic convolution modulo X^r - 1 and n.
fn multiply(x: &[u64], y: &[u64], n: u64) -> Vec<u64> {
    let mut out = vec![0; x.len()];
    for (i, &a) in x.iter().enumerate() {
        if a == 0 {
            continue;
        }
        for (j, &b) in y.iter().enumerate() {
            let k = (i + j) % x.len();
            out[k] = ((u128::from(out[k]) + u128::from(a) * u128::from(b)) % u128::from(n)) as u64;
        }
    }
    out
}
fn test_anr(a: u64, n: u64, r: usize) -> bool {
    let mut polynomial = vec![0; r];
    polynomial[0] = a % n;
    polynomial[1] = 1;
    let mut result = vec![0; r];
    result[0] = 1;
    let mut exponent = n;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = multiply(&result, &polynomial, n);
        }
        exponent >>= 1;
        if exponent != 0 {
            polynomial = multiply(&polynomial, &polynomial, n);
        }
    }
    let k = (n % r as u64) as usize;
    result[k] = if result[k] == 0 { n - 1 } else { result[k] - 1 };
    result[0] = ((u128::from(result[0]) + u128::from(n - a % n)) % u128::from(n)) as u64;
    result.iter().all(|&v| v == 0)
}

/// Test primality with the Bernstein AKS algorithm.
///
/// Port of `aks.c::is_aks_prime` with `IMPL_BERN41`, including its bounds.
/// This deterministic polynomial test can be slow for large primes.
///
/// ```
/// assert!(math_prime_util::is_aks_prime(101));
/// assert!(!math_prime_util::is_aks_prime(121));
/// ```
pub fn is_aks_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if crate::int::is_power_exp(n) > 1 {
        return false;
    }
    if n > 11 && [2, 3, 5, 7, 11].iter().any(|p| n % p == 0) {
        return false;
    }
    let log2n = (n as f64).ln() / 2_f64.ln();
    let r0 = (if log2n > 32.0 { 0.010 } else { 0.003 }) * log2n * log2n;
    let multiplier = if log2n > 32.0 { 6 } else { 30 };
    let mut r = crate::next_prime((r0 as u64).max(2)).unwrap();
    while !primitive(n, r) || !acceptable(n, r, multiplier * (r - 1)) {
        r = crate::next_prime(r).unwrap();
    }
    let mut low = ((0.475 * (r - 1) as f64) as u64).max(1);
    let mut high = multiplier * (r - 1);
    while low < high {
        let mid = low + (high - low) / 2;
        if acceptable(n, r, mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    let s = high + 1;
    let limit = s * (s - 1);
    for divisor in 2..=limit.min(crate::sqrtint(n)) {
        if n % divisor == 0 {
            return false;
        }
    }
    if u128::from(limit) * u128::from(limit) >= u128::from(n) {
        return true;
    }
    (2..=s).all(|a| powmod_u(a, n - 1, n) == 1) && (2..=s).all(|a| test_anr(a, n, r as usize))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn polynomial_congruence() {
        assert!(test_anr(2, 101, 7));
        assert!(!test_anr(2, 15, 7));
        assert!(test_anr(102, 101, 7));
        assert_eq!(multiply(&[0, 1], &[0, 1], 7), vec![1, 0]);
        assert_eq!(log_binomial(1, 2), -f64::MAX);
    }
}
