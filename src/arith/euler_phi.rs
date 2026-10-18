use super::{factors, range_len};
use crate::{rootint, sqrtint};

/// Return Euler's totient. Port of `totients.c::totient` and `_totient_fac`.
/// ```
/// assert_eq!(math_prime_util::euler_phi(36), 12);
/// ```
pub fn euler_phi(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    factors(n)
        .into_iter()
        .fold(1, |v, (p, e)| v * (p - 1) * p.pow(e - 1))
}

/// Return totients on an inclusive interval. Port of `totients.c::range_totient`.
/// # Panics
/// Panics if the interval is reversed or too large to allocate.
/// ```
/// assert_eq!(math_prime_util::euler_phi_range(0, 4), [0,1,1,2,2]);
/// ```
pub fn euler_phi_range(lo: u64, hi: u64) -> Vec<u64> {
    let len = range_len(lo, hi);
    if hi < 16 {
        const SMALL: [u64; 16] = [0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8];
        return SMALL[usize::try_from(lo).unwrap()..=usize::try_from(hi).unwrap()].to_vec();
    }
    if lo == 0 {
        let mut t = vec![0; len];
        t[1] = 1;
        let mut primes = Vec::new();
        for i in (1..=hi / 2).step_by(2) {
            let ix = usize::try_from(i).unwrap();
            if t[ix] == 0 {
                t[ix] = i - 1;
                if i <= sqrtint(hi) {
                    primes.push(i);
                }
            }
            let v = t[ix];
            for &p in &primes {
                if p > hi / i {
                    break;
                }
                let j = usize::try_from(i * p).unwrap();
                if i % p == 0 {
                    t[j] = v * p;
                    break;
                }
                t[j] = v * (p - 1);
            }
            t[ix * 2] = v;
            if i < hi / 2 {
                t[ix * 2 + 2] = t[ix + 1] * 2;
            }
        }
        for i in (1..len).step_by(2) {
            if t[i] == 0 {
                t[i] = u64::try_from(i).unwrap() - 1;
            }
        }
        t
    } else {
        // MPU factor.c::factor_range_init and _vec_factor use chunks of 256K.
        let mut result = Vec::with_capacity(len);
        for start in (0..len).step_by(256 * 1024) {
            let end = (start + 256 * 1024).min(len);
            let low = lo + u64::try_from(start).unwrap();
            let high = lo + u64::try_from(end - 1).unwrap();
            result.extend(sieve_chunk(low, high));
        }
        result
    }
}

// MPU factor.c::_fr_full_sieve: the same square-root/cube-root thresholds.
fn full_sieve(root: u64, width: u64) -> bool {
    let threshold = match root {
        0..10_000_000 => return true,
        10_000_000..35_000_000 => 900,
        35_000_000..100_000_000 => 1700,
        100_000_000..350_000_000 => 3400,
        350_000_000..1_000_000_000 => 5500,
        1_000_000_000..3_500_000_000 => 17000,
        _ => 19000,
    };
    width > threshold
}

fn sieve_chunk(lo: u64, hi: u64) -> Vec<u64> {
    if hi - lo < 100 {
        return (lo..=hi).map(euler_phi).collect();
    }
    let root = sqrtint(hi);
    let full = full_sieve(root, hi - lo);
    let limit = if full { root } else { rootint(hi, 3) };
    let mut rem: Vec<u64> = (lo..=hi).collect();
    let mut values = rem.clone();
    for p in crate::sieve::primes(2, limit) {
        let offset = (p - lo % p) % p;
        if offset > hi - lo {
            continue;
        }
        for i in
            (usize::try_from(offset).unwrap()..values.len()).step_by(usize::try_from(p).unwrap())
        {
            values[i] = values[i] / p * (p - 1);
            while rem[i] % p == 0 {
                rem[i] /= p;
            }
        }
    }
    for (r, v) in rem.into_iter().zip(&mut values) {
        if full {
            if r > 1 {
                *v = *v / r * (r - 1);
            }
        } else {
            for (p, _) in factors(r) {
                *v = *v / p * (p - 1);
            }
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::full_sieve;

    #[test]
    fn reference_sieve_thresholds() {
        assert!(full_sieve(9_999_999, 0));
        for (root, width) in [
            (10_000_000, 900),
            (35_000_000, 1700),
            (100_000_000, 3400),
            (350_000_000, 5500),
            (1_000_000_000, 17000),
            (3_500_000_000, 19000),
        ] {
            assert!(!full_sieve(root, width));
            assert!(full_sieve(root, width + 1));
        }
    }
}
