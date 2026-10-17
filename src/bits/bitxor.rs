/// Return the bitwise exclusive OR of two 64-bit integers.
///
/// The result uses all 64 bits, including the highest bit.
/// MPU 0.75 lists `bitxor` in `reference/mpu/TODO` only.
/// No C, `Util.pm`, or `PP.pm` function exists in this reference checkout.
/// This implements the requested native integer operation directly.
///
/// ```
/// use math_prime_util::bitxor;
/// assert_eq!(bitxor(0b1100, 0b1010), 0b0110);
/// ```
#[inline]
pub const fn bitxor(a: u64, b: u64) -> u64 {
    a ^ b
}

#[cfg(test)]
mod tests {
    use super::bitxor;

    #[test]
    fn bits_ops1_bitxor_edges() {
        assert_eq!(bitxor(0b1100, 0b1010), 0b0110);
        assert_eq!(bitxor(0, u64::MAX), u64::MAX);
        assert_eq!(bitxor(u64::MAX, u64::MAX), 0);
        assert_eq!(bitxor(1 << 63, 1), (1 << 63) | 1);
    }
}
