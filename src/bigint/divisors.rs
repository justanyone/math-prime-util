//! Divisor and totient functions on arbitrary precision values: the
//! functions of [`factor`](mod@crate::factor) and [`crate::arith`] with the same names,
//! for [`BigUint`], through [`factor`](super::factor::factor).
//!
//! Port of MPU 0.75 `PP.pm` (`divisors`, `divisor_sum`, `euler_phi`,
//! `carmichael_lambda`, `moebius`, `is_square_free`, `prime_bigomega`,
//! `prime_omega`). Every function needs the factorization of its input, so
//! each returns the cofactor the recipe could not split as its error, as
//! [`factor`](super::factor::factor) does. Below 2^64 the results equal the
//! native ones.
//!
//! ```
//! use math_prime_util::bigint::divisors::{euler_phi, divisor_sum};
//! use num_bigint::BigUint;
//! let m = (BigUint::from(1_u8) << 127_usize) - 1_u8;
//! assert_eq!(euler_phi(&(&m * 4_u8)), Ok((&m - 1_u8) * 2_u8));
//! assert_eq!(divisor_sum(&(&m * &m), 0), Ok(BigUint::from(3_u8)));
//! ```
use super::arith::lcm;
use super::factor::factor_exp;
use num_bigint::BigUint;

fn one() -> BigUint {
    BigUint::from(1_u8)
}

fn is_one(n: &BigUint) -> bool {
    *n == one()
}

/// The divisors of `n` in ascending order, those at most `max` when given.
///
/// Port of MPU 0.75 `PP.pm::divisors`. Zero returns `[]`, as does a bound
/// of zero; one returns `[1]`.
///
/// # Errors
/// Returns the first cofactor the recipe could not split.
///
/// ```
/// use math_prime_util::bigint::divisors::divisors;
/// use num_bigint::BigUint;
/// let n = BigUint::from(12_u8);
/// let got = divisors(&n, None).unwrap();
/// assert_eq!(got, [1_u8, 2, 3, 4, 6, 12].map(BigUint::from));
/// assert_eq!(divisors(&n, Some(&BigUint::from(4_u8))).unwrap(), [1_u8, 2, 3, 4].map(BigUint::from));
/// ```
pub fn divisors(n: &BigUint, max: Option<&BigUint>) -> Result<Vec<BigUint>, BigUint> {
    let max = max.filter(|m| *m < n).unwrap_or(n);
    if n.bits() == 0 || max.bits() == 0 {
        return Ok(Vec::new());
    }
    if is_one(n) || is_one(max) {
        return Ok(vec![one()]);
    }
    let mut result = vec![one()];
    for (p, e) in factor_exp(n)? {
        if p > *max {
            break;
        }
        let count = result.len();
        let mut multiple = one();
        for _ in 0..e {
            multiple *= &p;
            if multiple > *max {
                break;
            }
            for i in 0..count {
                let d = &result[i] * &multiple;
                if d <= *max {
                    result.push(d);
                }
            }
        }
    }
    result.sort_unstable();
    Ok(result)
}

/// The sum of the `k`-th powers of the divisors of `n`, `sigma_k(n)`.
///
/// Port of MPU 0.75 `PP.pm::divisor_sum` for an integer `k`; `k = 0`
/// counts the divisors. Zero returns 0; one returns 1.
///
/// # Errors
/// Returns the first cofactor the recipe could not split.
///
/// ```
/// use math_prime_util::bigint::divisors::divisor_sum;
/// use num_bigint::BigUint;
/// assert_eq!(divisor_sum(&BigUint::from(12_u8), 1), Ok(BigUint::from(28_u8)));
/// assert_eq!(divisor_sum(&BigUint::from(12_u8), 0), Ok(BigUint::from(6_u8)));
/// ```
pub fn divisor_sum(n: &BigUint, k: u64) -> Result<BigUint, BigUint> {
    if *n <= one() {
        return Ok(n.clone());
    }
    let mut product = one();
    for (p, e) in factor_exp(n)? {
        if k == 0 {
            product *= BigUint::from(e) + 1_u8;
            continue;
        }
        let pk = p.pow(u32::try_from(k).unwrap_or(u32::MAX));
        let mut power = one();
        let mut sum = one();
        for _ in 0..e {
            power *= &pk;
            sum += &power;
        }
        product *= sum;
    }
    Ok(product)
}

/// Euler's totient, the count of integers in `1..=n` coprime to `n`.
///
/// Port of MPU 0.75 `PP.pm::euler_phi`. Zero returns 0.
///
/// # Errors
/// Returns the first cofactor the recipe could not split.
///
/// ```
/// use math_prime_util::bigint::divisors::euler_phi;
/// use num_bigint::BigUint;
/// assert_eq!(euler_phi(&BigUint::from(36_u8)), Ok(BigUint::from(12_u8)));
/// ```
pub fn euler_phi(n: &BigUint) -> Result<BigUint, BigUint> {
    if *n <= one() {
        return Ok(n.clone());
    }
    let mut total = one();
    for (p, e) in factor_exp(n)? {
        total *= (&p - 1_u8) * p.pow(e - 1);
    }
    Ok(total)
}

