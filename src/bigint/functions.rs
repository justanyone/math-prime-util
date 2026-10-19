//! Functions of an arbitrary precision integer and its factorization.
//! Factorization errors return the cofactor that could not be split.
use super::arith::{gcd, rootint};
use super::divisors::{divisor_sum, divisors, euler_phi, moebius};
use super::factor::factor_exp;
use super::primality::is_prob_prime;
use num_bigint::{BigInt, BigUint};
use std::collections::BTreeMap;

fn pow(n: &BigUint, mut k: u64) -> BigUint {
    let mut result = BigUint::from(1_u8);
    let mut base = n.clone();
    while k != 0 {
        if k & 1 != 0 {
            result *= &base;
        }
        k >>= 1;
        if k != 0 {
            base = &base * &base;
        }
    }
    result
}

/// Return the sum of the proper divisors.
///
/// Port of MPU 0.75 `factor.c::aliquot_sum`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::aliquot_sum;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(aliquot_sum(&n).unwrap(), BigUint::from(16_u8));
/// ```
pub fn aliquot_sum(n: &BigUint) -> Result<BigUint, BigUint> {
    if n.bits() == 0 {
        return Ok(n.clone());
    }
    Ok(divisor_sum(n, 1)? - n)
}

/// Return sigma(n) minus twice n.
///
/// Port of MPU 0.75 `factor.c::aliquot_sum`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::abundance;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(abundance(&n).unwrap(), BigInt::from(4));
/// ```
pub fn abundance(n: &BigUint) -> Result<BigInt, BigUint> {
    Ok(BigInt::from(aliquot_sum(n)?) - BigInt::from(n.clone()))
}

/// Return the ceiling of the kth root.
///
/// Port of MPU 0.75 `util.c::rootint`.
///
/// # Panics
/// Panic if the degree is zero or the removal factor is below two.
///
/// ```
/// use math_prime_util::bigint::functions::crootint;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(crootint(&n, 2), BigUint::from(4_u8));
/// ```
pub fn crootint(n: &BigUint, k: u32) -> BigUint {
    assert!(k != 0, "crootint: k must be positive");
    let r = rootint(n, k);
    if r.pow(k) < *n { r + 1_u8 } else { r }
}

/// Return the Jordan totient of order k.
///
/// Port of MPU 0.75 `totients.c::jordan_totient`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::jordan_totient;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(jordan_totient(2, &n).unwrap(), BigUint::from(96_u8));
/// ```
pub fn jordan_totient(k: u64, n: &BigUint) -> Result<BigUint, BigUint> {
    if k == 0 || *n <= BigUint::from(1_u8) {
        return Ok(BigUint::from(u8::from(*n == BigUint::from(1_u8))));
    }
    let mut result = BigUint::from(1_u8);
    for (p, e) in factor_exp(n)? {
        let pk = pow(&p, k);
        result *= (&pk - 1_u8) * pk.pow(e - 1);
    }
    Ok(result)
}

/// Return the Dedekind psi function.
///
/// Port of MPU 0.75 `totients.c::dedekind_psi`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::dedekind_psi;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(dedekind_psi(&n).unwrap(), BigUint::from(24_u8));
/// ```
pub fn dedekind_psi(n: &BigUint) -> Result<BigUint, BigUint> {
    let mut result = n.clone();
    if n.bits() != 0 {
        for (p, _) in factor_exp(n)? {
            result = result / &p * (&p + 1_u8);
        }
    }
    Ok(result)
}

/// Return the prime base of a prime power, or one.
///
/// Port of MPU 0.75 `util.c::exp_mangoldt`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::exp_mangoldt;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(exp_mangoldt(&n).unwrap(), BigUint::from(1_u8));
/// ```
pub fn exp_mangoldt(n: &BigUint) -> Result<BigUint, BigUint> {
    if *n < BigUint::from(2_u8) {
        return Ok(BigUint::from(1_u8));
    }
    let factors = factor_exp(n)?;
    Ok(if factors.len() == 1 {
        factors[0].0.clone()
    } else {
        BigUint::from(1_u8)
    })
}

/// Return minus one to the number of prime factors.
///
/// Port of MPU 0.75 `moebius.c::liouville`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::liouville;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(liouville(&n).unwrap(), -1);
/// ```
pub fn liouville(n: &BigUint) -> Result<i8, BigUint> {
    let odd = factor_exp(n)?
        .iter()
        .fold(false, |odd, (_, e)| odd ^ (e % 2 != 0));
    Ok(if odd { -1 } else { 1 })
}

/// Remove each complete kth power factor.
///
/// Port of MPU 0.75 `powerfree.c::powerfree_part`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::powerfree_part;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(powerfree_part(&n, 2).unwrap(), BigUint::from(3_u8));
/// ```
pub fn powerfree_part(n: &BigUint, k: u32) -> Result<BigUint, BigUint> {
    if k < 2 || *n <= BigUint::from(1_u8) {
        return Ok(BigUint::from(u8::from(*n == BigUint::from(1_u8))));
    }
    Ok(factor_exp(n)?
        .into_iter()
        .map(|(p, e)| p.pow(e % k))
        .product())
}

