/// Return `n` with bit `index` set to zero.
///
/// Return `n` if `index >= 64`, since those bits are already zero.
/// MPU 0.75 lists `bitclear` in `TODO`; it has no reference C function.
///
/// ```
/// use math_prime_util::bitclear;
/// assert_eq!(bitclear(10, 1), 8);
/// assert_eq!(bitclear(10, 64), 10);
/// ```
pub fn bitclear(n: u64, index: u64) -> u64 {
    if index >= 64 {
        return n;
    }
    n & !(1 << index)
}

#[cfg(test)]
mod tests {
    use super::bitclear;

    #[test]
    fn positions_and_bounds() {
        for index in 0..64 {
            assert_eq!(bitclear(u64::MAX, index), u64::MAX ^ (1 << index));
            assert_eq!(bitclear(0, index), 0);
        }
        for index in [64, 65, u64::MAX] {
            assert_eq!(bitclear(42, index), 42);
        }
    }
}
