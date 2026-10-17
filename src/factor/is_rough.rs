use super::is_practical::factor_into;
use crate::{is_prime, next_prime, sqrtint};

/// Return whether all prime factors of `n` are at least `k`.
/// Zero is rough only for a zero bound. One is always rough.
///
/// Port of `reference/mpu/util.c`, `is_rough`.
///
/// ```
/// assert!(math_prime_util::is_rough(77, 7));
/// assert!(!math_prime_util::is_rough(77, 8));
/// ```
pub fn is_rough(n: u64, k: u64) -> bool {
    if n == 0 {
        return k == 0;
    }
    if n == 1 || k <= 1 {
        return true;
    }
    if k > n {
        return false;
    }
    if k == 2 {
        return true;
    }
    if k == 3 {
        return n & 1 != 0;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    if k <= 5 {
        return true;
    }
    if n % 5 == 0 {
        return false;
    }
    if k > sqrtint(n) {
        return is_prime(n) != 0;
    }
    let limit = if k <= 2500 { k } else { 200 };
    let mut p = 7;
    while p < k && p <= limit && p <= n / p {
        if n % p == 0 {
            return false;
        }
        p = next_prime(p).unwrap_or(u64::MAX);
    }
    if k <= 2500 {
        return true;
    }
    if n >> 36 != 0 {
        if is_prime(n) != 0 {
            return true;
        }
        if let Some(f) = pminus1(n) {
            if f < k || n / f < k {
                return false;
            }
            let mut factors = Vec::new();
            factor_into(f, &mut factors);
            factor_into(n / f, &mut factors);
            return factors.into_iter().all(|p| p >= k);
        }
    }
    let mut factors = Vec::new();
    factor_into(n, &mut factors);
    factors.into_iter().all(|p| p >= k)
}

// reference/mpu/factor.c, pminus1_factor, stage 1 with B1 = B2 = 500.
fn pminus1(n: u64) -> Option<u64> {
    let (mut a, mut saved_a, mut saved_q) = (2, 2, 1);
    for (i, q) in crate::primes(2, 500).into_iter().enumerate() {
        let mut power = q;
        while power <= 500 / q {
            power *= q;
        }
        a = crate::powmod(a, power, n).unwrap();
        if (i + 1) % 32 == 0 {
            if a == 0 || crate::gcd(a - 1, n) != 1 {
                break;
            }
            saved_a = a;
            saved_q = q;
        }
    }
    if a == 0 {
        return None;
    }
    let mut f = crate::gcd(a - 1, n);
    if f == n {
        a = saved_a;
        for q in crate::primes(saved_q + 1, 500) {
            let mut power = q;
            while power <= 500 / q {
                power *= q;
            }
            a = crate::powmod(a, power, n).unwrap();
            f = crate::gcd(a - 1, n);
            if f != 1 {
                break;
            }
        }
    }
    (f > 1 && f < n).then_some(f)
}

#[cfg(test)]
mod tests {
    use super::pminus1;

    #[test]
    fn pminus1_stage_one() {
        for n in [7, 7 * 11, 1009 * 1_000_003, 1013 * 1_000_033, 503 * 509] {
            if let Some(f) = pminus1(n) {
                assert!(f > 1 && f < n);
                assert_eq!(n % f, 0);
            }
        }
        assert!(pminus1(1009 * 1_000_003).is_some());
        assert!(pminus1(7).is_none());
    }
}
