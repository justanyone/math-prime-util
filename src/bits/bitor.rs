/// Return the bitwise OR of two 64-bit integers.
///
/// The result uses all 64 bits, including the highest bit.
/// MPU 0.75 lists `bitor` in `reference/mpu/TODO` only.
/// No C, `Util.pm`, or `PP.pm` function exists in this reference checkout.
/// This implements the requested native integer operation directly.
///
/// ```
/// use math_prime_util::bitor;
/// assert_eq!(bitor(0b1100, 0b1010), 0b1110);
/// ```
#[inline]
pub const fn bitor(a: u64, b: u64) -> u64 {
    a | b
}

#[cfg(test)]
mod tests {
    use super::bitor;

    #[test]
    fn bits_ops1_bitor_edges() {
        assert_eq!(bitor(0b1100, 0b1010), 0b1110);
        assert_eq!(bitor(0, 0), 0);
        assert_eq!(bitor(0, u64::MAX), u64::MAX);
        assert_eq!(bitor(1 << 63, 1), (1 << 63) | 1);
    }
}
