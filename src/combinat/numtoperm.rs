/// Returns the permutation of `0..n` at the zero-based lexicographic rank.
///
/// The rank wraps modulo `n!`. An empty input has one empty permutation.
/// Port of `num_to_perm` in MPU 0.75 `util.c`. It uses factorial digits
/// and shifts the selected entry into each position.
///
/// ```
/// assert_eq!(math_prime_util::numtoperm(5, 15), [0, 3, 2, 4, 1]);
/// assert_eq!(math_prime_util::numtoperm(3, 6), [0, 1, 2]);
/// ```
pub fn numtoperm(n: usize, mut rank: u64) -> Vec<usize> {
    let mut result: Vec<_> = (0..n).collect();
    if n < 2 {
        return result;
    }
    // 20! fits u64 and 21! does not. Skip the fixed leading entries.
    let mut f = 1_u64;
    let mut digits = 1_usize;
    while digits < n - 1 {
        let Some(next) = f.checked_mul(u64::try_from(digits + 1).expect("factor <= 21")) else {
            break;
        };
        digits += 1;
        f = next;
    }
    if let Some(period) = f.checked_mul(u64::try_from(n).unwrap_or(u64::MAX)) {
        rank %= period;
    }
    for i in n - 1 - digits..n - 1 {
        let offset = usize::try_from(rank / f).expect("offset <= 20");
        rank %= f;
        f /= u64::try_from(n - i - 1).expect("factor <= 20");
        if offset > 0 {
            result[i..=i + offset].rotate_right(1);
        }
    }
    result
}
