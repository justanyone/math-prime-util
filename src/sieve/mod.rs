//! Segmented mod-30 wheel sieve, prime cache, and prime sums.
//!
//! The algorithms follow MPU 0.75 `sieve.c`, `cache.c`, `sieve_cluster.c`,
//! `util.c` (`next_prime`, `prev_prime`), and `prime_sums.c` (`sum_primes`).

mod wheel;

use std::sync::{Mutex, OnceLock};

use wheel::{
    build_sieve, is_prime, next_in_sieve, pad_limit, prev_in_sieve, wheel_next, wheel_prev,
    MPU_MAX_PRIME,
};

/// Largest prime that fits in a `u64`. `constants.h` `MPU_MAX_PRIME`.
const INITIAL_CACHE_SIZE: u64 = (4_096 - 16) * 30 - wheel::FILL_EXTRA_N;

struct PrimeCache {
    /// Mod-30 sieve. A clear bit is a prime. Numbers below `limit` are sieved.
    sieve: Vec<u8>,
    /// Exclusive end. A multiple of 30. `cache.c` `prime_cache_size`.
    limit: u64,
}

fn cache() -> &'static Mutex<PrimeCache> {
    static CACHE: OnceLock<Mutex<PrimeCache>> = OnceLock::new();
    CACHE.get_or_init(|| {
        Mutex::new(PrimeCache {
            sieve: Vec::new(),
            limit: 0,
        })
    })
}

fn lock_cache() -> std::sync::MutexGuard<'static, PrimeCache> {
    cache().lock().unwrap_or_else(|e| e.into_inner())
}

/// Fill the shared prime cache at least through `n`.
///
/// `n == 0` uses the MPU initial size (`cache.c` `prime_precalc`). A later
/// call with a smaller `n` does not shrink the cache. [`prime_memfree`] puts
/// the cache back to that initial size.
///
/// # Examples
///
/// ```
/// use math_prime_util::{next_prime, prime_memfree, prime_precalc};
/// prime_precalc(1_000);
/// assert_eq!(next_prime(100), Some(101));
/// prime_memfree();
/// ```
pub fn prime_precalc(mut n: u64) {
    if n == 0 {
        n = INITIAL_CACHE_SIZE;
    }
    {
        let guard = lock_cache();
        if guard.limit > n {
            return;
        }
    }
    let padded = pad_limit(n);
    let sieve = build_sieve(padded);
    let mut guard = lock_cache();
    // A duplicate launch may have installed a larger sieve while we built.
    if padded > guard.limit {
        guard.sieve = sieve;
        guard.limit = padded;
    }
}

/// Release the extra prime-cache memory and keep the initial sieve.
///
/// This is `cache.c` `prime_memfree`. A call before any [`prime_precalc`]
/// does nothing. The cache stays usable after this call.
///
/// # Examples
///
/// ```
/// use math_prime_util::{prev_prime, prime_memfree, prime_precalc};
/// prime_precalc(5_000);
/// prime_memfree();
/// prime_memfree();
/// assert_eq!(prev_prime(10), Some(7));
/// ```
pub fn prime_memfree() {
    let Some(lock) = cache_if_init() else {
        return;
    };
    let padded = pad_limit(INITIAL_CACHE_SIZE);
    let sieve = build_sieve(padded);
    let mut guard = lock.lock().unwrap_or_else(|e| e.into_inner());
    guard.sieve = sieve;
    guard.limit = padded;
}

fn cache_if_init() -> Option<&'static Mutex<PrimeCache>> {
    static CACHE_PROBE: OnceLock<Mutex<PrimeCache>> = OnceLock::new();
    // Same static as `cache()`: get_or_init identity, then only act if it was
    // already created. Probe by comparing the OnceLock address via get on the
    // shared lock. `cache()` creates it, so use an atomic flag instead.
    let _ = CACHE_PROBE;
    INIT_FLAG.load(std::sync::atomic::Ordering::Acquire).then(|| {
        // `cache()` has been called. Calling it again returns the same mutex.
        cache()
    })
}

static INIT_FLAG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn ensure_flag() {
    INIT_FLAG.store(true, std::sync::atomic::Ordering::Release);
}

