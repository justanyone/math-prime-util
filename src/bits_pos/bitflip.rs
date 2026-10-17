/// Return `n` with bit `index` inverted.
///
/// Return `None` if `index >= 64`: the result cannot fit in a `u64`.
/// MPU 0.75 lists `bitflip` in `TODO`; it has no reference C function.
///
/// ```
/// use math_prime_util::bitflip;
/// assert_eq!(bitflip(10, 1), Some(8));
/// assert_eq!(bitflip(10, 0), Some(11));
/// assert_eq!(bitflip(10, 64), None);
/// ```
pub fn bitflip(n: u64, index: u64) -> Option<u64> {
    if index >= 64 {
        return None;
    }
    Some(n ^ (1 << index))
}

#[cfg(test)]
mod tests {
    use super::bitflip;

    #[test]
    fn positions_and_bounds() {
        for index in 0..64 {
            assert_eq!(bitflip(0, index), Some(1 << index));
            assert_eq!(bitflip(u64::MAX, index), Some(u64::MAX ^ (1 << index)));
        }
        for index in [64, 65, u64::MAX] {
            assert_eq!(bitflip(42, index), None);
        }
    }
}
