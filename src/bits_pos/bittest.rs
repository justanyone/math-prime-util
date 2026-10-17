/// Return whether bit `index` of `n` is one.
///
/// Return `false` if `index >= 64`, since those bits are zero.
/// MPU 0.75 lists `bittest` in `TODO`; it has no reference C function.
///
/// ```
/// use math_prime_util::bittest;
/// assert!(bittest(10, 3));
/// assert!(!bittest(10, 0));
/// assert!(!bittest(10, 64));
/// ```
pub fn bittest(n: u64, index: u64) -> bool {
    index < 64 && n & (1 << index) != 0
}

#[cfg(test)]
mod tests {
    use super::bittest;

    #[test]
    fn positions_and_bounds() {
        for index in 0..64 {
            assert!(bittest(1 << index, index));
            assert!(!bittest(0, index));
            assert!(!bittest(u64::MAX ^ (1 << index), index));
        }
        for index in [64, 65, u64::MAX] {
            assert!(!bittest(u64::MAX, index));
        }
    }
}
