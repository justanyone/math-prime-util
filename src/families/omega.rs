//! Integers with a fixed number of distinct prime factors.
use crate::{prime_count, primes, rootint};

fn primorial(k: u32) -> Option<u64> {
    if k > 15 {
        return None;
    }
    primes(2, 47)
        .into_iter()
        .take(k as usize)
        .try_fold(1u64, u64::checked_mul)
}

// omega_primes.c: _omega_prime_count_rec2. Prime powers share one factor.
fn count_rec(k: u32, n: u64, start: u64) -> u64 {
    if k == 1 {
        let mut sum = 0;
        for e in 1..=63 {
            let r = rootint(n, e);
            if r < start {
                break;
            }
            sum += prime_count(r) - prime_count(start - 1);
        }
        return sum;
    }
    let mut sum = 0;
    for p in primes(start, rootint(n, k)) {
        let mut power = p;
        while power <= n {
            sum += count_rec(k - 1, n / power, p + 1);
            if power > n / p {
                break;
            }
            power *= p;
        }
    }
    sum
}

/// Count integers at most `n` with `k` distinct prime factors.
/// Port of MPU `omega_primes.c`, `omega_prime_count`.
/// ```
/// assert_eq!(math_prime_util::omega_prime_count(2, 10), 2);
/// ```
pub fn omega_prime_count(k: u32, n: u64) -> u64 {
    if k == 0 {
        return u64::from(n >= 1);
    }
    if primorial(k).is_none_or(|p| n < p) {
        return 0;
    }
    count_rec(k, n, 2)
}

// omega_primes.c: sieve_omega_primes, bounded factor-sieve windows.
pub(super) fn factor_sieve(k: u32, lo: u64, hi: u64, repeated: bool) -> Vec<u64> {
    let mut out = Vec::new();
    let mut low = lo.max(2);
    while low <= hi {
        let high = low.saturating_add(262_143).min(hi);
        let len = usize::try_from(high - low + 1).expect("bounded sieve window");
        let mut rem: Vec<_> = (low..=high).collect();
        let mut counts = vec![0u32; len];
        for p in primes(2, crate::sqrtint(high)) {
            let offset = (p - low % p) % p;
            for i in (usize::try_from(offset).expect("prime fits usize")..len)
                .step_by(usize::try_from(p).expect("prime fits usize"))
            {
                counts[i] += 1;
                while rem[i] % p == 0 {
                    rem[i] /= p;
                    if repeated && rem[i] % p == 0 {
                        counts[i] += 1;
                    }
                }
            }
        }
        out.extend(
            rem.iter()
                .zip(counts)
                .zip(low..=high)
                .filter_map(|((&r, c), v)| (c + u32::from(r > 1) == k).then_some(v)),
        );
        if high == hi {
            break;
        }
        low = high + 1;
    }
    out
}

// Stop a branch when even its first distinct prime product is too large.
fn fits(k: u32, start: u64, limit: u64) -> bool {
    let mut product = 1u64;
    let mut p = crate::next_prime(start - 1).unwrap_or(u64::MAX);
    for _ in 0..k {
        if product > limit / p {
            return false;
        }
        product *= p;
        p = crate::next_prime(p).unwrap_or(u64::MAX);
    }
    true
}

// omega_primes.c: _omega_prime_gen_rec and _omega_prime_gen_last.
fn generate(k: u32, lo: u64, hi: u64, m: u64, start: u64, out: &mut Vec<u64>) {
    if !fits(k, start, hi / m) {
        return;
    }
    if k == 1 {
        let minpow = lo.div_ceil(m).max(1);
        for e in 1..=63 {
            let pmax = rootint(hi / m, e);
            if pmax < start {
                break;
            }
            let pmin = start.max(rootint(minpow - 1, e) + 1);
            for p in primes(pmin, pmax) {
                if let Some(v) = p.checked_pow(e).and_then(|v| m.checked_mul(v)) {
                    out.push(v);
                }
            }
        }
    } else {
        for p in primes(start, rootint(hi / m, k)) {
            if !fits(k, p, hi / m) {
                break;
            }
            let mut v = m * p;
            loop {
                generate(k - 1, lo, hi, v, p + 1, out);
                if v > hi / p {
                    break;
                }
                v *= p;
            }
        }
    }
}

/// List integers in the closed range with `k` distinct prime factors.
/// Port of MPU `omega_primes.c`, `range_omega_prime_sieve`.
/// ```
/// assert_eq!(math_prime_util::omega_primes(2, 1, 12), vec![6, 10, 12]);
/// ```
pub fn omega_primes(k: u32, lo: u64, hi: u64) -> Vec<u64> {
    if lo > hi {
        return Vec::new();
    }
    if k == 0 {
        return if lo <= 1 && hi >= 1 {
            vec![1]
        } else {
            Vec::new()
        };
    }
    let Some(min) = primorial(k) else {
        return Vec::new();
    };
    let lo = lo.max(min);
    if lo == min
        && (hi.saturating_sub(lo) > 100_000_000 || (k >= 10 && hi.saturating_sub(lo) > 5_000_000))
    {
        let mut out = Vec::new();
        generate(k, lo, hi, 1, 2, &mut out);
        out.sort_unstable();
        out
    } else {
        factor_sieve(k, lo, hi, false)
    }
}

/// Return the `n`th integer with `k` distinct prime factors.
/// Return `None` if no native result exists.
/// Port of MPU `omega_primes.c`, `nth_omega_prime`.
/// ```
/// assert_eq!(math_prime_util::nth_omega_prime(2, 3), Some(12));
/// ```
pub fn nth_omega_prime(k: u32, n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    if k == 0 {
        return (n == 1).then_some(1);
    }
    let mut lo = primorial(k)?;
    // omega_primes.c: max_omega_prime_count and max_nth_omega_prime.
    let max = [
        1,
        425_656_284_140_516_112,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        5_512_134_903_353,
        87_133_769_732,
        446_745_559,
        299_178,
    ][k as usize];
    if n > max {
        return None;
    }
    if n == 1 {
        return Some(lo);
    }
    let mut hi = lo;
    while omega_prime_count(k, hi) < n {
        if hi == u64::MAX {
            return None;
        }
        hi = hi.saturating_mul(2);
    }
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if omega_prime_count(k, mid) < n {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    Some(lo)
}
