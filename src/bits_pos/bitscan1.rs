/// Return the lowest one-bit position at or above `start` in `n`.
///
/// Return `None` if no such bit exists. This includes `start >= 64`.
/// MPU 0.75 proposes `bitscan1` in `TODO` via `valuation(n, 2)`
/// for a zero start. It has no reference C function for this operation.
///
/// ```
/// use math_prime_util::bitscan1;
/// assert_eq!(bitscan1(0b1010, 0), Some(1));
/// assert_eq!(bitscan1(0b1010, 2), Some(3));
/// assert_eq!(bitscan1(0, 0), None);
/// assert_eq!(bitscan1(u64::MAX, 64), None);
/// ```
pub fn bitscan1(n: u64, start: u64) -> Option<u64> {
    if start >= 64 {
        return None;
    }
    let shifted = n >> start;
    if shifted == 0 {
        return None;
    }
    Some(start + u64::from(shifted.trailing_zeros()))
}

#[cfg(test)]
mod tests {
    use super::bitscan1;

    #[test]
    fn positions_and_bounds() {
        for start in 0..64 {
            assert_eq!(bitscan1(0, start), None);
            assert_eq!(bitscan1(u64::MAX, start), Some(start));
            for index in 0..64 {
                let expected = (index >= start).then_some(index);
                assert_eq!(bitscan1(1 << index, start), expected);
            }
        }
        for start in [64, 65, u64::MAX] {
            assert_eq!(bitscan1(u64::MAX, start), None);
        }
    }
}
