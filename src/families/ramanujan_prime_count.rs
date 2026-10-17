use super::ramanujan_primes::{SMALL, range};

/// Count the Ramanujan primes at most `n`.
///
/// Port of MPU 0.75 `ramanujan_primes.c`, `ramanujan_prime_count` and
/// `_ramanujan_prime_window`. Powers of two use the C lookup table.
///
/// ```
/// assert_eq!(math_prime_util::ramanujan_prime_count(1000), 72);
/// ```
pub fn ramanujan_prime_count(n: u64) -> u64 {
    const POW2: [u64; 57] = [0, 1, 1, 1, 2, 4, 7, 13, 23, 42, 75, 137, 255, 463, 872, 1612, 3030, 5706, 10749, 20387, 38635, 73584, 140336, 268216, 513705, 985818, 1894120, 3645744, 7027290, 13561906, 26207278, 50697533, 98182656, 190335585, 369323301, 717267167, 1394192236, 2712103833, 5279763823, 10285641777, 20051180846, 39113482639, 76344462797, 149100679004, 291354668495, 569630404447, 1114251967767, 2180634225768, 4269555883751, 8363243713305, 16388947026629, 32129520311897, 63012603695171, 123627200537929, 242637500756376, 476379740340417, 935609435783647];
    if n.is_power_of_two() && n.ilog2() <= 56 { return POW2[n.ilog2() as usize]; }
    if n <= 241 { return SMALL.partition_point(|&p| p <= n) as u64; }
    let v = crate::prime_count(n) - crate::prime_count(n / 2);
    let mut window = if n > 1_000_000_000 { 320 } else { 20 };
    loop {
        let first = v.saturating_sub(window).max(1);
        let values = range(first, v + window);
        if values[0] < n && values[values.len()-1] > n {
            return first + values.partition_point(|&p| p <= n) as u64 - 1;
        }
        window *= 2;
    }
}
