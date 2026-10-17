/// Returns the least Pillai witness, or zero if there is none.
///
/// A witness `v` has `v! = -1 (mod n)` and `n % v != 1`.
/// Port of `pillai_v` in MPU 0.75 `util.c`.
///
/// ```
/// assert_eq!(math_prime_util::is_pillai(23), 14);
/// assert_eq!(math_prime_util::is_pillai(24), 0);
/// ```
pub fn is_pillai(n: u64) -> u64 {
    if n < 23 || !matches!(n % 30, 1 | 7 | 11 | 13 | 17 | 19 | 23 | 29) || n % 7 == 0 {
        return 0;
    }
    let mut fac = 5040 % n;
    for v in 8..n - 1 {
        if fac == 0 {
            break;
        }
        fac = crate::mulmod(fac, v, n);
        if fac == n - 1 && n % v != 1 {
            return v;
        }
    }
    0
}
