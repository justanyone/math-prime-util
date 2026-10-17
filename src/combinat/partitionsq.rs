/// Number of partitions of `n` into distinct parts, or `None` on `u64` overflow.
///
/// Port of `npartitionsq` in `util.c` (MPU 0.75). The method is the pentagonal
/// number recurrence with the Euler `E(x^2)` correction. On a 64-bit word the
/// C code returns 0 (overflow) for `n` above 791.
///
/// ```
/// assert_eq!(math_prime_util::partitionsq(5), Some(3));
/// assert_eq!(math_prime_util::partitionsq(792), None);
/// ```
pub fn partitionsq(n: u64) -> Option<u64> {
    if n <= 3 {
        return Some(1 + u64::from(n == 3));
    }
    if n > 791 {
        return None;
    }
    let limit = usize::try_from(n).ok()?;
    let root = usize::try_from(crate::sqrtint(n + 1)).ok()?;
    let mut pent = vec![0_u64; 2 * root + 2];
    pent[1] = 1;
    for index in 1..=root {
        let width = u64::try_from(index).unwrap_or(0);
        pent[2 * index] = width * (3 * width + 1) / 2;
        pent[2 * index + 1] = (width + 1) * (3 * width + 2) / 2;
    }
    let mut part = vec![0_u64; limit + 1];
    part[0] = 1;
    for value in 1..=limit {
        let value_u = u64::try_from(value).ok()?;
        let mut sum = 0_i128;
        let mut index = 1_usize;
        while index < pent.len() && pent[index] <= value_u {
            let previous = value - usize::try_from(pent[index]).ok()?;
            if (index + 1) & 2 != 0 {
                sum += i128::from(part[previous]);
            } else {
                sum -= i128::from(part[previous]);
            }
            index += 1;
        }
        // Non-zero only at double pentagonals. The sign is opposite the
        // recurrence sign at that index. See `npartitionsq` in `util.c`.
        if value % 2 == 0 {
            let half = value_u / 2;
            let mut index = 1_usize;
            while index < pent.len() && pent[index] <= half {
                if pent[index] == half {
                    if (index + 1) & 2 != 0 {
                        sum -= 1;
                    } else {
                        sum += 1;
                    }
                    break;
                }
                index += 1;
            }
        }
        part[value] = u64::try_from(sum).ok()?;
    }
    Some(part[limit])
}
