use crate::{gcd, is_prime, sqrtint};

/// Tries Fermat's difference of squares method.
///
/// Port of `factor.c::fermat_factor` and the `XS.xs` factor wrapper in MPU 0.75.
/// `None` uses 64 million rounds. A failed attempt retains the cofactor.
/// The returned factors can be composite. Zero returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::fermat_factor;
/// assert_eq!(fermat_factor(91, None), vec![7, 13]);
/// ```
pub fn fermat_factor(n: u64, rounds: Option<u64>) -> Vec<u64> {
    attempt(n, |n| split(n, rounds.unwrap_or(64_000_000)))
}

fn split(n: u64, mut rounds: u64) -> u64 {
    let root = sqrtint(n);
    let mut x = i128::from(2 * root + 1);
    let mut y = 1;
    let mut r = -i128::from(n - root * root);
    while r != 0 {
        if rounds == 0 {
            return n;
        }
        rounds -= 1;
        r += x;
        x += 2;
        loop {
            r -= y;
            y += 2;
            if r <= 0 {
                break;
            }
        }
    }
    u64::try_from((x - y) / 2).unwrap_or(n)
}

// The common public wrapper in XS.xs, not a recursive prime factorization.
pub(super) fn attempt(mut n: u64, method: impl FnOnce(u64) -> u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut result = Vec::new();
    for p in [2, 3, 5] {
        while n % p == 0 {
            result.push(p);
            n /= p;
        }
    }
    if n > 1 {
        let f = if is_prime(n) != 0 { n } else { method(n) };
        let f = gcd(n, f);
        if f > 1 && f < n {
            result.extend([f.min(n / f), f.max(n / f)]);
        } else {
            result.push(n);
        }
    }
    result
}
