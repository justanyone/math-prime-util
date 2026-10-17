/// Returns prime factor exponents in descending order.
///
/// Port of `prime_signature` in MPU `XS.xs`, using `factorint` in `factor.c`.
/// Zero returns `[1]`. One returns an empty vector.
///
/// ```
/// assert_eq!(math_prime_util::prime_signature(360), vec![3, 2, 1]);
/// ```
pub fn prime_signature(n: u64) -> Vec<u32> {
    let mut exponents = super::prime_bigomega::exponents(n);
    // MPU uses insertion sort, since there are at most 15 distinct factors.
    for i in 1..exponents.len() {
        let t = exponents[i];
        let mut j = i;
        while j > 0 && exponents[j - 1] < t {
            exponents[j] = exponents[j - 1];
            j -= 1;
        }
        exponents[j] = t;
    }
    exponents
}