/// Carmichael's function, the exponent of the multiplicative group mod `n`.
///
/// Port of MPU 0.75 `PP.pm::carmichael_lambda`. Zero returns 0.
///
/// # Errors
/// Returns the first cofactor the recipe could not split.
///
/// ```
/// use math_prime_util::bigint::divisors::carmichael_lambda;
/// use num_bigint::BigUint;
/// assert_eq!(carmichael_lambda(&BigUint::from(36_u8)), Ok(BigUint::from(6_u8)));
/// assert_eq!(carmichael_lambda(&BigUint::from(16_u8)), Ok(BigUint::from(4_u8)));
/// ```
pub fn carmichael_lambda(n: &BigUint) -> Result<BigUint, BigUint> {
    if *n < BigUint::from(8_u8) {
        return euler_phi(n);
    }
    if n.count_ones() == 1 {
        return Ok(n >> 2_u32);
    }
    let mut result = one();
    for (p, mut e) in factor_exp(n)? {
        if p == BigUint::from(2_u8) && e > 2 {
            e -= 1;
        }
        result = lcm(&result, &((&p - 1_u8) * p.pow(e - 1)));
    }
    Ok(result)
}

/// The Moebius function: 0 for a square factor, else `(-1)^omega(n)`.
///
/// Port of MPU 0.75 `PP.pm::moebius`. Zero returns 0. A repeated factor
/// among the primes below 5000 answers 0 without the full factorization.
///
/// # Errors
/// Returns the first cofactor the recipe could not split.
///
/// ```
/// use math_prime_util::bigint::divisors::moebius;
/// use num_bigint::BigUint;
/// assert_eq!(moebius(&BigUint::from(30_u8)), Ok(-1));
/// assert_eq!(moebius(&BigUint::from(12_u8)), Ok(0));
/// ```
pub fn moebius(n: &BigUint) -> Result<i8, BigUint> {
    if *n <= one() {
        return Ok(i8::from(is_one(n)));
    }
    if small_square(n) {
        return Ok(0);
    }
    let factors = factor_exp(n)?;
    if factors.iter().any(|(_, e)| *e > 1) {
        Ok(0)
    } else if factors.len() % 2 == 0 {
        Ok(1)
    } else {
        Ok(-1)
    }
}

/// True when a prime below 5000 divides `n` twice.
fn small_square(n: &BigUint) -> bool {
    let small = super::factor::trial_factor(n, Some(4999));
    small.windows(2).any(|pair| pair[0] == pair[1])
}

/// True when no square above 1 divides `n`.
///
/// Port of MPU 0.75 `PP.pm::is_square_free` (`moebius(n) != 0`). Zero
/// returns false; one returns true.
///
/// # Errors
/// Returns the first cofactor the recipe could not split.
///
/// ```
/// use math_prime_util::bigint::divisors::is_square_free;
/// use num_bigint::BigUint;
/// assert_eq!(is_square_free(&BigUint::from(30_u8)), Ok(true));
/// assert_eq!(is_square_free(&BigUint::from(12_u8)), Ok(false));
/// ```
pub fn is_square_free(n: &BigUint) -> Result<bool, BigUint> {
    Ok(moebius(n)? != 0)
}

/// The number of prime factors of `n` with multiplicity, `Omega(n)`.
///
/// Port of MPU 0.75 `PP.pm::prime_bigomega`. Zero returns 1 (the one
/// "factor" 0, as `factor(0)` gives `[0]`); one returns 0.
///
/// # Errors
/// Returns the first cofactor the recipe could not split.
///
/// ```
/// use math_prime_util::bigint::divisors::prime_bigomega;
/// use num_bigint::BigUint;
/// assert_eq!(prime_bigomega(&BigUint::from(360_u16)), Ok(6));
/// ```
pub fn prime_bigomega(n: &BigUint) -> Result<u64, BigUint> {
    Ok(factor_exp(n)?.iter().map(|(_, e)| u64::from(*e)).sum())
}

