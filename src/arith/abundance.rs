use super::aliquot_sum;

/// Return the aliquot sum minus n as a native signed word.
/// MPU converts the difference to i64 with wraparound.
/// Return `None` if the aliquot sum exceeds u64.
///
/// Port of MPU 0.75 `XS.xs::abundance` and `factor.c::aliquot_sum`.
///
/// ```
/// assert_eq!(math_prime_util::abundance(12), Some(4));
/// ```
pub fn abundance(n: u64) -> Option<i64> {
    let sum = aliquot_sum(n)?;
    Some(i64::from_ne_bytes(sum.wrapping_sub(n).to_ne_bytes()))
}
