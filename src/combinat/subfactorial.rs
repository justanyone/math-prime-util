/// Returns the number of derangements of `n` items, or `None` on overflow.
///
/// Port of `subfactorial` in MPU 0.75 `util.c`.
/// Uses its recurrence and 64-bit bound.
///
/// ```
/// use math_prime_util::subfactorial;
/// assert_eq!(subfactorial(0), Some(1));
/// assert_eq!(subfactorial(5), Some(44));
/// assert_eq!(subfactorial(21), None);
/// ```
pub fn subfactorial(n: u64) -> Option<u64> {
    if n <= 3 {
        return Some(if n == 0 { 1 } else { n - 1 });
    }
    if n >= 21 {
        return None;
    }
    let result = n * subfactorial(n - 1)?;
    Some(if n & 1 == 1 { result - 1 } else { result + 1 })
}
