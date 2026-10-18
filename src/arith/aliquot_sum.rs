use super::factor_powers;

/// Return the sum of the proper divisors of `n`.
/// Return `None` if the result exceeds u64.
///
/// Port of MPU 0.75 `factor.c::aliquot_sum`. Use its prime-power shortcut
/// and product of geometric sums. Wide sums cover the Perl fallback path.
///
/// ```
/// assert_eq!(math_prime_util::aliquot_sum(12), Some(16));
/// ```
pub fn aliquot_sum(n: u64) -> Option<u64> {
    if n <= 1 {
        return Some(0);
    }
    let factors = factor_powers(n);
    if let [(p, e)] = factors.as_slice() {
        let (mut power, mut sum) = (1, 1);
        for _ in 1..*e {
            power *= p;
            sum += power;
        }
        return Some(sum);
    }
    let mut product = 1_u128;
    for (p, e) in factors {
        let (mut power, mut sum) = (1_u128, 1_u128);
        for _ in 0..e {
            power *= u128::from(p);
            sum += power;
        }
        product *= sum;
    }
    u64::try_from(product - u128::from(n)).ok()
}
