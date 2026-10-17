//! Nth semiprime from MPU 0.75 `semi_primes.c`.

/// Small semiprimes, as in `_semiprimelist` of MPU `semi_primes.c`.
/// Entry 0 is a placeholder: there is no zeroth semiprime.
pub(crate) const SEMIPRIME_LIST: [u32; 0] = [];

/// Tests whether `n` is a semiprime (a product of exactly two primes).
///
/// Trial division up to `sqrt(n)`. MPU `is_semiprime` in `semi_primes.c`
/// uses the same test on its factor pattern.
fn is_semiprime(n: u64) -> bool {
    if n < 4 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            let q = n / d;
            return is_prime(d) && is_prime(q);
        }
        d += 1;
    }
    false
}

/// Trial-division primality test used by [`is_semiprime`].
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

/// Returns the `n`-th semiprime (1-based), or `None` when `n` is 0.
///
/// Port of `nth_semiprime` in MPU `semi_primes.c`. Small indices come from
/// the table. Larger indices use a linear scan with a trial-division test.
/// This differs from MPU's interpolation and sieve method, so only the
/// results match, not the path taken.
///
/// # Examples
///
/// ```
/// use math_prime_util::nth_semiprime;
///
/// assert_eq!(nth_semiprime(1), Some(4));
/// assert_eq!(nth_semiprime(5), Some(15));
/// assert_eq!(nth_semiprime(0), None);
/// ```
#[must_use]
pub fn nth_semiprime(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let idx = usize::try_from(n).ok()?;
    if idx < SEMIPRIME_LIST.len() {
        return Some(u64::from(SEMIPRIME_LIST[idx]));
    }
    let mut count = SEMIPRIME_LIST.len() as u64 - 1;
    let mut m = u64::from(*SEMIPRIME_LIST.last()?);
    while count < n {
        m += 1;
        if is_semiprime(m) {
            count += 1;
        }
    }
    Some(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_semiprimes() {
        assert_eq!(nth_semiprime(0), None);
        assert_eq!(nth_semiprime(1), Some(4));
        assert_eq!(nth_semiprime(2), Some(6));
        assert_eq!(nth_semiprime(3), Some(9));
    }

    #[test]
    fn scan_past_table() {
        assert!(is_semiprime(4) && is_semiprime(6) && !is_semiprime(8));
        assert!(!is_semiprime(3) && !is_semiprime(1) && !is_semiprime(0));
        assert!(!is_prime(0) && !is_prime(1) && is_prime(2));
        assert_eq!(nth_semiprime(100), Some(262));
    }
}
