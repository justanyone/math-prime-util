/// Returns the product of the first `n` primes, or `None` on `u64` overflow.
///
/// Port of `util.c`'s `pn_primorial` and its 64-bit `_pn_prim` table in MPU 0.75.
///
/// ```
/// assert_eq!(math_prime_util::pn_primorial(4), Some(210));
/// assert_eq!(math_prime_util::pn_primorial(16), None);
/// ```
pub fn pn_primorial(n: u64) -> Option<u64> {
    const PRODUCTS: [u64; 16] = [
        1, 2, 6, 30, 210, 2310, 30030, 510510, 9699690, 223092870,
        6469693230, 200560490130, 7420738134810, 304250263527210,
        13082761331670030, 614889782588491410,
    ];
    usize::try_from(n).ok().and_then(|i| PRODUCTS.get(i).copied())
}