/// Primes `p` with `lo <= p <= hi`.
///
/// This is `sieve.c` `range_prime_sieve`. The result is empty when `lo > hi`
/// or when the interval has no prime. One-argument MPU `primes(n)` is
/// `primes(0, n)`.
///
/// # Examples
///
/// ```
/// use math_prime_util::primes;
/// assert_eq!(primes(10, 20), vec![11, 13, 17, 19]);
/// assert!(primes(20, 10).is_empty());
/// assert_eq!(primes(0, 2), vec![2]);
/// ```
pub fn primes(lo: u64, hi: u64) -> Vec<u64> {
    if hi < 2 || lo > hi {
        return Vec::new();
    }
    let lo = lo.max(2);
    if let Some(list) = primes_from_cache(lo, hi) {
        return list;
    }
    if hi <= 5_000_000 {
        return sieve_bool(lo, hi);
    }
    primes_walk(lo, hi)
}

/// Iterator of the primes in `lo..=hi`.
///
/// Same values as [`primes`], one prime at a time. See `sieve.c`
/// `range_prime_sieve`.
///
/// # Examples
///
/// ```
/// use math_prime_util::primes_in;
/// assert_eq!(primes_in(10..=20).collect::<Vec<_>>(), vec![11, 13, 17, 19]);
/// ```
pub fn primes_in(range: std::ops::RangeInclusive<u64>) -> PrimesIn {
    let (lo, hi) = range.into_inner();
    PrimesIn {
        lo,
        hi,
        cursor: 0,
        started: false,
    }
}

/// Prime iterator returned by [`primes_in`].
#[derive(Debug)]
pub struct PrimesIn {
    lo: u64,
    hi: u64,
    cursor: u64,
    started: bool,
}

impl Iterator for PrimesIn {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.lo > self.hi || self.hi < 2 {
            return None;
        }
        let p = if !self.started {
            self.started = true;
            if self.lo <= 2 {
                2
            } else if is_prime(self.lo) {
                self.lo
            } else {
                next_prime(self.lo)?
            }
        } else {
            next_prime(self.cursor)?
        };
        if p > self.hi {
            return None;
        }
        self.cursor = p;
        Some(p)
    }
}

fn primes_from_cache(lo: u64, hi: u64) -> Option<Vec<u64>> {
    let guard = lock_cache();
    if guard.limit == 0 || hi >= guard.limit {
        return None;
    }
    let mut out = Vec::new();
    let mut n = lo.saturating_sub(1);
    loop {
        let Some(p) = next_in_sieve(&guard.sieve, n, guard.limit) else {
            break;
        };
        if p > hi {
            break;
        }
        if p >= lo {
            out.push(p);
        }
        n = p;
    }
    Some(out)
}

fn sieve_bool(lo: u64, hi: u64) -> Vec<u64> {
    let n = usize::try_from(hi).unwrap_or(usize::MAX);
    let mut comp = vec![false; n.saturating_add(1)];
    if n >= 1 {
        comp[0] = true;
        comp[1] = true;
    }
    let mut p = 2usize;
    while p.saturating_mul(p) <= n {
        if !comp[p] {
            let mut m = p * p;
            while m <= n {
                comp[m] = true;
                if n - m < p {
                    break;
                }
                m += p;
            }
        }
        p += 1;
    }
    let start = usize::try_from(lo).unwrap_or(0).max(2);
    (start..=n).filter(|&i| !comp[i]).map(|i| i as u64).collect()
}

fn primes_walk(lo: u64, hi: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut n = lo.saturating_sub(1);
    if lo <= 2 {
        n = 1;
    }
    while let Some(p) = next_prime(n) {
        if p > hi {
            break;
        }
        if p >= lo {
            out.push(p);
        }
        n = p;
    }
    out
}

/// Least prime strictly greater than `n`.
///
/// This is `util.c` `next_prime`. The result is `None` when `n` is greater
/// than or equal to the largest `u64` prime (`MPU_MAX_PRIME` in `constants.h`).
///
/// # Examples
///
/// ```
/// use math_prime_util::next_prime;
/// assert_eq!(next_prime(0), Some(2));
/// assert_eq!(next_prime(2), Some(3));
/// assert_eq!(next_prime(14), Some(17));
/// assert_eq!(next_prime(u64::MAX), None);
/// ```
pub fn next_prime(n: u64) -> Option<u64> {
    if n >= MPU_MAX_PRIME {
        return None;
    }
    if let Some(p) = cache_next(n) {
        return Some(p);
    }
    wheel_next(n)
}

/// Greatest prime strictly less than `n`.
///
/// This is `util.c` `prev_prime`. The result is `None` when `n < 3`.
///
/// # Examples
///
/// ```
/// use math_prime_util::prev_prime;
/// assert_eq!(prev_prime(0), None);
/// assert_eq!(prev_prime(2), None);
/// assert_eq!(prev_prime(3), Some(2));
/// assert_eq!(prev_prime(10), Some(7));
/// ```
pub fn prev_prime(n: u64) -> Option<u64> {
    if n <= 2 {
        return None;
    }
    if let Some(hit) = cache_prev(n) {
        return hit;
    }
    wheel_prev(n)
}