/// Return prime exponents in descending order.
///
/// Port of MPU 0.75 `XS.xs::prime_signature`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::prime_signature;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(prime_signature(&n).unwrap(), vec![2, 1]);
/// ```
pub fn prime_signature(n: &BigUint) -> Result<Vec<u32>, BigUint> {
    let mut result: Vec<_> = factor_exp(n)?.into_iter().map(|(_, e)| e).collect();
    result.sort_unstable_by(|a, b| b.cmp(a));
    Ok(result)
}

/// Return the product of the distinct prime factors.
///
/// Port of MPU 0.75 `powerfree.c::squarefree_kernel`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::squarefree_kernel;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(squarefree_kernel(&n).unwrap(), BigUint::from(6_u8));
/// ```
pub fn squarefree_kernel(n: &BigUint) -> Result<BigUint, BigUint> {
    Ok(factor_exp(n)?.into_iter().map(|(p, _)| p).product())
}

/// Return the sum of the distinct prime factors.
///
/// Port of MPU 0.75 `factor.c::sopf`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::sopf;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(sopf(&n).unwrap(), BigUint::from(5_u8));
/// ```
pub fn sopf(n: &BigUint) -> Result<BigUint, BigUint> {
    Ok(factor_exp(n)?.into_iter().map(|(p, _)| p).sum())
}

/// Return the sum of prime factors with repetition.
///
/// Port of MPU 0.75 `factor.c::sopfr`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::sopfr;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(sopfr(&n).unwrap(), BigUint::from(7_u8));
/// ```
pub fn sopfr(n: &BigUint) -> Result<BigUint, BigUint> {
    Ok(factor_exp(n)?.into_iter().map(|(p, e)| p * e).sum())
}

/// Return the Ramanujan sum c_k(n).
///
/// Port of MPU 0.75 `XS.xs::ramanujan_sum`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::ramanujan_sum;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(ramanujan_sum(&n, &n).unwrap(), BigInt::from(4));
/// ```
pub fn ramanujan_sum(k: &BigUint, n: &BigUint) -> Result<BigInt, BigUint> {
    if k.bits() == 0 || n.bits() == 0 {
        return Ok(BigInt::from(0));
    }
    let g = k / gcd(k, n);
    let m = moebius(&g)?;
    if m == 0 {
        return Ok(BigInt::from(0));
    }
    Ok(BigInt::from(m) * BigInt::from(euler_phi(k)? / euler_phi(&g)?))
}

/// Remove exact factors and return the remainder and count.
///
/// Port of MPU 0.75 `util.c::valuation_remainder`.
///
/// # Panics
/// Panic if the degree is zero or the removal factor is below two.
///
/// ```
/// use math_prime_util::bigint::functions::remove_factors_exp;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(remove_factors_exp(&n, &BigUint::from(2_u8)), Some((BigUint::from(3_u8), 2)));
/// ```
pub fn remove_factors_exp(n: &BigUint, k: &BigUint) -> Option<(BigUint, u64)> {
    assert!(
        *k > BigUint::from(1_u8),
        "remove_factors_exp: k must exceed one"
    );
    if n.bits() == 0 {
        return None;
    }
    let mut rest = n.clone();
    let mut count = 0;
    while (&rest % k).bits() == 0 {
        rest /= k;
        count += 1;
    }
    Some((rest, count))
}

/// Remove all exact factors of k.
///
/// Port of MPU 0.75 `util.c::valuation_remainder`.
///
/// # Panics
/// Panic if the degree is zero or the removal factor is below two.
///
/// ```
/// use math_prime_util::bigint::functions::remove_factors;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(remove_factors(&n, &BigUint::from(2_u8)), Some(BigUint::from(3_u8)));
/// ```
pub fn remove_factors(n: &BigUint, k: &BigUint) -> Option<BigUint> {
    remove_factors_exp(n, k).map(|(rest, _)| rest)
}

/// Return all integers whose totient is n.
///
/// Port of MPU 0.75 `totients.c::inverse_totient`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::inverse_totient;
/// use num_bigint::{BigInt, BigUint};
/// let n = BigUint::from(12_u8);
/// assert_eq!(inverse_totient(&BigUint::from(1_u8)).unwrap(), vec![BigUint::from(1_u8), BigUint::from(2_u8)]);
/// ```
pub fn inverse_totient(n: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    if n.bits() == 0 {
        return Ok(Vec::new());
    }
    let mut values = BTreeMap::from([(BigUint::from(1_u8), vec![BigUint::from(1_u8)])]);
    // PP.pm::inverse_totient builds sparse lists indexed by totient divisors.
    for d in divisors(n, None)? {
        let p = &d + 1_u8;
        if is_prob_prime(&p) == 0 {
            continue;
        }
        let previous = values.clone();
        let mut dp = d;
        let mut pp = p.clone();
        while (n % &dp).bits() == 0 {
            for (v, list) in &previous {
                let target = v * &dp;
                if target <= *n && (n % &target).bits() == 0 {
                    values
                        .entry(target)
                        .or_default()
                        .extend(list.iter().map(|x| x * &pp));
                }
            }
            dp *= &p;
            pp *= &p;
        }
    }
    let mut result = values.remove(n).unwrap_or_default();
    result.sort_unstable();
    Ok(result)
}
