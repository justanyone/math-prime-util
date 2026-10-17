/// Return the 64-bit complement of `n`.
///
/// The result uses all 64 bits, including the highest bit.
/// MPU 0.75 lists `bitnot` in `reference/mpu/TODO` only.
/// No C, `Util.pm`, or `PP.pm` function exists in this reference checkout.
/// This implements the requested native integer operation directly.
///
/// ```
/// use math_prime_util::bitnot;
/// assert_eq!(bitnot(0b1100), u64::MAX - 0b1100);
/// ```
#[inline]
pub const fn bitnot(n: u64) -> u64 {
    !n
}

#[cfg(test)]
mod tests {
    use super::bitnot;

    #[test]
    fn bits_ops1_bitnot_edges() {
        assert_eq!(bitnot(0b1100), u64::MAX - 0b1100);
        assert_eq!(bitnot(0), u64::MAX);
        assert_eq!(bitnot(u64::MAX), 0);
        assert_eq!(bitnot(1 << 63), (1 << 63) - 1);
        assert_eq!(bitnot((1 << 63) - 1), 1 << 63);
    }
}