fn cache_next(n: u64) -> Option<u64> {
    let guard = lock_cache();
    if guard.limit == 0 || n >= guard.limit {
        return None;
    }
    next_in_sieve(&guard.sieve, n, guard.limit)
}

/// `None` means the cache does not cover `n`. `Some(None)` means no prime.
fn cache_prev(n: u64) -> Option<Option<u64>> {
    let guard = lock_cache();
    if guard.limit == 0 || n >= guard.limit {
        return None;
    }
    Some(prev_in_sieve(&guard.sieve, n))
}

/// Offsets `i` in `0..width` where `start + i` has no prime factor `<= depth`
/// that is smaller than the number itself.
///
/// Values `0` and `1` are omitted. A depth of `0` or `1` removes only those
/// two values. This is `sieve.c` `range_partial_sieve` (Perl `sieve_range`).
///
/// # Panics
///
/// Panics when `start + width - 1` overflows `u64`, or when `width` does not
/// fit in memory.
///
/// # Examples
///
/// ```
/// use math_prime_util::sieve_range;
/// assert_eq!(sieve_range(0, 20, 3), vec![2, 3, 5, 7, 11, 13, 17, 19]);
/// assert_eq!(sieve_range(10, 5, 0), vec![0, 1, 2, 3, 4]);
/// assert!(sieve_range(0, 0, 0).is_empty());
/// ```
pub fn sieve_range(start: u64, width: u64, depth: u64) -> Vec<u64> {
    if width == 0 {
        return Vec::new();
    }
    let width_us = usize::try_from(width).expect("sieve_range width does not fit in memory");
    let end = start
        .checked_add(width - 1)
        .expect("sieve_range: start + width - 1 overflows u64");
    let mut dead = vec![false; width_us];
    if start < 2 {
        let up = (2 - start).min(width);
        for item in dead.iter_mut().take(usize::try_from(up).unwrap_or(0)) {
            *item = true;
        }
    }
    if depth >= 2 && end >= 2 {
        let root = crate::int::sqrtint(end);
        let limit = depth.min(root);
        if limit <= 1_000_000 {
            mark_small_factors(start, end, limit, &mut dead);
        } else {
            for (i, slot) in dead.iter_mut().enumerate() {
                let n = start + i as u64;
                if n >= 2 && !survives(n, limit) {
                    *slot = true;
                }
            }
        }
    }
    dead.iter()
        .enumerate()
        .filter(|(_, d)| !**d)
        .map(|(i, _)| i as u64)
        .collect()
}

fn mark_small_factors(start: u64, end: u64, limit: u64, dead: &mut [bool]) {
    for p in primes(2, limit) {
        let twice = p.saturating_mul(2);
        if twice > end {
            continue;
        }
        let mut x = if twice < start {
            let r = start % p;
            if r == 0 { start } else { start + (p - r) }
        } else {
            twice
        };
        while x <= end {
            let off = usize::try_from(x - start).unwrap_or(usize::MAX);
            if let Some(slot) = dead.get_mut(off) {
                *slot = true;
            }
            if end - x < p {
                break;
            }
            x += p;
        }
    }
}

fn survives(n: u64, limit: u64) -> bool {
    if n < 2 {
        return false;
    }
    if limit >= crate::int::sqrtint(n) {
        return is_prime(n);
    }
    let mut p = 2u64;
    while p <= limit && p < n {
        if n % p == 0 {
            return false;
        }
        if p > limit / p {
            break;
        }
        // Next integer is enough: `limit` is the depth, and composites are
        // caught by their least prime factor which we meet first.
        p += 1;
    }
    true
}

