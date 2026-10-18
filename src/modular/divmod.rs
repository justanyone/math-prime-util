use super::{invmod, mulmod};

/// `a / b mod n`, which is `(a * invmod(b, n)) mod n`.
///
/// Returns `None` when `n == 0` or when `b` has no inverse modulo `n`
/// (for example `b` is a multiple of `n > 1`). `n == 1` yields `Some(0)`.
///
/// Ported from MPU `util.c` `divmod` (`modinverse` then `mulmod`). The public
/// wrapper in `XS.xs` `divmod` returns `undef` when that inverse is 0; this
/// function returns `None` for the same inputs.
///
/// ```
/// use math_prime_util::divmod;
/// assert_eq!(divmod(6, 3, 7), Some(2));
/// assert_eq!(divmod(1, 2, 0), None);
/// assert_eq!(divmod(5, 0, 7), None);
/// assert_eq!(divmod(1, 2, 4), None);
/// ```
pub fn divmod(a: u64, b: u64, n: u64) -> Option<u64> {
    invmod(b, n).and_then(|binv| mulmod(a, binv, n))
}

#[cfg(test)]
mod tests {
    use super::divmod;

    #[test]
    fn divmod_cases() {
        assert_eq!(divmod(6, 3, 7), Some(2));
        assert_eq!(divmod(1, 2, 0), None);
        // No inverse: the XS wrapper returns undef. `Some(0)` would still
        // satisfy the old `util.c` return and hide this.
        assert_eq!(divmod(1, 2, 4), None);
        assert_eq!(divmod(5, 0, 7), None);
        assert_eq!(divmod(5, 7, 7), None);
        assert_eq!(divmod(5, 1, 1), Some(0));
    }
}
