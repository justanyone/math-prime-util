/// Return the least common multiple of `1..=n`.
/// Return `None` if the result exceeds `u64`.
///
/// Port of `util.c`, `consecutive_integer_lcm`, from MPU 0.75.
/// The method multiplies the largest prime powers at most `n`.
///
/// ```
/// use math_prime_util::consecutive_integer_lcm;
/// assert_eq!(consecutive_integer_lcm(10), Some(2520));
/// assert_eq!(consecutive_integer_lcm(47), None);
/// ```
pub fn consecutive_integer_lcm(n: u64) -> Option<u64> {
    // The C small-prime table overflows at 47. No later prime is needed.
    const PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    if n >= 47 {
        return None;
    }
    let mut result = 1u64;
    for prime in PRIMES {
        if prime > n {
            break;
        }
        let mut power = prime;
        while power <= n / prime {
            power *= prime;
        }
        result = result.checked_mul(power)?;
    }
    Some(result)
}
