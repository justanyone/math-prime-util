/// Number of unrestricted partitions of `n`, or `None` when it does not fit in `u64`.
///
/// Port of `npartitions` in `util.c` (MPU 0.75). The method is the pentagonal
/// number recurrence. On a 64-bit word the C code returns 0 (overflow) for
/// `n` above 416.
///
/// ```
/// assert_eq!(math_prime_util::partitions(5), Some(7));
/// assert_eq!(math_prime_util::partitions(417), None);
/// ```
pub fn partitions(n: u64) -> Option<u64> {
    if n <= 3 {
        return Some(if n == 0 { 1 } else { n });
    }
    if n > 416 {
        return None;
    }
    let limit = usize::try_from(n).ok()?;
    let root = usize::try_from(crate::sqrtint(n + 1)).ok()?;
    let pent = generalized_pentagonals(root);
    let mut part = vec![0_u64; limit + 1];
    part[0] = 1;
    for value in 1..=limit {
        let mut sum = 0_i128;
        let mut index = 1_usize;
        let value_u = u64::try_from(value).ok()?;
        while index < pent.len() && pent[index] <= value_u {
            let previous = value - usize::try_from(pent[index]).ok()?;
            if (index + 1) & 2 != 0 {
                sum += i128::from(part[previous]);
            } else {
                sum -= i128::from(part[previous]);
            }
            index += 1;
        }
        part[value] = u64::try_from(sum).ok()?;
    }
    Some(part[limit])
}

/// Generalized pentagonals `k(3k-1)/2` for `k = 1, -1, 2, -2, ...`.
///
/// Same table fill as `npartitions` in `util.c`: `pent[1] = 1` and then
/// `pent[2i] = i(3i+1)/2`, `pent[2i+1] = (i+1)(3i+2)/2`.
fn generalized_pentagonals(root: usize) -> Vec<u64> {
    let mut pent = vec![0_u64; 2 * root + 2];
    pent[1] = 1;
    for index in 1..=root {
        let width = u64::try_from(index).unwrap_or(0);
        pent[2 * index] = width * (3 * width + 1) / 2;
        pent[2 * index + 1] = (width + 1) * (3 * width + 2) / 2;
    }
    pent
}
