use super::prho_factor::{dispatch, multiply, split};
use crate::{gcd, primes_in, sqrtint};

/// Splits `n` with Williams p+1 and two Lucas seeds.
///
/// This ports `factor.c`'s `pplus1_factor` and `pp1_pow` in MPU 0.75.
/// It includes the XS small-factor wrapper. `None` uses bound 200.
/// Cofactors can be composite. Failure returns the unsplit cofactor.
/// Zero returns `[0]`. One returns an empty vector.
///
/// ```
/// use math_prime_util::pplus1_factor;
/// assert_eq!(pplus1_factor(403, None), vec![13, 31]);
/// ```
pub fn pplus1_factor(n: u64, bound: Option<u64>) -> Vec<u64> {
    dispatch(n, |n| pplus1(n, bound.unwrap_or(200)))
}

fn subtract(a: u64, b: u64, n: u64) -> u64 {
    let b = b % n;
    if a >= b { a - b } else { n - (b - a) }
}

fn lucas_power(x0: u64, exp: u64, n: u64) -> u64 {
    let mut x = x0;
    let mut y = subtract(multiply(x, x, n), 2, n);
    let mut bit = 1_u64 << (exp.ilog2() - 1);
    while bit != 0 {
        let t = subtract(multiply(x, y, n), x0, n);
        if exp & bit != 0 {
            x = t;
            y = subtract(multiply(y, y, n), 2, n);
        } else {
            y = t;
            x = subtract(multiply(x, x, n), 2, n);
        }
        bit >>= 1;
    }
    x
}

fn pplus1(n: u64, bound: u64) -> Vec<u64> {
    let (mut x1, mut x2, mut f) = (7 % n, 11 % n, 1);
    let root = sqrtint(bound);
    for p in primes_in(2..=bound) {
        let mut k = p;
        if p < root {
            while k <= bound / p {
                k *= p;
            }
        }
        x1 = lucas_power(x1, k, n);
        if x1 != 2 {
            f = gcd(subtract(x1, 2, n), n);
            if f != 1 && f != n {
                break;
            }
        }
        x2 = lucas_power(x2, k, n);
        if x2 != 2 {
            f = gcd(subtract(x2, 2, n), n);
            if f != 1 && f != n {
                break;
            }
        }
    }
    split(n, f)
}
