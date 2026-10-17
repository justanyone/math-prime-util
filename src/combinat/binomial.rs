/// Returns the binomial coefficient, or `None` on overflow.
///
/// Port of `binomial` in MPU 0.75 `util.c`. The product uses gcd
/// cancellation before a multiplication that can overflow.
///
/// ```
/// assert_eq!(math_prime_util::binomial(20, 15), Some(15504));
/// ```
pub fn binomial(mut n: u64, mut k: u64) -> Option<u64> {
    if k == 0 {
        return Some(1);
    }
    if k == 1 {
        return Some(n);
    }
    if k >= n {
        return Some(u64::from(k == n));
    }
    k = k.min(n - k);
    if (n >= 73 && k >= 25) || (n >= 92 && k >= 19) || (n >= 131 && k >= 15) {
        return None;
    }
    let mut r = 1_u64;
    for d in 1..=k {
        if r >= u64::MAX / n {
            let g = crate::gcd(n, d);
            let nr = n / g;
            let dr = d / g;
            let g = crate::gcd(r, dr);
            r = (r / g).checked_mul(nr)? / (dr / g);
        } else {
            r = r * n / d;
        }
        n -= 1;
    }
    Some(r)
}
