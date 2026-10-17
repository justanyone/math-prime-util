/// Return the lowest zero-bit position at or above `start` in `n`.
///
/// Positions above 63 are zero. Thus `start >= 64` returns `start`.
/// If all bits from `start` through 63 are one, return 64.
/// MPU 0.75 proposes `bitscan0` in `TODO` via `valuation(n + 1, 2)`
/// for a zero start. It has no reference C function for this operation.
///
/// ```
/// use math_prime_util::bitscan0;
/// assert_eq!(bitscan0(0b1011, 0), 2);
/// assert_eq!(bitscan0(u64::MAX, 0), 64);
/// assert_eq!(bitscan0(0, 100), 100);
/// ```
pub fn bitscan0(n: u64, start: u64) -> u64 {
    if start >= 64 {
        return start;
    }
    start + u64::from((!(n >> start)).trailing_zeros()).min(64 - start)
}

#[cfg(test)]
mod tests {
    use super::bitscan0;

    #[test]
    fn positions_and_bounds() {
        for start in 0..64 {
            assert_eq!(bitscan0(0, start), start);
            assert_eq!(bitscan0(u64::MAX, start), 64);
            for index in start..64 {
                assert_eq!(bitscan0(u64::MAX ^ (1 << index), start), index);
            }
        }
        for start in [64, 65, u64::MAX] {
            assert_eq!(bitscan0(u64::MAX, start), start);
        }
    }
}
