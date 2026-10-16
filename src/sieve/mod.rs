//! Prime enumeration and a shared cache.
//!
//! API targets: MPU `sieve.c`, `cache.c`, and `sieve_cluster.c`.
//! The reference submodule was absent when this implementation was written.
//! Algorithm fidelity to those C files is not yet verified.

use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

const SEGMENT: u64 = 32_768;
const BASE_LIMIT: u64 = 1_000_000;

#[derive(Default)]
struct PrimeCache {
    limit: u64,
    primes: Arc<Vec<u64>>,
}

static CACHE: OnceLock<Mutex<PrimeCache>> = OnceLock::new();

fn cache() -> MutexGuard<'static, PrimeCache> {
    CACHE.get_or_init(|| Mutex::new(PrimeCache::default()))
        .lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn base_primes(limit: u64) -> Arc<Vec<u64>> {
    let mut cache = cache();
    if cache.limit < limit {
        let mut result = Vec::new();
        let mut low = 2;
        while low <= limit {
            let high = low.saturating_add(SEGMENT - 1).min(limit);
            let mut composite = vec![false; usize::try_from(high - low + 1).unwrap()];
            for &p in &result {
                if p > high / p { break; }
                mark(&mut composite, low, high, p);
            }
            for n in low..=high {
                if !composite[usize::try_from(n - low).unwrap()] {
                    result.push(n);
                    if n <= high / n { mark(&mut composite, low, high, n); }
                }
            }
            low = high + 1;
        }
        cache.limit = limit;
        cache.primes = Arc::new(result);
    }
    Arc::clone(&cache.primes)
}

fn mark(composite: &mut [bool], low: u64, high: u64, p: u64) {
    let start = u128::from(low).div_ceil(u128::from(p)) * u128::from(p);
    let start = start.max(u128::from(p) * u128::from(p));
    if start > u128::from(high) { return; }
    let mut n = u64::try_from(start).unwrap();
    loop {
        composite[usize::try_from(n - low).unwrap()] = true;
        let Some(next) = n.checked_add(p).filter(|&v| v <= high) else { break; };
        n = next;
    }
}

fn prime(n: u64) -> bool {
    if n < 2 { return false; }
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 { return n == p; }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    // This base set is deterministic over the full u64 domain.
    for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if a % n == 0 { continue; }
        let mut x = crate::powmod(a, d, n).unwrap();
        if x == 1 || x == n - 1 { continue; }
        let mut passed = false;
        for _ in 1..s {
            x = u64::try_from(u128::from(x) * u128::from(x) % u128::from(n)).unwrap();
            if x == n - 1 { passed = true; break; }
        }
        if !passed { return false; }
    }
    true
}

/// Return primes in the inclusive interval.
///
/// MPU target: `sieve.c`, `start_segment_primes` and `next_segment_primes`.
/// Reference verification is pending. Large endpoints use primality tests.
/// ```
/// assert_eq!(math_prime_util::primes(10, 20), vec![11, 13, 17, 19]);
/// ```
pub fn primes(lo: u64, hi: u64) -> Vec<u64> {
    if lo > hi || hi < 2 { return Vec::new(); }
    let bound = crate::sqrtint(hi);
    let base = base_primes(bound.min(BASE_LIMIT));
    let mut result = Vec::new();
    let mut low = lo.max(2);
    loop {
        let high = low.saturating_add(SEGMENT - 1).min(hi);
        let mut composite = vec![false; usize::try_from(high - low + 1).unwrap()];
        for &p in base.iter() {
            if p > high / p { break; }
            mark(&mut composite, low, high, p);
        }
        for n in low..=high {
            if !composite[usize::try_from(n - low).unwrap()] && (bound <= BASE_LIMIT || prime(n)) {
                result.push(n);
            }
        }
        if high == hi { break; }
        low = high + 1;
    }
    result
}

/// Return the first prime strictly above `n`, if it fits in `u64`.
///
/// MPU API: `next_prime`; the C source is not available for verification.
/// ```
/// assert_eq!(math_prime_util::next_prime(7), Some(11));
/// assert_eq!(math_prime_util::next_prime(u64::MAX), None);
/// ```
pub fn next_prime(n: u64) -> Option<u64> {
    if n < 2 { return Some(2); }
    let mut candidate = n.checked_add(1)? | 1;
    loop {
        if prime(candidate) { return Some(candidate); }
        candidate = candidate.checked_add(2)?;
    }
}

/// Return the first prime strictly below `n`.
///
/// MPU API: `prev_prime`; the C source is not available for verification.
/// ```
/// assert_eq!(math_prime_util::prev_prime(7), Some(5));
/// assert_eq!(math_prime_util::prev_prime(2), None);
/// ```
pub fn prev_prime(n: u64) -> Option<u64> {
    if n <= 2 { return None; }
    if n == 3 { return Some(2); }
    let mut candidate = (n - 2) | 1;
    loop {
        if prime(candidate) { return Some(candidate); }
        candidate -= 2;
    }
}

