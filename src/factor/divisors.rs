/// Return the positive divisors in ascending order.
///
/// Port of MPU 0.75 `factor.c::divisor_list` and `_divisors_from_factors`.
/// `None` includes all divisors. `Some(max)` sets an inclusive limit.
/// Zero has no positive divisors in this API.
///
/// ```
/// use math_prime_util::divisors;
/// assert_eq!(divisors(12, None), vec![1, 2, 3, 4, 6, 12]);
/// assert_eq!(divisors(12, Some(4)), vec![1, 2, 3, 4]);
/// ```
pub fn divisors(n: u64, max: Option<u64>) -> Vec<u64> {
    let max = max.unwrap_or(n).min(n);
    if n == 0 || max == 0 {
        return Vec::new();
    }
    // `max <= n`, so `n == 1` implies `max == 1` here.
    if max == 1 {
        return vec![1];
    }
    let mut result = vec![1];
    for (p, exponent) in super::factor_exp(n) {
        let count = result.len();
        let mut multiple = 1;
        for _ in 0..exponent {
            multiple *= p;
            for i in 0..count {
                let divisor = result[i] * multiple;
                if divisor <= max {
                    result.push(divisor);
                }
            }
        }
    }
    result.sort_unstable();
    result
}
