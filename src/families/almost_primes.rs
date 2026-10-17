use crate::{primes, rootint, semi_primes, sqrtint};
use super::almost_prime_count::max_nth;

// almost_primes.c: _genkap. Nondecreasing prime factors avoid duplicates.
fn generate(k: u64, lo: u64, hi: u64, m: u64, begin: u64, list: &mut Vec<u64>) {
    if k == 1 {
        let start = (lo / m + u64::from(lo % m != 0)).max(begin);
        list.extend(primes(start, hi/m).into_iter().map(|p| m*p));
    } else {
        for p in primes(begin, rootint(hi/m, u32::try_from(k).expect("k is below 64"))) {
            let t = m*p;
            if lo/t + u64::from(lo%t != 0) <= hi/t {
                generate(k-1, lo, hi, t, p, list);
            }
        }
    }
}

// almost_primes.c: range_almost_prime_sieve. Count prime powers in a window.
fn sieve(k: u64, lo: u64, hi: u64) -> Vec<u64> {
    let factors = primes(2, sqrtint(hi));
    let mut result = Vec::new();
    let mut start = lo;
    loop {
        let end = start.saturating_add(65535).min(hi);
        let mut remaining: Vec<_> = (start..=end).collect();
        let mut counts = vec![0_u64; remaining.len()];
        for &p in &factors {
            let offset = (p - start%p)%p;
            if offset > end-start { continue; }
            let mut i = usize::try_from(offset).expect("window offset fits usize");
            while i < remaining.len() {
                while remaining[i]%p == 0 { remaining[i] /= p; counts[i] += 1; }
                let Ok(step) = usize::try_from(p) else { break; };
                i += step;
            }
        }
        for (i, (&rest, &count)) in remaining.iter().zip(&counts).enumerate() {
            if count + u64::from(rest > 1) == k {
                result.push(start + u64::try_from(i).expect("window index fits u64"));
            }
        }
        if end == hi { break; }
        start = end+1;
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
    if k >= 64 { return Vec::new(); }
    lo = lo.max(1_u64 << k);
    hi = hi.min(max_nth(k));
    if lo > hi { return Vec::new(); }
    if k == 0 { return vec![1]; }
    if k == 1 { return primes(lo,hi); }
    if k == 2 { return semi_primes(lo,hi); }
    if (hi >= 1_000_000_000_000 && hi-lo <= 5_000_000)
        || (hi >= 10_000_000_000_000 && hi-lo <= 200_000_000)
        || (hi >= 100_000_000_000_000 && hi-lo <= 400_000_000) {
        return sieve(k,lo,hi);
    }
    let mut result = Vec::new();
    generate(k,lo,hi,1,2,&mut result);
    result.sort_unstable();
    result
}
