use super::is_prime_power;
use crate::{primes, rootint, sqrtint};

/// Return all prime powers in the inclusive interval `lo..=hi`.
///
/// Return an empty vector if the interval is empty.
/// Port of MPU 0.75 `prime_powers.c`, `prime_power_sieve` and
/// `prime_power_sieve2`. Generate higher powers, sort them, and merge primes.
///
/// ```
/// use math_prime_util::prime_powers;
/// assert_eq!(prime_powers(4, 16), vec![4, 5, 7, 8, 9, 11, 13, 16]);
/// assert!(prime_powers(17, 16).is_empty());
/// ```
pub fn prime_powers(lo: u64, hi: u64) -> Vec<u64> {
    if hi < 2 || lo > hi {
        return Vec::new();
    }
    let scan_limit = if hi < 7_929_856 { 10 } else { sqrtint(hi) >> 8 };
    if hi - lo < scan_limit {
        return (lo..=hi).filter(|&n| is_prime_power(n) != 0).collect();
    }
    let mut powers = Vec::new();
    for k in 2..=hi.ilog2() {
        for p in primes(2, rootint(hi, k)) {
            let power = p.pow(k);
            if power >= lo {
                powers.push(power);
            }
        }
    }
    powers.sort_unstable();
    let primes = primes(lo, hi);
    if powers.is_empty() {
        return primes;
    }
    let mut result = Vec::with_capacity(primes.len() + powers.len());
    let (mut iprime, mut ipower) = (0, 0);
    while iprime < primes.len() && ipower < powers.len() {
        if primes[iprime] < powers[ipower] {
            result.push(primes[iprime]);
            iprime += 1;
        } else {
            result.push(powers[ipower]);
            ipower += 1;
        }
    }
    result.extend_from_slice(&primes[iprime..]);
    result.extend_from_slice(&powers[ipower..]);
    result
}
