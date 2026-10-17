use super::prho_factor::{dispatch, multiply, split};
use crate::int::mont::Mont;
use crate::{gcd, primes_in};

/// Splits `n` with Pollard p-1 and two stages.
///
/// This ports `factor.c`'s `pminus1_factor` in MPU 0.75 and its XS wrapper.
/// `None` uses bounds 1,000,000 and ten times the first bound.
/// Cofactors can be composite. Failure returns the unsplit cofactor.
/// Zero returns `[0]`. One returns an empty vector.
///
/// ```
/// use math_prime_util::pminus1_factor;
/// assert_eq!(pminus1_factor(403, None, None), vec![13, 31]);
/// ```
pub fn pminus1_factor(n: u64, b1: Option<u64>, b2: Option<u64>) -> Vec<u64> {
    let b1 = b1.unwrap_or(1_000_000);
    let b2 = b2.unwrap_or_else(|| b1.saturating_mul(10));
    dispatch(n, |n| pminus1(n, b1, b2))
}

/// `a^k mod n` with one Montgomery context (`PMINUS1_APPLY_POWER` in
/// `factor.c` when `USE_MONTMATH` is set). `n` is odd.
fn power(a: u64, k: u64, n: u64) -> u64 {
    let mont = Mont::new(n);
    mont.recover(mont.pow(mont.to_mont(a % n), k))
}

fn prime_power(q: u64, bound: u64) -> u64 {
    let mut k = q;
    while k <= bound / q {
        k *= q;
    }
    k
}

fn pminus1(n: u64, b1: u64, b2: u64) -> Vec<u64> {
    let (mut a, mut q, mut save_a, mut save_q) = (2, 2, 2, 1);
    for (i, p) in primes_in(2..=b1).enumerate() {
        q = p;
        a = power(a, prime_power(q, b1), n);
        if (i + 1) % 32 == 0 {
            if a == 0 || gcd(a - 1, n) != 1 {
                break;
            }
            save_a = a;
            save_q = q;
        }
    }
    if a == 0 {
        return vec![n];
    }
    let mut f = gcd(a - 1, n);
    if f == n {
        a = save_a;
        for p in primes_in(save_q + 1..=b1) {
            q = p;
            a = power(a, prime_power(q, b1), n);
            f = gcd(a.abs_diff(1), n);
            if f != 1 {
                break;
            }
        }
    }
    if f == 1 && b2 > b1 {
        let bm = a;
        let mut b = 1;
        let mut cache = [0; 111];
        let mut diff = multiply(bm, bm, n);
        for entry in &mut cache[..20] {
            *entry = diff;
            diff = multiply(multiply(diff, bm, n), bm, n);
        }
        a = power(a, q, n);
        for (i, p) in primes_in(q + 1..=b2).enumerate() {
            let gap = p - q;
            q = p;
            let index = usize::try_from(gap / 2)
                .unwrap_or(usize::MAX)
                .wrapping_sub(1);
            let diff = if gap % 2 != 0 || index >= cache.len() {
                power(bm, gap, n)
            } else {
                if cache[index] == 0 {
                    cache[index] = if index > 0 && cache[index - 1] != 0 {
                        multiply(multiply(cache[index - 1], bm, n), bm, n)
                    } else {
                        power(bm, gap, n)
                    };
                }
                cache[index]
            };
            a = multiply(a, diff, n);
            if a == 0 {
                break;
            }
            b = multiply(b, a - 1, n);
            if (i + 1) % 64 == 0 && gcd(b, n) != 1 {
                break;
            }
        }
        f = gcd(b, n);
    }
    split(n, f)
}

#[cfg(test)]
mod tests {
    use super::{pminus1, prime_power};

    #[test]
    fn bounds_and_recovery() {
        assert_eq!(prime_power(2, 32), 32);
        assert_eq!(prime_power(3, 32), 27);
        assert_eq!(pminus1(49, 0, 0), vec![49]);
        assert_eq!(pminus1(49, 0, 10), vec![7, 7]);
        assert_eq!(pminus1(91, 1000, 1000), vec![91]);
        assert_eq!(pminus1(563_777_293, 1000, 20_000), vec![23_099, 24_407]);
    }
}
