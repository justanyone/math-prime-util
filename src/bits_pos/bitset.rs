/// Return `n` with bit `index` set to one.
///
/// Return `None` if `index >= 64`: the result cannot fit in a `u64`.
/// MPU 0.75 lists `bitset` in `TODO`; it has no reference C function.
///
/// ```
/// use math_prime_util::bitset;
/// assert_eq!(bitset(8, 1), Some(10));
/// assert_eq!(bitset(0, 64), None);
/// ```
pub fn bitset(n: u64, index: u64) -> Option<u64> {
    if index >= 64 {
        return None;
    }
    Some(n | (1 << index))
}

#[cfg(test)]
mod tests {
    use super::bitset;

    #[test]
    fn positions_and_bounds() {
        for index in 0..64 {
            assert_eq!(bitset(0, index), Some(1 << index));
            assert_eq!(bitset(u64::MAX, index), Some(u64::MAX));
        }
        for index in [64, 65, u64::MAX] {
            assert_eq!(bitset(0, index), None);
            assert_eq!(bitset(u64::MAX, index), None);
        }
    }
}
