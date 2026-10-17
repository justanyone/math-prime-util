use super::native::{factor_exp, valuation};
use super::{invmod, mulmod_u, powmod_u};
use crate::{is_prime, sqrtint};

/// Return `n! mod m`. Return `None` for a zero modulus.
///
/// Port of `reference/mpu/factmod.c`: `factorialmod` and `_facmod`.
/// ```
/// use math_prime_util::factorialmod;
/// assert_eq!(factorialmod(10, 101), Some(72));
/// ```
pub fn factorialmod(n: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }
    if n >= m || m == 1 {
        return Some(0);
    }
    if n <= 1 {
        return Some(1);
    }
    let prime = is_prime(m) != 0;
    let d = if prime && n > m / 2 { m - n - 1 } else { n };
    if d < 2 {
        return Some(if d == 0 { m - 1 } else { 1 });
    }
    if d > 100
        && !prime
        && factor_exp(m)
            .iter()
            .map(|&(p, e)| p * u64::from(e))
            .max()
            .is_some_and(|bound| n >= bound)
    {
        return Some(0);
    }
    let mut r = 1;
    if d < 1000 {
        for i in 2..=d {
            r = mulmod_u(r, i, m);
            if r == 0 {
                break;
            }
        }
    } else {
        let sqn = sqrtint(d);
        let nsqn = d / sqn;
        let (mut j, mut lo, mut hi, mut product) = (sqn, 0, 0, 1);
        for p in crate::primes_in(2..=d) {
            if p <= nsqn {
                r = mulmod_u(r, powmod_u(p, valuation(d, p), m), m);
            } else {
                while p > hi {
                    r = mulmod_u(r, powmod_u(product, j, m), m);
                    product = 1;
                    j -= 1;
                    lo = d / (j + 1) + 1;
                    hi = d / j;
                }
                if p >= lo {
                    product = mulmod_u(product, p, m);
                }
            }
            if r == 0 {
                break;
            }
        }
        r = mulmod_u(r, product, m);
    }
    if d != n && r != 0 {
        if d % 2 == 0 {
            r = m - r;
        }
        return invmod(r, m);
    }
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segmented_and_wilson_paths_match_products() {
        for m in [10007, 20014, 8192] {
            for n in [0, 1, 10, 101, 1000, 1001, 5004, 10005, 10006] {
                let expected = (2..=n).fold(1 % m, |r, i| mulmod_u(r, i, m));
                assert_eq!(factorialmod(n, m), Some(expected));
            }
        }
    }
}
