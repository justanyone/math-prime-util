use super::goldbach_pairs::visit_pairs;

/// Count the unordered prime pairs whose sum is `n`.
///
/// Port of MPU 0.75 `goldbach.c`, `goldbach_pair_count` and `sieve_pairs`.
///
/// ```
/// use math_prime_util::goldbach_pair_count;
/// assert_eq!(goldbach_pair_count(22), 3);
/// assert_eq!(goldbach_pair_count(3), 0);
/// ```
pub fn goldbach_pair_count(n: u64) -> u64 {
    let mut count = 0;
    visit_pairs(n, |_| count += 1);
    count
}
