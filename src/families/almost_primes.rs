use super::almost_prime_count::max_nth;
use crate::{primes, rootint, semi_primes, sqrtint};

// almost_primes.c: _genkap. Nondecreasing prime factors avoid duplicates.
fn generate(k: u64, lo: u64, hi: u64, m: u64, begin: u64, list: &mut Vec<u64>) {
    if k == 1 {
        let start = (lo / m + u64::from(lo % m != 0)).max(begin);
        list.extend(primes(start, hi / m).into_iter().map(|p| m * p));
    } else {
        for p in primes(
            begin,
            rootint(hi / m, u32::try_from(k).expect("k is below 64")),
        ) {
            let t = m * p;
            if lo / t + u64::from(lo % t != 0) <= hi / t {
                generate(k - 1, lo, hi, t, p, list);
            }
        }
    }
}

// almost_primes.c: range_almost_prime_sieve. Count prime powers in a window.
fn sieve(k: u64, lo: u64, hi: u64) -> Vec<u64> {
    // Reduce the factor count when every result has a forced power of two.
    let mut reduced = k;
    let mut top = hi;
    super::almost_prime_count::reduce(&mut reduced, &mut top);
    let r = k - reduced;
    if r > 0 {
        return almost_primes(reduced, (lo >> r) + u64::from(lo % (1_u64 << r) != 0), top)
            .into_iter()
            .map(|n| n << r)
            .collect();
    }
    let mut result = Vec::new();
    let mut start = lo;
    loop {
        let end = start.saturating_add(262143).min(hi);
        let mut products: Vec<u64> = (start..=end).map(|n| 1_u64 << n.trailing_zeros()).collect();
        let mut counts: Vec<u64> = (start..=end)
            .map(|n| u64::from(n.trailing_zeros()))
            .collect();
        for p in primes(3, sqrtint(end / (1_u64 << (k - 2)))) {
            let mut power = p;
            loop {
                let offset = (power - start % power) % power;
                if offset <= end - start {
                    let mut i = usize::try_from(offset).expect("window offset fits usize");
                    while i < products.len() {
                        products[i] *= p;
                        counts[i] += 1;
                        let Ok(step) = usize::try_from(power) else {
                            break;
                        };
                        i += step;
                    }
                }
                let Some(next) = power.checked_mul(p) else {
                    break;
                };
                if next > end {
                    break;
                }
                power = next;
            }
        }
        for (i, (&product, &count)) in products.iter().zip(&counts).enumerate() {
            let n = start + u64::try_from(i).expect("window index fits u64");
            if count + u64::from(product < n) == k {
                result.push(n);
            }
        }
        if end == hi {
            break;
        }
        start = end + 1;
    }
    result
}

/// List the integers in `lo..=hi` with `k` prime factors, with repetition.
///
/// Port of MPU 0.75 `almost_primes.c`: `generate_almost_primes`, `_genkap`,
/// and `range_almost_prime_sieve`.
/// ```
/// use math_prime_util::almost_primes;
/// assert_eq!(almost_primes(3, 1, 30), vec![8, 12, 18, 20, 27, 28, 30]);
/// ```
pub fn almost_primes(k: u64, mut lo: u64, mut hi: u64) -> Vec<u64> {
    if k >= 64 {
        return Vec::new();
    }
    lo = lo.max(1_u64 << k);
    hi = hi.min(max_nth(k));
    if lo > hi {
        return Vec::new();
    }
    if k == 0 {
        return vec![1];
    }
    if k == 1 {
        return primes(lo, hi);
    }
    if k == 2 {
        return semi_primes(lo, hi);
    }
    if (hi >= 1_000_000_000_000 && hi - lo <= 5_000_000)
        || (hi >= 10_000_000_000_000 && hi - lo <= 200_000_000)
        || (hi >= 100_000_000_000_000 && hi - lo <= 400_000_000)
    {
        return sieve(k, lo, hi);
    }
    let mut result = Vec::new();
    generate(k, lo, hi, 1, 2, &mut result);
    result.sort_unstable();
    result
}