/// The number of distinct prime factors of `n`, `omega(n)`.
///
/// Port of MPU 0.75 `PP.pm::prime_omega`. Zero returns 1; one returns 0.
///
/// # Errors
/// Returns the first cofactor the recipe could not split.
///
/// ```
/// use math_prime_util::bigint::divisors::prime_omega;
/// use num_bigint::BigUint;
/// assert_eq!(prime_omega(&BigUint::from(360_u16)), Ok(3));
/// ```
pub fn prime_omega(n: &BigUint) -> Result<u64, BigUint> {
    Ok(u64::try_from(factor_exp(n)?.len()).unwrap_or(u64::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn big(n: u64) -> BigUint {
        BigUint::from(n)
    }

    fn mersenne(bits: usize) -> BigUint {
        (one() << bits) - 1_u8
    }

    #[test]
    fn small_values_match_the_native_path() {
        for n in 0..=2000_u64 {
            let b = big(n);
            assert_eq!(euler_phi(&b), Ok(big(crate::euler_phi(n))), "phi {n}");
            assert_eq!(
                carmichael_lambda(&b),
                Ok(big(crate::carmichael_lambda(n))),
                "lambda {n}"
            );
            assert_eq!(moebius(&b), Ok(crate::moebius(n)), "mu {n}");
            assert_eq!(
                is_square_free(&b),
                Ok(crate::is_square_free(n)),
                "sqfree {n}"
            );
            assert_eq!(
                divisors(&b, None),
                Ok(crate::divisors(n, None).into_iter().map(big).collect()),
                "divisors {n}"
            );
            for k in 0..3 {
                assert_eq!(
                    divisor_sum(&b, k).map(|v| v.to_string()),
                    Ok(crate::divisor_sum(n, k).unwrap().to_string()),
                    "sigma_{k} {n}"
                );
            }
            if n > 0 {
                assert_eq!(
                    prime_bigomega(&b),
                    Ok(u64::from(crate::prime_bigomega(n))),
                    "Omega {n}"
                );
                assert_eq!(
                    prime_omega(&b),
                    Ok(u64::from(crate::prime_omega(n))),
                    "omega {n}"
                );
            }
        }
        assert_eq!(prime_bigomega(&big(0)), Ok(1));
        assert_eq!(prime_omega(&big(0)), Ok(1));
        assert_eq!(divisors(&big(0), None), Ok(vec![]));
        assert_eq!(divisors(&big(12), Some(&big(0))), Ok(vec![]));
        assert_eq!(divisors(&big(12), Some(&one())), Ok(vec![one()]));
        assert_eq!(
            divisors(&big(12), Some(&big(100))),
            divisors(&big(12), None)
        );
        assert_eq!(
            divisors(&big(12), Some(&big(5))),
            Ok(vec![one(), big(2), big(3), big(4)])
        );
        assert_eq!(divisors(&big(12), Some(&big(2))), Ok(vec![one(), big(2)]));
        assert_eq!(divisor_sum(&big(0), 3), Ok(big(0)));
        assert_eq!(divisor_sum(&big(2), 40), Ok((one() << 40_usize) + 1_u8));
    }

    #[test]
    fn big_values() {
        let m = mersenne(127);
        let n = &m * &m * 12_u8;
        assert_eq!(euler_phi(&n), Ok(&m * (&m - 1_u8) * 4_u8));
        assert_eq!(carmichael_lambda(&n), Ok(&m * (&m - 1_u8)));
        assert_eq!(moebius(&n), Ok(0));
        assert_eq!(moebius(&(&m * 6_u8)), Ok(-1));
        assert_eq!(moebius(&(&m * 30_u8)), Ok(1));
        assert_eq!(moebius(&(&m * 210_u8)), Ok(-1));
        assert_eq!(moebius(&(&m * 9_u8)), Ok(0));
        assert!(small_square(&(&m * 9_u8)));
        assert!(!small_square(&(&m * &m)));
        assert_eq!(is_square_free(&n), Ok(false));
        assert_eq!(is_square_free(&(&m * 6_u8)), Ok(true));
        assert_eq!(prime_bigomega(&n), Ok(5));
        assert_eq!(prime_omega(&n), Ok(3));
        assert_eq!(divisor_sum(&n, 0), Ok(big(18)));
        assert_eq!(divisor_sum(&(&m * 2_u8), 1), Ok((&m + 1_u8) * 3_u8));
        assert_eq!(
            divisor_sum(&(&m * &m), 2),
            Ok(one() + &m * &m + &m * &m * &m * &m)
        );
        let d = divisors(&n, None).unwrap();
        assert_eq!(d.len(), 18);
        assert_eq!(d[17], n);
        assert_eq!(
            divisors(&n, Some(&big(100))).unwrap(),
            [1_u8, 2, 3, 4, 6, 12].map(BigUint::from)
        );
        assert_eq!(divisors(&n, Some(&(&m * 2_u8))).unwrap().len(), 8);
        // A power of two above 2^64.
        let two_100 = one() << 100_usize;
        assert_eq!(carmichael_lambda(&two_100), Ok(one() << 98_usize));
        assert_eq!(euler_phi(&two_100), Ok(one() << 99_usize));
        assert_eq!(carmichael_lambda(&(&two_100 * 3_u8)), Ok(one() << 98_usize));
        assert_eq!(
            divisors(&two_100, Some(&big(8))).unwrap(),
            [1_u8, 2, 4, 8].map(BigUint::from)
        );
        // An unsplit cofactor is the error of every function.
        let hard: BigUint = "383939849620258888044469981".parse::<BigUint>().unwrap()
            * "128630034069733398025664261656771"
                .parse::<BigUint>()
                .unwrap();
        let cheap =
            super::super::factor::factor_within(&hard, std::time::Duration::from_millis(20));
        assert_eq!(cheap.unsplit, vec![hard.clone()]);
        assert_eq!(moebius(&(&hard * 4_u8)), Ok(0));
        assert_eq!(is_square_free(&(&hard * 4_u8)), Ok(false));
    }
}
