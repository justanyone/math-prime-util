//! Integers in a range with a given divisor count.
//!
//! Port of MPU 0.75 `inverse_sigma0.c`: the divisor-count sieve (`tau_sieve`)
//! for narrow ranges and the prime-signature enumeration (`by_signature`)
//! for wide ranges. The signature path is also used for counting wide
//! ranges, where the C code uses the faster `count_sig` recursion.

use crate::{crootint, prime_count, rootint};

/// Largest range width the divisor-count sieve handles, as in the C code.
const TAU_MAX_WIDTH: u64 = 10_000_000;
/// Largest width for the chunked sieve at large `lo`.
const TAU_MAX_CHUNKED_WIDTH: u64 = 100_000_000;
/// Smallest `lo` for which the chunked sieve is used.
const TAU_CHUNK_MIN_LO: u64 = 100_000_000_000;

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
    if k == 2 && lo <= hi {
        let below = lo.max(1) - 1;
        return Some(prime_count(hi) - prime_count(below));
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
    if k == 2 {
        for_each_prime(lo.max(2), hi, emit);
        return;
    }
    if !use_tau_sieve(lo, hi, k) {
        let mut list = by_signature(k, lo, hi);
        list.sort_unstable();
        list.dedup();
        list.into_iter().for_each(emit);
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

/// `use_tau_sieve` in `inverse_sigma0.c`.
fn use_tau_sieve(lo: u64, hi: u64, k: u64) -> bool {
    let width = hi - lo + 1;
    k <= 65_535
        && (width <= TAU_MAX_WIDTH || (lo >= TAU_CHUNK_MIN_LO && width <= TAU_MAX_CHUNKED_WIDTH))
}

/// `by_signature`: all `n` in `lo..=hi` with `k` divisors, unsorted, found
/// by splitting `k` into factors `e + 1` and placing primes under each exponent.
fn by_signature(k: u64, lo: u64, hi: u64) -> Vec<u64> {
    // Only factors up to 64 can be `e + 1` for `n < 2^64`.
    let divs: Vec<u64> = (2..=64).filter(|d| k % d == 0).collect();
    let mut out = Vec::new();
    let mut parts = Vec::new();
    partitions(&divs, k, 2, &mut parts, (lo, hi), &mut out);
    out
}

/// `partitions`: factor `rem` into non-decreasing parts, each at least `min_d`.
fn partitions(
    divs: &[u64],
    rem: u64,
    min_d: u64,
    parts: &mut Vec<u64>,
    range: (u64, u64),
    out: &mut Vec<u64>,
) {
    for &d in divs {
        if d < min_d {
            continue;
        }
        if d > rem / d {
            break;
        }
        if rem % d == 0 {
            parts.push(d);
            partitions(divs, rem / d, d, parts, range, out);
            parts.pop();
        }
    }
    if rem >= min_d {
        parts.push(rem);
        process_signature(parts, range, out);
        parts.pop();
    }
}

/// `process_signature`: turn the parts into descending exponents and recurse.
fn process_signature(parts: &[u64], (lo, hi): (u64, u64), out: &mut Vec<u64>) {
    let max_exp = u64::from(hi.ilog2());
    let mut exps = Vec::with_capacity(parts.len());
    for &part in parts.iter().rev() {
        let e = part - 1;
        if e == 0 || e > max_exp {
            return;
        }
        exps.push(u32::try_from(e).expect("exponent is at most 63"));
    }
    if !min_signature_le_hi(&exps, hi) {
        return;
    }
    let mut used = Vec::new();
    recurse(&exps, 0, 1, &mut used, (lo, hi), out);
}

/// `min_signature_le_hi`: the smallest number with this signature is at most `hi`.
fn min_signature_le_hi(exps: &[u32], hi: u64) -> bool {
    let mut n = 1_u64;
    let mut p = 1_u64;
    for &e in exps {
        p = crate::next_prime(p).expect("small primes exist");
        match mul_pow_limit(n, p, e, hi) {
            Some(v) => n = v,
            None => return false,
        }
    }
    true
}

/// `mul_pow_limit`: `m * p^e`, or `None` if it exceeds `limit`.
fn mul_pow_limit(mut m: u64, p: u64, e: u32, limit: u64) -> Option<u64> {
    for _ in 0..e {
        if m > limit / p {
            return None;
        }
        m *= p;
    }
    Some(m)
}

/// `prime_upper_bound`: largest prime that may sit at `exps[idx]`, given that
/// the later primes are each at least as large.
fn prime_upper_bound(exps: &[u32], idx: usize, maxpow: u64) -> u64 {
    let e = exps[idx];
    let same = exps[idx..].iter().take_while(|&&x| x == e).count();
    let lower_sum: u32 = exps[idx + same..].iter().sum();
    if lower_sum >= 64 {
        return 0;
    }
    let base = maxpow >> lower_sum;
    if base == 0 {
        return 0;
    }
    rootint(base, e * u32::try_from(same).expect("signature is short"))
}

/// `recurse`: choose distinct primes for `exps[idx..]` with product `n * ...` in range.
fn recurse(
    exps: &[u32],
    idx: usize,
    n: u64,
    used: &mut Vec<u64>,
    (lo, hi): (u64, u64),
    out: &mut Vec<u64>,
) {
    let e = exps[idx];
    let last = idx + 1 == exps.len();
    let maxpow = hi / n;
    let mut pl = 2;
    let ph;
    if last {
        let minpow = if n >= lo { 1 } else { lo.div_ceil(n) };
        pl = if e == 1 { minpow } else { crootint(minpow, e) }.max(2);
        ph = if e == 1 { maxpow } else { rootint(maxpow, e) };
    } else {
        ph = rootint(maxpow, e).min(prime_upper_bound(exps, idx, maxpow));
    }
    if idx > 0 && e == exps[idx - 1] && pl <= used[idx - 1] {
        pl = used[idx - 1] + 1;
    }
    if pl > ph {
        return;
    }
    for_each_prime(pl, ph, |p| {
        if used.contains(&p) {
            return;
        }
        let Some(v) = mul_pow_limit(n, p, e, hi) else {
            return;
        };
        if last {
            if v >= lo {
                out.push(v);
            }
        } else {
            used.push(p);
            recurse(exps, idx + 1, v, used, (lo, hi), out);
            used.pop();
        }
    });
}

/// Call `visit` for each prime in `lo..=hi`, in order, by segmented sieve.
///
/// Costs time in the width of the range, not in `hi` (after `range_prime_sieve`).
fn for_each_prime(lo: u64, hi: u64, mut visit: impl FnMut(u64)) {
    let lo = lo.max(2);
    if lo > hi {
        return;
    }
    let base = primes_up_to(isqrt(hi));
    let mut seglo = lo;
    loop {
        let seghi = hi.min(seglo.saturating_add(SEGMENT - 1));
        let len = usize::try_from(seghi - seglo + 1).expect("segment fits usize");
        let mut flags = vec![true; len];
        for &p in &base {
            if p > seghi / p {
                break;
            }
            let mut m = seglo.div_ceil(p).saturating_mul(p).max(p * p);
            while m <= seghi {
                flags[usize::try_from(m - seglo).expect("offset fits usize")] = false;
                match m.checked_add(p) {
                    Some(next) => m = next,
                    None => break,
                }
            }
        }
        for (i, &is_p) in flags.iter().enumerate() {
            if is_p {
                visit(seglo + i as u64);
            }
        }
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

    fn sieved(k: u64, lo: u64, hi: u64) -> Vec<u64> {
        let mut out = Vec::new();
        let primes = primes_up_to(isqrt(hi));
        sieve_segment(&primes, k, lo, hi, &mut |n| out.push(n));
        out
    }

    #[test]
    fn signature_path_matches_sieve() {
        for k in [3, 4, 6, 8, 9, 12, 16, 18, 24, 30, 36, 48, 64, 100, 128] {
            let mut got = by_signature(k, 1, 20_000);
            got.sort_unstable();
            got.dedup();
            assert_eq!(got, sieved(k, 1, 20_000), "k={k}");
            let mut got = by_signature(k, 5_000, 12_345);
            got.sort_unstable();
            got.dedup();
            assert_eq!(got, sieved(k, 5_000, 12_345), "k={k} sub-range");
        }
    }

    #[test]
    fn signature_edges() {
        assert_eq!(by_signature(2u64.pow(40), 1, 1000), Vec::<u64>::new());
        assert!(by_signature(65, 1, 1 << 40).len() > 1);
        assert_eq!(mul_pow_limit(1, 2, 70, u64::MAX), None);
        assert_eq!(prime_upper_bound(&[1, 1, 1], 0, 0), 0);
        assert_eq!(prime_upper_bound(&[1, 63, 2], 0, u64::MAX), 0);
    }

    #[test]
    fn wide_ranges_use_signatures() {
        assert!(!use_tau_sieve(1, 10_000_001, 8));
        assert!(use_tau_sieve(1, 10_000_000, 8));
        assert!(use_tau_sieve(200_000_000_000, 200_050_000_000, 8));
        assert!(!use_tau_sieve(1, 10, 70_000));
        let wide = inverse_sigma0(4, 1, 10_000_001).unwrap();
        assert_eq!(wide, sieved(4, 1, 10_000_001));
        assert_eq!(
            inverse_sigma0_count(4, 1, 10_000_001),
            Some(wide.len() as u64)
        );
        let lo = 1_u64 << 40;
        let mut high = by_signature(48, lo, lo + 3_000_000);
        high.sort_unstable();
        high.dedup();
        assert_eq!(high, sieved(48, lo, lo + 3_000_000));
    }

    #[test]
    fn for_each_prime_edges() {
        let mut got = Vec::new();
        for_each_prime((1 << 40) - 100, 1 << 40, |p| got.push(p));
        assert_eq!(got, vec![(1 << 40) - 87]);
        for_each_prime(10, 5, |_| panic!("empty range"));
    }

    #[test]
    fn k_two_uses_primes() {
        assert_eq!(inverse_sigma0(2, 0, 1), Some(vec![]));
        assert_eq!(inverse_sigma0_count(2, 0, 1), Some(0));
        assert_eq!(inverse_sigma0_count(2, 5, 4), Some(0));
        assert_eq!(inverse_sigma0(2, 0, 10), Some(vec![2, 3, 5, 7]));
        assert_eq!(inverse_sigma0_count(2, 0, 10), Some(4));
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
