/// Returns the number of ordered set partitions, or `None` on overflow.
///
/// Port of `fubini` and its `stirling2` sum in MPU 0.75 `util.c`.
/// At the C bound of 16, uses the binomial recurrence from `_add_fubini`
/// in `lib/Math/Prime/Util/PP.pm`, as the XS dispatcher does.
/// The fallback uses `u128` intermediates and fits `u64` through 18.
///
/// ```
/// use math_prime_util::fubini;
/// assert_eq!(fubini(4), Some(75));
/// assert_eq!(fubini(18), Some(3_385_534_663_256_845_323));
/// assert_eq!(fubini(19), None);
/// ```
pub fn fubini(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(1);
    }
    if n >= 19 {
        return None;
    }
    if n >= 16 {
        return fubini_fallback(usize::try_from(n).ok()?);
    }
    let n = u32::try_from(n).ok()?;
    let mut sum = 1_u64;
    for k in 2..=n {
        // util.c: stirling2(n, k) = sum((-1)^(k-j) C(k,j) j^n) / k!.
        let mut binomial = 1_i128;
        let mut stirling_sum = 0_i128;
        for j in 1..=k {
            binomial = binomial * i128::from(k - j + 1) / i128::from(j);
            let term = binomial * i128::from(j).pow(n);
            if (k - j) & 1 == 1 {
                stirling_sum -= term;
            } else {
                stirling_sum += term;
            }
        }
        let factorial = super::factorial(u64::from(k))?;
        let stirling = u64::try_from(stirling_sum / i128::from(factorial)).ok()?;
        sum += factorial * stirling;
    }
    Some(sum)
}

fn fubini_fallback(n: usize) -> Option<u64> {
    let mut values = [0_u128; 19];
    values[..5].copy_from_slice(&[1, 1, 3, 13, 75]);
    for degree in 5..=n {
        let mut binomial = 1_u128;
        let mut sum = 0;
        for (i, &value) in values[..degree].iter().enumerate() {
            sum += binomial * value;
            binomial = binomial * (degree - i) as u128 / (i + 1) as u128;
        }
        values[degree] = sum;
    }
    u64::try_from(values[n]).ok()
}
