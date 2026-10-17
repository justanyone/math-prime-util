//! Twin prime wheel sieve.

pub(super) const MAX_TWIN: u64 = 18_446_744_073_709_550_771;

/// Return the smaller primes of twin pairs in `lo..=hi`.
///
/// Port of `twin_primes.c` (MPU 0.75), `range_twin_prime_sieve`.
/// The second prime may exceed `hi`.
///
/// ```
/// assert_eq!(math_prime_util::twin_primes(3, 11), vec![3, 5, 11]);
/// ```
#[must_use]
pub fn twin_primes(lo: u64, hi: u64) -> Vec<u64> {
    let mut out = Vec::new();
    scan(lo, hi, |p| { out.push(p); true });
    out
}

// Scan the same three twin positions per wheel byte as the C routine.
// Return false from visit to stop selection at the requested index.
pub(super) fn scan(lo: u64, hi: u64, mut visit: impl FnMut(u64) -> bool) {
    let hi = hi.min(MAX_TWIN);
    if lo > hi { return; }
    for p in [3, 5] {
        if lo <= p && p <= hi && !visit(p) { return; }
    }
    let mut low = lo.max(7);
    // A narrow interval near the native limit must not allocate sqrt(hi) space.
    if hi > 1_000_000_000_000 {
        while low <= hi {
            if matches!(low % 30, 11 | 17 | 29)
                && crate::is_prime(low) && crate::is_prime(low + 2) && !visit(low) {
                return;
            }
            low += 1;
        }
        return;
    }
    let base_primes = crate::primes(7, crate::sqrtint(hi + 2));
    while low <= hi {
        let high = low.saturating_add(983_039).min(hi);
        let base = low / 30 * 30;
        let limit = high + 2;
        let mut composite = vec![false; usize::try_from(limit - base + 1).expect("bounded segment")];
        for &p in &base_primes {
            let start = (base.div_ceil(p) * p).max(p * p);
            for n in (start..=limit).step_by(usize::try_from(p).expect("bounded prime")) {
                composite[usize::try_from(n - base).expect("bounded offset")] = true;
            }
        }
        for block in (base..=high).step_by(30) {
            for offset in [11, 17, 29] {
                let p = block + offset;
                if p < low || p > high { continue; }
                let i = usize::try_from(p - base).expect("bounded offset");
                if !composite[i] && !composite[i + 2] && !visit(p) { return; }
            }
        }
        low = high + 1;
    }
}
