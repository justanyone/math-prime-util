use super::factors;

pub(super) const SMALL: [i8; 16] = [-1, 1, -1, -1, 1, -1, 1, -1, -1, 1, 1, -1, -1, -1, 1, 1];

/// Return minus one to the number of prime factors, with multiplicity.
///
/// Port of MPU 0.75 `moebius.c::liouville` and `_small_liouville`.
/// Zero returns minus one, as in MPU.
///
/// ```
/// assert_eq!(math_prime_util::liouville(12), -1);
/// assert_eq!(math_prime_util::liouville(16), 1);
/// ```
pub fn liouville(n: u64) -> i8 {
    if n < 16 {
        // The table guard bounds n below 16 on all supported targets.
        #[allow(clippy::cast_possible_truncation)]
        let index = n as usize;
        return SMALL[index];
    }
    if factors(n).iter().map(|&(_, e)| e).sum::<u32>() % 2 == 0 {
        1
    } else {
        -1
    }
}
