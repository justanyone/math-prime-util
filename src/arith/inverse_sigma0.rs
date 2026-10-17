//! Integers in a range with a given divisor count.
//!
//! Port of MPU 0.75 `inverse_sigma0.c`, sieve path (`tau_sieve`). The
//! generator and prime-signature paths are not ported yet.

/// Width of one sieve segment. The C code uses 10^7 with 16-bit counts.
/// This port uses 2^20 to bound memory. Segmenting does not change results.
const SEGMENT: u64 = 1 << 20;

/// Return the integers in `lo..=hi` that have exactly `k` divisors, in
/// ascending order. Values below 1 are skipped. Return `None` when `k` is 0.
///
/// Port of MPU 0.75 `inverse_sigma0.c::inverse_sigma0_list` (sieve path).
///
/// ```
/// use math_prime_util::inverse_sigma0;
/// assert_eq!(inverse_sigma0(2, 10, 20), Some(vec![11, 13, 17, 19]));
/// assert_eq!(inverse_sigma0(0, 1, 10), None);
/// ```
#[must_use]
pub fn inverse_sigma0(k: u64, lo: u64, hi: u64) -> Option<Vec<u64>> {
    if k == 0 {
        return None;
    }
    let mut out = Vec::new();
    for_each_match(k, lo, hi, |n| out.push(n));
    Some(out)
}

/// Return how many integers in `lo..=hi` have exactly `k` divisors.
/// Return 0 when `k` is 0.
///
/// Port of MPU 0.75 `inverse_sigma0.c::inverse_sigma0_count` (sieve path).
///
/// ```
/// use math_prime_util::inverse_sigma0_count;
/// assert_eq!(inverse_sigma0_count(4, 1, 10), Some(3));
/// assert_eq!(inverse_sigma0_count(0, 1, 10), Some(0));
/// ```
#[must_use]
pub fn inverse_sigma0_count(k: u64, lo: u64, hi: u64) -> Option<u64> {
    if k == 0 {
        return Some(0);
    }
    let mut count = 0;
    for_each_match(k, lo, hi, |_| count += 1);
    Some(count)
}

/// Call `emit` for each `n` in `lo..=hi` with `k` divisors, in order.
///
/// The k=1 and k>hi early returns follow `inverse_sigma0.c`, so large
/// endpoints do not allocate the sieve.
fn for_each_match(k: u64, lo: u64, hi: u64, mut emit: impl FnMut(u64)) {
    let lo = lo.max(1);
    if lo > hi || k > hi {
        return;
    }
    if k == 1 {
        // Only 1 has one divisor; lo >= 1 and lo <= hi by the check above.
        if lo == 1 {
            emit(1);
        }
        return;
    }
    let primes = primes_up_to(isqrt(hi));
    let mut seglo = lo;
    loop {
        let seghi = hi.min(seglo.saturating_add(SEGMENT - 1));
        sieve_segment(&primes, k, seglo, seghi, &mut emit);
        if seghi == hi {
            break;
        }
        seglo = seghi + 1;
    }
}

/// Divisor-count sieve for `lo..=hi`, after `tau_sieve` in `inverse_sigma0.c`.
///
/// Every prime up to `sqrt(hi)` is divided out. The residue left is then 1
/// or one prime, so it adds a factor of 2 to the count.
#[allow(clippy::cast_possible_truncation)] // Offsets are below SEGMENT.
fn sieve_segment(primes: &[u64], k: u64, lo: u64, hi: u64, emit: &mut impl FnMut(u64)) {
    let len = (hi - lo + 1) as usize;
    let mut rem: Vec<u64> = (lo..=hi).collect();
    let mut tau = vec![1_u64; len];
    for &p in primes {
        if p * p > hi {
            break;
        }
        let first = lo.div_ceil(p).saturating_mul(p);
        if first > hi {
            continue;
        }
        let start = (first - lo) as usize;
        for i in (start..len).step_by(p as usize) {
            let mut e = 0;
            while rem[i] % p == 0 {
                rem[i] /= p;
                e += 1;
            }
            tau[i] *= e + 1;
        }
    }
    for i in 0..len {
        let t = if rem[i] > 1 { tau[i] * 2 } else { tau[i] };
        if t == k {
            emit(lo + i as u64);
        }
    }
}

/// Primes up to and including `n`, by the sieve of Eratosthenes.
#[allow(clippy::cast_possible_truncation)] // Callers pass sqrt of a u64.
fn primes_up_to(n: u64) -> Vec<u64> {
    let n = n as usize;
    let mut composite = vec![false; n + 1];
    let mut primes = Vec::new();
    for p in 2..=n {
        if composite[p] {
            continue;
        }
        primes.push(p as u64);
        if p <= n / p {
            for m in (p * p..=n).step_by(p) {
                composite[m] = true;
            }
        }
    }
    primes
}

/// Integer square root: the largest `r` with `r * r <= n`.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while u128::from(r) * u128::from(r) > u128::from(n) {
        r -= 1;
    }
    while u128::from(r + 1) * u128::from(r + 1) <= u128::from(n) {
        r += 1;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_lists() {
        assert_eq!(inverse_sigma0(1, 0, 10), Some(vec![1]));
        assert_eq!(inverse_sigma0(2, 10, 20), Some(vec![11, 13, 17, 19]));
        assert_eq!(inverse_sigma0(4, 10, 10), Some(vec![10]));
        assert_eq!(inverse_sigma0(0, 1, 10), None);
    }

    #[test]
    fn count_zero_k_is_zero() {
        assert_eq!(inverse_sigma0_count(0, 1, 10), Some(0));
    }

    #[test]
    fn k_one_and_large_endpoints_return_early() {
        // Each call must return without sieving up to sqrt(u64::MAX).
        assert_eq!(inverse_sigma0_count(1, u64::MAX, u64::MAX), Some(0));
        assert_eq!(inverse_sigma0(1, u64::MAX, u64::MAX), Some(vec![]));
        assert_eq!(inverse_sigma0_count(1, 0, 0), Some(0));
        assert_eq!(inverse_sigma0(1, 0, 5), Some(vec![1]));
        assert_eq!(inverse_sigma0_count(1, 2, 5), Some(0));
        // k > hi cannot match, since tau(n) <= n.
        assert_eq!(inverse_sigma0_count(u64::MAX, 1, 1000), Some(0));
    }

    #[test]
    fn empty_and_reversed_ranges() {
        assert_eq!(inverse_sigma0(2, 5, 4), Some(vec![]));
        assert_eq!(inverse_sigma0_count(2, 5, 4), Some(0));
        assert_eq!(inverse_sigma0_count(2, 0, 0), Some(0));
    }

    #[test]
    fn primes_span_segments() {
        // pi(2^21) = 155611 and the segment width is 2^20.
        assert_eq!(inverse_sigma0_count(2, 1, 1 << 21), Some(155_611));
        assert_eq!(inverse_sigma0_count(1, 1, 1 << 21), Some(1));
    }

    #[test]
    fn isqrt_edges() {
        for n in [0, 1, 2, 3, 4, 15, 16, 17, u64::MAX] {
            let r = isqrt(n);
            assert!(u128::from(r) * u128::from(r) <= u128::from(n));
            assert!(u128::from(r + 1) * u128::from(r + 1) > u128::from(n));
        }
    }
}
