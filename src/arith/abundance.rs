use super::aliquot_sum;

/// Return the aliquot sum minus n as a native signed word.
/// Return `None` if the aliquot sum exceeds u64 or the difference does not
/// fit i64. MPU 0.75 wraps the difference instead
/// (docs/upstream-findings.md, finding 2).
///
/// Port of MPU 0.75 `XS.xs::abundance` and `factor.c::aliquot_sum`.
///
/// ```
/// assert_eq!(math_prime_util::abundance(12), Some(4));
/// ```
pub fn abundance(n: u64) -> Option<i64> {
    let sum = aliquot_sum(n)?;
    i64::try_from(i128::from(sum) - i128::from(n)).ok()
}
