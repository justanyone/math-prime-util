use super::factors;
use crate::gcd;

/// Return Carmichael's lambda. Port of `totients.c::carmichael_lambda`.
/// ```
/// assert_eq!(math_prime_util::carmichael_lambda(24), 2);
/// ```
#[allow(clippy::missing_panics_doc)] // The index is bounded by n < 8.
pub fn carmichael_lambda(mut n: u64) -> u64 {
    if n < 8 {
        return [0, 1, 1, 2, 2, 4, 2, 6][usize::try_from(n).unwrap()];
    }
    if n.is_power_of_two() {
        return n >> 2;
    }
    let twos = n.trailing_zeros();
    n >>= twos;
    let initial = if twos == 0 { 1 } else { 1 << (twos - 1 - u32::from(twos > 2)) };
    factors(n).into_iter().fold(initial, |v, (p, e)| {
        let term = p.pow(e - 1) * (p - 1);
        v / gcd(v, term) * term
    })
}

