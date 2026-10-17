use crate::{is_prime, primes, sqrtint};

/// Return the smaller primes in all Goldbach pairs for `n`, in order.
/// Return an empty list if no pair exists.
///
/// Port of MPU 0.75 `goldbach.c`, `goldbach_pairs` and `sieve_pairs`.
/// The two segments sieve both members of each pair.
///
/// ```
/// use math_prime_util::goldbach_pairs;
/// assert_eq!(goldbach_pairs(22), vec![3, 5, 11]);
/// assert!(goldbach_pairs(11).is_empty());
/// ```
pub fn goldbach_pairs(n: u64) -> Vec<u64> {
    let mut pairs = Vec::new();
    visit_pairs(n, |p| pairs.push(p));
    // The double sieve visits the larger primes in ascending order.
    let small = pairs.iter().take_while(|&&p| p < 11).count();
    pairs[small..].reverse();
    pairs
}

pub(super) fn visit_pairs(n: u64, mut visit: impl FnMut(u64)) {
    if n < 4 {
        return;
    }
    if n & 1 != 0 || n == 4 {
        if is_prime(n - 2) != 0 {
            visit(2);
        }
        return;
    }
    for p in [3, 5, 7] {
        if n >= 2 * p && is_prime(n - p) != 0 {
            visit(p);
        }
    }
    if n < 22 {
        return;
    }
    let base = primes(2, sqrtint(n - 11));
    let mut lo = n / 2;
    let end = n - 11;
    loop {
        let hi = lo.saturating_add(32_767).min(end);
        let high = sieve_segment(lo, hi, &base);
        let low = sieve_segment(n - hi, n - lo, &base);
        for (i, &prime) in high.iter().enumerate() {
            if prime && low[high.len() - 1 - i] {
                visit(n - (lo + u64::try_from(i).expect("segment index fits u64")));
            }
        }
        if hi == end {
            break;
        }
        lo = hi + 1;
    }
}

fn sieve_segment(lo: u64, hi: u64, base: &[u64]) -> Vec<bool> {
    let mut sieve = vec![true; usize::try_from(hi - lo + 1).expect("segment length fits usize")];
    for &p in base {
        if p > hi / p {
            break;
        }
        let offset = (p - lo % p) % p;
        if offset > hi - lo {
            continue;
        }
        let first = lo + offset;
        let mut multiple = first.max(p * p);
        while multiple <= hi {
            sieve[usize::try_from(multiple - lo).expect("segment index fits usize")] = false;
            if hi - multiple < p {
                break;
            }
            multiple += p;
        }
    }
    sieve
}

#[cfg(test)]
mod tests {
    use super::sieve_segment;

    #[test]
    fn segment_boundaries() {
        assert_eq!(sieve_segment(11, 11, &[2, 3]), vec![true]);
        assert_eq!(sieve_segment(12, 12, &[2, 3]), vec![false]);
        let lo = u64::MAX - 40;
        let high = sieve_segment(lo, u64::MAX, &[2, 3, 5]);
        for (offset, &unmarked) in high.iter().enumerate() {
            let n = lo + u64::try_from(offset).unwrap();
            assert_eq!(unmarked, [2, 3, 5].iter().all(|p| n % p != 0));
        }
    }
}
