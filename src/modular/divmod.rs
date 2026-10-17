use super::{invmod, mulmod};

/// `a / b mod n`, or `None` if `n == 0`. Returns `Some(0)` when `b` has no
/// inverse mod `n`, as MPU does.
///
/// Ported from MPU's `util.c` (`divmod`), which calls `modinverse` and
/// `mulmod`.
///
/// ```
/// use math_prime_util::divmod;
/// assert_eq!(divmod(6, 3, 7), Some(2));
/// assert_eq!(divmod(1, 2, 0), None);
/// assert_eq!(divmod(1, 2, 4), Some(0));
/// ```
pub fn divmod(a: u64, b: u64, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    match invmod(b, n) {
        Some(binv) => mulmod(a, binv, n),
        None => Some(0),
    }
}

#[cfg(test)]
mod tests {
    use super::divmod;

    #[test]
    fn divmod_cases() {
        assert_eq!(divmod(6, 3, 7), Some(2));
        assert_eq!(divmod(1, 2, 0), None);
        assert_eq!(divmod(1, 2, 4), Some(0));
        assert_eq!(divmod(5, 1, 1), Some(0));
    }
}
