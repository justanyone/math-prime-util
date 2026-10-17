/// Return the bitwise AND of two 64-bit integers.
///
/// The result uses all 64 bits, including the highest bit.
/// MPU 0.75 lists `bitand` in `reference/mpu/TODO` only.
/// No C, `Util.pm`, or `PP.pm` function exists in this reference checkout.
/// This implements the requested native integer operation directly.
///
/// ```
/// use math_prime_util::bitand;
/// assert_eq!(bitand(0b1100, 0b1010), 0b1000);
/// ```
#[inline]
pub const fn bitand(a: u64, b: u64) -> u64 {
    a & b
}

#[cfg(test)]
mod tests {
    use super::bitand;

    #[test]
    fn bits_ops1_bitand_edges() {
        assert_eq!(bitand(0b1100, 0b1010), 0b1000);
        assert_eq!(bitand(0, u64::MAX), 0);
        assert_eq!(bitand(u64::MAX, u64::MAX), u64::MAX);
        assert_eq!(bitand(1 << 63, u64::MAX), 1 << 63);
    }
}
