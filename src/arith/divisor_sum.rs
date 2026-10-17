use super::factor_powers;

pub(super) fn sigma(n: u64, k: u64) -> Option<u128> {
    if n <= 1 {
        return Some(u128::from(n));
    }
    let mut product = 1_u128;
    for (p, e) in factor_powers(n) {
        if k == 0 {
            product *= u128::from(e + 1);
            continue;
        }
        let pk = u128::from(p).checked_pow(u32::try_from(k).ok()?)?;
        let (mut power, mut sum) = (1_u128, 1_u128);
        for _ in 0..e {
            power = power.checked_mul(pk)?;
            sum = sum.checked_add(power)?;
        }
        product = product.checked_mul(sum)?;
    }
    Some(product)
}

/// Return the sum of the k-th powers of the divisors of n.
/// Return `None` if the result exceeds u64.
///
/// Port of MPU 0.75 `factor.c::divisor_sum`.
///
/// ```
/// assert_eq!(math_prime_util::divisor_sum(12, 1), Some(28));
/// ```
pub fn divisor_sum(n: u64, k: u64) -> Option<u64> {
    u64::try_from(sigma(n, k)?).ok()
}