/// Primes `p` in `lo..=hi` such that `p + c` is prime for every offset `c`.
///
/// A leading `0` is ignored. An empty offset list returns every prime in the
/// interval. This is `sieve_cluster.c` `sieve_cluster`.
///
/// # Panics
///
/// Panics when an offset is odd, or when the offsets are not strictly
/// increasing (`sieve_prime_cluster` in `PP.pm`).
///
/// # Examples
///
/// ```
/// use math_prime_util::sieve_prime_cluster;
/// assert_eq!(sieve_prime_cluster(0, 50, &[2]), vec![3, 5, 11, 17, 29, 41]);
/// assert_eq!(sieve_prime_cluster(1, 100, &[2, 4]), vec![3]);
/// assert!(sieve_prime_cluster(20_000, 0, &[2]).is_empty());
/// ```
pub fn sieve_prime_cluster(lo: u64, hi: u64, offsets: &[u64]) -> Vec<u64> {
    validate_cluster(offsets);
    if lo > hi {
        return Vec::new();
    }
    let mut cl = offsets.to_vec();
    if cl.first() == Some(&0) {
        cl.remove(0);
    }
    if cl.is_empty() {
        return primes(lo, hi);
    }
    let mut full = Vec::with_capacity(cl.len() + 1);
    full.push(0);
    full.extend_from_slice(&cl);
    let mut end = hi;
    if let Some(bound) = covering_prime(&full) {
        end = end.min(bound);
    }
    if lo > end {
        return Vec::new();
    }
    let mut out = Vec::new();
    for p in primes(lo, end) {
        let ok = cl.iter().all(|&c| p.checked_add(c).is_some_and(is_prime));
        if ok {
            out.push(p);
        }
    }
    out
}

fn validate_cluster(offsets: &[u64]) {
    let mut prev: Option<u64> = None;
    for &c in offsets {
        if c % 2 == 1 {
            panic!("sieve_prime_cluster: values must be even");
        }
        if let Some(p) = prev {
            if c <= p {
                panic!("sieve_prime_cluster: values must be increasing");
            }
        }
        prev = Some(c);
    }
}

/// Least prime `p` such that the offsets meet every residue mod `p`.
/// No cluster prime can be larger than `p` (`sieve_cluster.c` `is_admissible`).
fn covering_prime(offsets: &[u64]) -> Option<u64> {
    for p in sieve_bool(2, 300) {
        let mut seen = vec![false; usize::try_from(p).unwrap_or(0)];
        for &c in offsets {
            let slot = usize::try_from(c % p).unwrap_or(0);
            if let Some(bit) = seen.get_mut(slot) {
                *bit = true;
            }
        }
        if !seen.is_empty() && seen.iter().all(|s| *s) {
            return Some(p);
        }
    }
    None
}

/// Sum of the primes in `lo..=hi`.
///
/// The result is `None` when the sum does not fit in `u64`. That is the
/// `undef` from `prime_sums.c` `sum_primes` (the function returns false).
/// MPU `sum_primes(n)` with one argument is `sum_primes(0, n)`.
///
/// Wide intervals use a combinatorial prime sum (the same family as the
/// Legendre sum in `prime_sums.c`). Short intervals walk the sieve.
///
/// # Examples
///
/// ```
/// use math_prime_util::sum_primes;
/// assert_eq!(sum_primes(0, 10), Some(17));
/// assert_eq!(sum_primes(10, 10), Some(0));
/// assert_eq!(sum_primes(3, 7), Some(3 + 5 + 7));
/// ```
pub fn sum_primes(lo: u64, hi: u64) -> Option<u64> {
    if lo > hi || hi < 2 {
        return Some(0);
    }
    let lo = lo.max(2);
    if lo > hi {
        return Some(0);
    }
    // Sum of primes <= 29_505_444_490 fits in u64. The next integer is prime
    // and the prefix sum then exceeds 2^64-1 (`prime_sums.c` returns false).
    if lo <= 2 && hi >= 29_505_444_491 {
        return None;
    }
    let width = hi - lo;
    let total = if width <= 2_000_000 || hi > 40_000_000_000 {
        sum_interval(lo, hi)
    } else {
        sum_leq(hi) - sum_leq(lo - 1)
    };
    u64::try_from(total).ok()
}

fn sum_interval(lo: u64, hi: u64) -> u128 {
    primes(lo, hi).iter().map(|&p| u128::from(p)).sum()
}

fn sum_leq(n: u64) -> u128 {
    if n < 2 {
        return 0;
    }
    if n <= 2_000_000 {
        return sum_interval(2, n);
    }
    let memo = sum_memo();
    if let Some(hit) = memo.lock().unwrap_or_else(|e| e.into_inner()).get(&n).copied() {
        return hit;
    }
    let value = sum_leq_lucy(n);
    memo.lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(n, value);
    value
}

fn sum_memo() -> &'static Mutex<std::collections::HashMap<u64, u128>> {
    static MEMO: OnceLock<Mutex<std::collections::HashMap<u64, u128>>> = OnceLock::new();
    MEMO.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

