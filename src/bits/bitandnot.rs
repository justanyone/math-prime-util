/// Return the bits of `a` that are not set in `b`.
///
/// The result uses all 64 bits, including the highest bit.
/// MPU 0.75 lists `bitandnot` in `reference/mpu/TODO` only.
/// No C, `Util.pm`, or `PP.pm` function exists in this reference checkout.
/// This implements the requested native integer operation directly.
///
/// ```
/// use math_prime_util::bitandnot;
/// assert_eq!(bitandnot(0b1100, 0b1010), 0b0100);
/// ```
#[inline]
pub const fn bitandnot(a: u64, b: u64) -> u64 {
    a & !b
}

#[cfg(test)]
mod tests {
    use super::bitandnot;

    #[test]
    fn bits_ops1_bitandnot_edges() {
        assert_eq!(bitandnot(0b1100, 0b1010), 0b0100);
        assert_eq!(bitandnot(u64::MAX, 0), u64::MAX);
        assert_eq!(bitandnot(u64::MAX, u64::MAX), 0);
        assert_eq!(bitandnot(u64::MAX, 1 << 63), (1 << 63) - 1);
        assert_eq!(bitandnot(0, u64::MAX), 0);
        assert_eq!(bitandnot(1, 0), 1);
        assert_eq!(bitandnot(0, 1), 0);
    }
}
