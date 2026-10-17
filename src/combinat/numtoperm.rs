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
// The quotient is at most 20, so it fits usize on every target.
#[allow(clippy::cast_possible_truncation)]
pub fn numtoperm(n: usize, mut rank: u64) -> Vec<usize> {
    let mut result: Vec<_> = (0..n).collect();
    if n < 2 {
        return result;
    }
    // 20! fits u64 and 21! does not. Skip the fixed leading entries.
    let mut f = 1_u64;
    let mut digits = 1_usize;
    while digits < n - 1 {
        let Some(next) = f.checked_mul((digits + 1) as u64) else {
            break;
        };
        digits += 1;
        f = next;
    }
    if let Some(period) = f.checked_mul(n as u64) {
        rank %= period;
    }
    for i in n - 1 - digits..n - 1 {
        let offset = (rank / f) as usize;
        rank %= f;
        f /= (n - i - 1) as u64;
        if offset > 0 {
            result[i..=i + offset].rotate_right(1);
        }
    }
    result
}
