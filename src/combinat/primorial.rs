/// Returns the product of primes at most `n`, or `None` on `u64` overflow.
///
/// Port of `util.c`'s `primorial` and its 64-bit `_prim_map` table in MPU 0.75.
///
/// ```
/// assert_eq!(math_prime_util::primorial(10), Some(210));
/// assert_eq!(math_prime_util::primorial(53), None);
/// ```
pub fn primorial(n: u64) -> Option<u64> {
    const PRIME_COUNTS: [u8; 53] = [
        0, 0, 1, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 10,
        10, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 13, 13, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15,
    ];
    let count = PRIME_COUNTS.get(usize::try_from(n).ok()?)?;
    super::pn_primorial(u64::from(*count))
}
