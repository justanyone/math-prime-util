//! Nth semiprime from MPU 0.75 `semi_primes.c`.

/// Small semiprimes, as in `_semiprimelist` of MPU `semi_primes.c`.
/// Entry 0 is a placeholder: there is no zeroth semiprime.
pub(crate) const SEMIPRIME_LIST: [u32; 191] = [
    0, 4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35,
    38, 39, 46, 49, 51, 55, 57, 58, 62, 65, 69, 74, 77, 82,
    85, 86, 87, 91, 93, 94, 95, 106, 111, 115, 118, 119, 121, 122,
    123, 129, 133, 134, 141, 142, 143, 145, 146, 155, 158, 159, 161, 166,
    169, 177, 178, 183, 185, 187, 194, 201, 202, 203, 205, 206, 209, 213,
    214, 215, 217, 218, 219, 221, 226, 235, 237, 247, 249, 253, 254, 259,
    262, 265, 267, 274, 278, 287, 289, 291, 295, 298, 299, 301, 302, 303,
    305, 309, 314, 319, 321, 323, 326, 327, 329, 334, 335, 339, 341, 346,
    355, 358, 361, 362, 365, 371, 377, 381, 382, 386, 391, 393, 394, 395,
    398, 403, 407, 411, 413, 415, 417, 422, 427, 437, 445, 446, 447, 451,
    453, 454, 458, 466, 469, 471, 473, 478, 481, 482, 485, 489, 493, 497,
    501, 502, 505, 511, 514, 515, 517, 519, 526, 527, 529, 533, 535, 537,
    538, 542, 543, 545, 551, 553, 554, 559, 562, 565, 566, 573, 579, 581,
    583, 586, 589, 591, 597, 611, 614, 622, 623,
];

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
        assert_eq!(nth_semiprime(100), Some(u64::from(SEMIPRIME_LIST[100])));
        // 623 is the last table entry; 626 = 2 * 313 is the next semiprime.
        assert_eq!(nth_semiprime(190), Some(623));
        assert_eq!(nth_semiprime(191), Some(626));
    }
}
