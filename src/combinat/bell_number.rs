/// Returns a Bell number, or `None` on overflow.
///
/// Port of `bell_number` in MPU 0.75 `util.c`. This uses the same Bell
/// triangle and the 64-bit bound of 26.
///
/// ```
/// assert_eq!(math_prime_util::bell_number(5), Some(52));
/// ```
pub fn bell_number(n: u64) -> Option<u64> {
    if n <= 1 {
        return Some(1);
    }
    if n >= 26 {
        return None;
    }
    // The last row has entries larger than its first entry.
    let mut row = [0_u128; 27];
    let mut next = [0_u128; 27];
    row[0] = 1;
    for i in 1..=usize::try_from(n).ok()? {
        next[0] = row[i - 1];
        for j in 0..i {
            next[j + 1] = next[j] + row[j];
        }
        row[..=i].copy_from_slice(&next[..=i]);
    }
    u64::try_from(row[0]).ok()
}
