use super::is_carmichael::prime_factors;

/// Return whether the group of order `n` must be cyclic.
///
/// Port of `is_cyclic` in MPU 0.75 `util.c`, with factoring from `factor.c`.
/// It tests whether `n` and its totient are coprime.
///
/// ```
/// assert!(math_prime_util::is_cyclic(35));
/// assert!(!math_prime_util::is_cyclic(21));
/// ```
pub fn is_cyclic(n: u64) -> bool {
    if n < 4 {
        return n != 0;
    }
    if n % 2 == 0
        || [9, 25, 49, 21, 39, 55, 57, 93, 121, 169, 111, 129, 155, 183]
            .iter()
            .any(|p| n % p == 0)
    {
        return false;
    }
    if n <= 200 {
        return true;
    }
    let factors = prime_factors(n);
    if factors.len() == 1 {
        return true;
    }
    if factors.windows(2).any(|w| w[0] == w[1]) {
        return false;
    }
    let phi = factors.iter().map(|p| p - 1).product();
    crate::gcd(n, phi) == 1
}