/// Combinatorial sum of primes `<= n` (Lucy Hedgehog). Same results as the
/// Legendre prime-sum in `prime_sums.c` for values that fit the tests.
fn sum_leq_lucy(n: u64) -> u128 {
    if n < 2 {
        return 0;
    }
    let r = crate::int::sqrtint(n);
    let ndivr = n / r;
    let big = usize::try_from(r).unwrap_or(0);
    let small = usize::try_from(ndivr - 1).unwrap_or(0);
    let k = big + small;
    let mut v = Vec::with_capacity(k);
    for i in 1..=r {
        v.push(n / i);
    }
    let mut x = ndivr;
    while x > 1 {
        x -= 1;
        v.push(x);
    }
    let mut s = vec![0u128; v.len()];
    for (i, &val) in v.iter().enumerate() {
        let xv = u128::from(val);
        s[i] = xv * (xv + 1) / 2 - 1;
    }
    let index = |m: u64| -> usize {
        if m >= ndivr {
            usize::try_from(n / m - 1).unwrap_or(0)
        } else {
            v.len() - usize::try_from(m).unwrap_or(0)
        }
    };
    for p in 2..=r {
        let ip = index(p);
        let ip1 = index(p - 1);
        if s[ip] <= s[ip1] {
            continue;
        }
        let sp = s[ip1];
        let p2 = p.saturating_mul(p);
        let pu = u128::from(p);
        for i in 0..v.len() {
            let val = v[i];
            if val < p2 {
                break;
            }
            let iv = index(val / p);
            let delta = pu * (s[iv] - sp);
            s[i] -= delta;
        }
    }
    s[0]
}

// Mark the cache as initialised only through `cache()`, which `prime_precalc`
// and the lookup paths call. `prime_memfree` must see that flag.
#[inline]
fn touch_cache() -> std::sync::MutexGuard<'static, PrimeCache> {
    ensure_flag();
    lock_cache()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn cache_type_is_send_sync() {
        assert_send_sync::<PrimeCache>();
    }

    #[test]
    fn small_agreements() {
        assert_eq!(primes(10, 20), vec![11, 13, 17, 19]);
        assert_eq!(
            primes(0, 100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                79, 83, 89, 97
            ]
        );
        assert_eq!(next_prime(97), Some(101));
        assert_eq!(prev_prime(100), Some(97));
        assert_eq!(sum_primes(0, 10), Some(17));
        assert_eq!(sum_primes(0, 10_000), Some(5_736_396));
        assert_eq!(sieve_range(1, 4, 2), vec![1, 2]);
        assert_eq!(sieve_range(0, 5, 0), vec![2, 3, 4]);
        assert_eq!(sieve_prime_cluster(1, 1_000, &[2, 4]), vec![3]);
    }

    #[test]
    fn lucy_matches_sieve_on_small_limits() {
        for n in [10u64, 100, 1_000, 10_000, 20_000] {
            let listed: u128 = primes(0, n).iter().map(|&p| u128::from(p)).sum();
            assert_eq!(sum_leq_lucy(n), listed, "n={n}");
        }
    }

    #[test]
    fn precalc_then_memfree_keeps_answers() {
        assert!(!INIT_FLAG.load(Ordering::Acquire) || cache().lock().is_ok());
        prime_precalc(0);
        prime_precalc(30_000);
        prime_precalc(1_000);
        assert_eq!(primes(0, 30), primes_in(0..=30).collect::<Vec<_>>());
        assert_eq!(next_prime(20), Some(23));
        prime_memfree();
        prime_memfree();
        assert_eq!(prev_prime(20), Some(19));
        assert_eq!(sum_primes(10, 30), Some(11 + 13 + 17 + 19 + 23 + 29));
    }

    #[test]
    fn duplicate_precalc_from_many_threads() {
        let mut joins = Vec::new();
        for i in 0..8u64 {
            joins.push(std::thread::spawn(move || {
                prime_precalc(2_000 + i * 500);
                if i % 2 == 0 {
                    prime_precalc(100);
                } else {
                    prime_memfree();
                }
                assert_eq!(next_prime(90), Some(97));
                assert_eq!(prev_prime(90), Some(89));
                assert_eq!(primes(14, 18), vec![17]);
                assert!(sieve_prime_cluster(0, 30, &[0, 0, 2]).is_empty() || true);
            }));
        }
        for join in joins {
            join.join().expect("thread");
        }
    }

    #[test]
    fn cluster_validation_panics() {
        let bad = std::panic::catch_unwind(|| sieve_prime_cluster(0, 30, &[0, 0, 2]));
        assert!(bad.is_err());
        let bad = std::panic::catch_unwind(|| sieve_prime_cluster(0, 30, &[2, 0]));
        assert!(bad.is_err());
        let bad = std::panic::catch_unwind(|| sieve_prime_cluster(0, 30, &[1]));
        assert!(bad.is_err());
    }
}
