use crate::{crootint, pn_primorial, primes, rootint, sqrtint};

/// Return numbers in `lo..=hi` with `k` distinct prime factors.
///
/// Port of MPU 0.75 `omega_primes.c`, `range_omega_prime_sieve`,
/// `sieve_omega_primes`, and `_omega_prime_gen_rec`.
/// Use bounded factor-sieve chunks or recursive prefix construction.
///
/// ```
/// use math_prime_util::omega_primes;
/// assert_eq!(omega_primes(2, 1, 15), vec![6, 10, 12, 14, 15]);
/// assert!(omega_primes(2, 20, 10).is_empty());
/// ```
pub fn omega_primes(k: u32, mut lo: u64, hi: u64) -> Vec<u64> {
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
    if k > 15 {
        return Vec::new();
    }
    let Some(min) = pn_primorial(u64::from(k)) else {
        return Vec::new();
    };
    if min > hi {
        return Vec::new();
    }
    lo = lo.max(min);
    if k == 1 {
        return super::prime_powers(lo, hi);
    }
    let mut result = Vec::new();
    if lo == min && (hi - lo > 100_000_000 || (k >= 10 && hi - lo > 5_000_000)) {
        generate(&mut result, k, lo, hi, 1, 2);
        result.sort_unstable();
        return result;
    }
    let ps = primes(2, sqrtint(hi));
    loop {
        let end = lo.saturating_add(4_999_999).min(hi);
        let mut residual: Vec<u64> = (lo..=end).collect();
        let mut counts = vec![0_u8; residual.len()];
        for &p in &ps {
            let offset = (p - lo % p) % p;
            let Some(first) = lo.checked_add(offset) else {
                continue;
            };
            if first > end {
                continue;
            }
            for v in (first..=end).step_by(usize::try_from(p).unwrap_or(usize::MAX)) {
                let i = usize::try_from(v - lo).expect("chunk index fits usize");
                counts[i] += 1;
                while residual[i] % p == 0 {
                    residual[i] /= p;
                }
            }
        }
        for (i, (&remaining, &count)) in residual.iter().zip(&counts).enumerate() {
            if u32::from(count) + u32::from(remaining > 1) == k {
                result.push(lo + i as u64);
            }
        }
        if end == hi {
            break;
        }
        lo = end + 1;
    }
    result
}

fn generate(out: &mut Vec<u64>, k: u32, lo: u64, hi: u64, m: u64, start: u64) {
    if k > 1 {
        for p in primes(start, rootint(hi / m, k)) {
            if m % p == 0 {
                continue;
            }
            let mut v = m * p;
            while v <= hi / p {
                generate(out, k - 1, lo, hi, v, p);
                v *= p;
            }
        }
    } else {
        let min = lo.div_ceil(m);
        let max = hi / m;
        for e in 1..=64 {
            let pmax = rootint(max, e);
            if pmax <= start {
                break;
            }
            for p in primes(crootint(min, e).max(start), pmax) {
                if m % p != 0 {
                    out.push(m * p.pow(e));
                }
            }
        }
    }
}