/// Return offsets whose values survive sieving by primes through `depth`.
/// The interval starts at `start` and contains `width` values.
///
/// MPU target: `sieve.c`, `sieve_range`. Reference verification is pending.
/// # Panics
/// Panics if the last value exceeds `u64::MAX`.
/// ```
/// assert_eq!(math_prime_util::sieve_range(45, 10, 5), vec![2, 4, 8]);
/// ```
pub fn sieve_range(start: u64, width: u64, depth: u64) -> Vec<u64> {
    if width == 0 { return Vec::new(); }
    let end = start.checked_add(width - 1).expect("sieve interval overflow");
    if depth >= crate::sqrtint(end) {
        return primes(start, end).into_iter().map(|p| p - start).collect();
    }
    let base = base_primes(depth);
    let mut result = Vec::new();
    let mut low = start;
    loop {
        let high = low.saturating_add(SEGMENT - 1).min(end);
        let mut composite = vec![false; usize::try_from(high - low + 1).unwrap()];
        for &p in base.iter().take_while(|&&p| p <= depth) {
            mark(&mut composite, low, high, p);
        }
        for n in low..=high {
            if n >= 2 && !composite[usize::try_from(n - low).unwrap()] { result.push(n - start); }
        }
        if high == end { break; }
        low = high + 1;
    }
    result
}

/// Return primes `p` in the interval for which each `p + offset` is prime.
/// Zero is implicit. Offsets can also start with zero.
///
/// MPU target: `sieve_cluster.c`, `sieve_prime_cluster`. Verification is pending.
/// # Panics
/// Panics if offsets are not strictly increasing.
/// ```
/// assert_eq!(math_prime_util::sieve_prime_cluster(0, 20, &[2]), vec![3, 5, 11, 17]);
/// ```
pub fn sieve_prime_cluster(lo: u64, hi: u64, offsets: &[u64]) -> Vec<u64> {
    assert!(offsets.windows(2).all(|w| w[0] < w[1]), "offsets must increase");
    // A covered residue class leaves only clusters containing the small prime.
    for q in [2, 3, 5, 7, 11, 13] {
        if (0..q).all(|r| r == 0 || offsets.iter().any(|&d| d % q == r)) {
            return primes(lo, hi.min(q)).into_iter().filter(|&p| {
                offsets.iter().all(|&d| p.checked_add(d).is_some_and(prime))
            }).collect();
        }
    }
    primes(lo, hi).into_iter().filter(|&p| {
        offsets.iter().all(|&d| p.checked_add(d).is_some_and(prime))
    }).collect()
}

// Quotient-table prime sum. This is a fallback, not a verified MPU port.
fn prefix_sum(n: u64) -> u128 {
    if n < 2 { return 0; }
    let root = crate::sqrtint(n);
    let values: Vec<u64> = (1..=root).map(|i| n / i)
        .chain((1..=n / (root + 1)).rev()).collect();
    let mut sums: Vec<u128> = values.iter().map(|&v| u128::from(v) * u128::from(v + 1) / 2 - 1).collect();
    let index = |v: u64| -> usize {
        if v <= root { values.len() - usize::try_from(v).unwrap() }
        else { usize::try_from(n / v - 1).unwrap() }
    };
    for p in 2..=root {
        let before = sums[index(p - 1)];
        if sums[index(p)] == before { continue; }
        for (i, &v) in values.iter().enumerate() {
            if v < p * p { break; }
            sums[i] -= u128::from(p) * (sums[index(v / p)] - before);
        }
    }
    sums[0]
}

/// Sum primes in the inclusive interval. Return `None` on overflow.
///
/// MPU API: `sum_primes`. The quotient-table fallback is not a verified C port.
/// ```
/// assert_eq!(math_prime_util::sum_primes(2, 10), Some(17));
/// ```
pub fn sum_primes(lo: u64, hi: u64) -> Option<u64> {
    if lo > hi { return Some(0); }
    if hi - lo <= 1_000_000 {
        return primes(lo, hi).into_iter().try_fold(0_u64, u64::checked_add);
    }
    u64::try_from(prefix_sum(hi) - prefix_sum(lo.saturating_sub(1))).ok()
}

/// Populate the shared prime cache through `limit`. Repeated calls are safe.
///
/// MPU target: `cache.c`, `get_prime_cache`. Reference verification is pending.
/// ```
/// math_prime_util::prime_precalc(1000);
/// assert_eq!(math_prime_util::primes(990, 1000), vec![991, 997]);
/// ```
pub fn prime_precalc(limit: u64) {
    base_primes(limit);
}

/// Release the shared cache. Active readers retain their snapshots.
///
/// MPU target: `cache.c`, `prime_memfree`. Reference verification is pending.
/// ```
/// math_prime_util::prime_memfree();
/// assert_eq!(math_prime_util::next_prime(10), Some(11));
/// ```
pub fn prime_memfree() {
    *cache() = PrimeCache::default();
}
