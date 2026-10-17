use super::is_almost_prime::below_power;
use super::is_semiprime::factors;

/// Return whether `n` has exactly `k` distinct prime factors.
///
/// Port of MPU 0.75 `omega_primes.c::is_omega_prime` and
/// `factor.c::prime_omega`. The prime power test uses
/// `prime_powers.c::prime_power`.
///
/// ```
/// use math_prime_util::is_omega_prime;
/// assert!(is_omega_prime(2, 12));
/// assert!(!is_omega_prime(3, 12));
/// ```
pub fn is_omega_prime(mut k: u32, mut n: u64) -> bool {
    if n <= 1 { return n == 1 && k == 0; }
    for p in [2, 3, 5, 7, 11] {
        if k > 0 && n % p == 0 {
            k -= 1;
            loop {
                n /= p;
                if n % p != 0 { break; }
            }
        }
    }
    if n == 1 { return k == 0; }
    if k == 0 { return false; }
    if k == 1 {
        if crate::is_prime(n) != 0 { return true; }
        let power = crate::int::is_power_exp(n);
        return power > 1 && crate::is_prime(crate::rootint(n, power)) != 0;
    }
    if below_power(n, 13, u64::from(k)) { return false; }
    let mut f = factors(n);
    f.dedup();
    f.len() as u64 == u64::from(k)
}
