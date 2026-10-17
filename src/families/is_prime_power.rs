use crate::{int::is_power_exp, is_prob_prime, rootint};

/// Return the exponent if `n` is a power of a prime, or zero otherwise.
///
/// Port of MPU 0.75 `prime_powers.c`, `prime_power` (`is_prime_power`).
/// Primes have exponent one. Zero and one return zero.
///
/// ```
/// use math_prime_util::is_prime_power;
/// assert_eq!(is_prime_power(8), 3);
/// assert_eq!(is_prime_power(13), 1);
/// assert_eq!(is_prime_power(36), 0);
/// ```
pub fn is_prime_power(mut n: u64) -> u32 {
    if n < 2 {
        return 0;
    }
    if n % 2 == 0 {
        return if n.is_power_of_two() { n.trailing_zeros() } else { 0 };
    }
    for p in [3, 5, 7] {
        if n % p == 0 {
            let mut power = 0;
            loop {
                n /= p;
                power += 1;
                if n % p != 0 {
                    break;
                }
            }
            return if n == 1 { power } else { 0 };
        }
    }
    if is_prob_prime(n) != 0 {
        return 1;
    }
    // The integer module supplies MPU's maximal perfect-power exponent.
    let power = is_power_exp(n);
    if power != 0 && is_prob_prime(rootint(n, power)) != 0 {
        power
    } else {
        0
    }
}
