use crate::{gcd, is_prime, powmod};

/// Return whether `n` is a Carmichael number.
///
/// Port of `is_carmichael` in MPU 0.75 `util.c`. Factoring uses the
/// non-Montgomery `pbrent_factor` method from `factor.c`.
///
/// ```
/// assert!(math_prime_util::is_carmichael(561));
/// assert!(!math_prime_util::is_carmichael(563));
/// ```
pub fn is_carmichael(n: u64) -> bool {
    if n < 561 || n % 2 == 0 {
        return false;
    }
    if [9, 25, 49, 121, 169].iter().any(|p| n % p == 0) {
        return false;
    }
    for p in [5, 7, 11, 13, 17, 19, 23] {
        if n % p == 0 && (n - 1) % (p - 1) != 0 {
            return false;
        }
    }
    if n > 5_000_000 {
        for p in [29, 31, 37, 41, 43] {
            if n % p == 0 && (n - 1) % (p - 1) != 0 {
                return false;
            }
        }
        if powmod(2, n - 1, n) != Some(1) {
            return false;
        }
    }
    let factors = prime_factors(n);
    factors.len() >= 3
        && factors.windows(2).all(|w| w[0] != w[1])
        && factors.iter().all(|p| (n - 1) % (p - 1) == 0)
}

// Private support until the sibling factor port supplies the public API.
// MPU factor.c: factor (sorted factors), and pbrent_factor (Brent rho).
pub(super) fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43] {
        while n > 1 && n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    let mut pending = vec![n];
    while let Some(n) = pending.pop() {
        if n < 2 {
            continue;
        }
        if is_prime(n) != 0 {
            factors.push(n);
            continue;
        }
        let mut a = 1;
        let divisor = loop {
            if let Some(d) = brent(n, a) {
                break d;
            }
            a += 1;
        };
        pending.push(divisor);
        pending.push(n / divisor);
    }
    factors.sort_unstable();
    factors
}

#[allow(clippy::cast_possible_truncation)] // Remainders modulo u64 fit u64.
fn brent(n: u64, mut a: u64) -> Option<u64> {
    let step =
        |x: u64, a: u64| ((u128::from(x) * u128::from(x) + u128::from(a)) % u128::from(n)) as u64;
    let inner = if n <= 4_000_000_000 { 32 } else { 160 };
    let (mut r, mut xi, mut xm) = (1_u64, 1, 1);
    let mut rounds = 500_000;
    let mut fails = 6;
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = xi;
        let mut f = 1;
        while left > 0 {
            let count = left.min(inner);
            saved = xi;
            left -= count;
            rounds -= count;
            xi = step(xi, a);
            let mut m = xi.abs_diff(xm);
            for _ in 1..count {
                xi = step(xi, a);
                m = (u128::from(m) * u128::from(xi.abs_diff(xm)) % u128::from(n)) as u64;
            }
            f = gcd(m, n);
            if f != 1 {
                break;
            }
        }
        if f == 1 {
            r *= 2;
            xm = xi;
            continue;
        }
        if f == n {
            xi = saved;
            loop {
                xi = step(xi, a);
                f = gcd(xi.abs_diff(xm), n);
                if f != 1 || r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        if f == n || f == 0 {
            if fails == 0 {
                break;
            }
            fails -= 1;
            xm = ((u128::from(xm) + 11) % u128::from(n)) as u64;
            xi = xm;
            a += 1;
        } else {
            return Some(f);
        }
    }
    None
}
