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
        1,
        2,
        6,
        30,
        210,
        2310,
        30_030,
        510_510,
        9_699_690,
        223_092_870,
        6_469_693_230,
        200_560_490_130,
        7_420_738_134_810,
        304_250_263_527_210,
        13_082_761_331_670_030,
        614_889_782_588_491_410,
    ];
    usize::try_from(n)
        .ok()
        .and_then(|i| PRODUCTS.get(i).copied())
}
