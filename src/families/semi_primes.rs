use crate::{is_prime, primes_in, sqrtint};

/// Return the semiprimes in the inclusive range `lo..=hi`.
///
/// Port of `semi_primes.c`, `range_semiprime_sieve`, in MPU 0.75.
/// A semiprime is a product of two primes, which may be equal.
///
/// ```
/// use math_prime_util::semi_primes;
/// assert_eq!(semi_primes(4, 15), vec![4, 6, 9, 10, 14, 15]);
/// ```
pub fn semi_primes(lo: u64, hi: u64) -> Vec<u64> {
    let lo = lo.max(4);
    let hi = hi.min(18_446_744_073_709_551_601);
    if lo > hi {
        return Vec::new();
    }
    let mut marks = vec![0_u8; usize::try_from(hi - lo + 1).expect("range fits usize")];
    // Resolve primes before marking. Resolve a product as soon as its second
    // factor is prime. This avoids walking to sqrt(hi) for narrow high ranges.
    let mut resolved: Vec<bool> = (lo..=hi).map(|n| is_prime(n) != 0).collect();
    let mut remaining = resolved.iter().filter(|&&done| !done).count();
    for p in primes_in(2..=sqrtint(hi)) {
        if remaining == 0 {
            break;
        }
        let square = p * p;
        let start = u128::from(lo).div_ceil(u128::from(p)) * u128::from(p);
        let mut n = start.max(u128::from(square));
        while n <= u128::from(hi) {
            let value = u64::try_from(n).expect("marked value fits u64");
            let i = usize::try_from(value - lo).expect("index fits usize");
            if !resolved[i] {
                marks[i] += 1;
                if value >= square.saturating_mul(2) && value % square == 0 {
                    marks[i] += 1;
                }
                if marks[i] > 1 || is_prime(value / p) != 0 {
                    resolved[i] = true;
                    remaining -= 1;
                }
            }
            n += u128::from(p);
        }
    }
    marks
        .into_iter()
        .zip(lo..=hi)
        .filter_map(|(count, n)| (count == 1).then_some(n))
        .collect()
}
