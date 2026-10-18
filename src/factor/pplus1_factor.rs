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

#[cfg(test)]
mod tests {
    use super::{lucas_power, pplus1, subtract};

    #[test]
    fn lucas_recurrence_and_small_bounds() {
        let n = 101;
        let mut previous = 2;
        let mut current = 7;
        for exponent in 2..64 {
            let next = (7 * current + n - previous) % n;
            assert_eq!(lucas_power(7, exponent, n), next);
            previous = current;
            current = next;
        }
        assert_eq!(subtract(1, 2, 101), 100);
        assert_eq!(subtract(100, 2, 101), 98);
        assert_eq!(pplus1(49, 0), vec![49]);
        assert_eq!(pplus1(49, 1), vec![49]);
        assert_eq!(pplus1(7, 200), vec![7]);
    }

    #[test]
    fn seeds_do_not_overflow_near_u64_max() {
        // 7 % n and 11 % n must stay reduced: n + 7 would overflow.
        for n in [u64::MAX - 4, u64::MAX - 9, u64::MAX - 12] {
            let got = pplus1(n, 30);
            assert_eq!(got.iter().product::<u64>(), n);
        }
    }

    #[test]
    fn exact_results() {
        use crate::factor::test_support::{fold, wide_inputs};
        let mut h = 0;
        for n in wide_inputs() {
            if n % 2 == 0 {
                continue;
            }
            for bound in [0, 2, 20, 200, 2000] {
                let got = pplus1(n, bound);
                assert_eq!(got.iter().product::<u64>(), n, "pplus1({n},{bound})");
                h = fold(h, 1);
                for f in got {
                    h = fold(h, f);
                }
            }
        }
        assert_eq!(h, 18_287_764_772_764_943_135);
    }
}
