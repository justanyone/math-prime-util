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
/// Port of MPU 0.75 `XS.xs::abundance` and `factor.c::aliquot_sum`.
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

/// Return the Ramanujan sum `c_k(n)`.
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
/// Port of MPU 0.75 `totients.c::inverse_totient_list`.
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
    if n.bits() == 0 || (*n > BigUint::from(1_u8) && n.bit(0)) {
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

/// Return the integers in `lo..=hi` with exactly `k` divisors, in ascending
/// order.
///
/// Port of MPU 0.75 `PP.pm::inverse_sigma0` and `_inverse_sigma0_generate`,
/// the path MPU takes past 2^64. `k = 1` and `k = 2` give 1 and the primes.
/// A wide range (`lo <= hi / 2`) is built from the prime signatures that `k`
/// allows, with a native prime sieve and count below 2^64. A narrow range is
/// factored value by value, so keep it short. Wide ranges past 2^64 are
/// practical only when the signatures of `k` are sparse, such as prime `k`.
/// `k = 0` gives an empty list (MPU's `[]`); the native form returns `None`.
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::inverse_sigma0;
/// use num_bigint::BigUint;
/// let r = inverse_sigma0(&4_u8.into(), &1_u8.into(), &10_u8.into()).unwrap();
/// assert_eq!(r, [6_u8, 8, 10].map(BigUint::from));
/// // Each p^4 has five divisors: 89^4 and 97^4 fall in [2^24, 2^27].
/// let r = inverse_sigma0(&5_u8.into(), &(1_u32 << 24).into(), &(1_u32 << 27).into()).unwrap();
/// assert_eq!(r.len(), 7);
/// ```
pub fn inverse_sigma0(k: &BigUint, lo: &BigUint, hi: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    let mut out = Sigma0Out::List(Vec::new());
    sigma0_search(k, lo, hi, &mut out)?;
    let Sigma0Out::List(mut list) = out else {
        unreachable!("list output")
    };
    list.sort_unstable();
    Ok(list)
}

/// Return how many integers in `lo..=hi` have exactly `k` divisors.
///
/// Port of MPU 0.75 `PP.pm::inverse_sigma0_count`; see [`inverse_sigma0`]
/// for the method and its limits. Prime counts below 2^64 use the native
/// [`prime_count`](crate::prime_count).
///
/// # Errors
/// Return the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::functions::inverse_sigma0_count;
/// use num_bigint::BigUint;
/// // n = p^2 below 2^70: one for each prime up to 2^35.
/// let hi = BigUint::from(1_u8) << 70_usize;
/// let c = inverse_sigma0_count(&3_u8.into(), &1_u8.into(), &hi).unwrap();
/// assert_eq!(c, BigUint::from(1_480_206_279_u32));
/// ```
pub fn inverse_sigma0_count(k: &BigUint, lo: &BigUint, hi: &BigUint) -> Result<BigUint, BigUint> {
    let mut out = Sigma0Out::Count(BigUint::ZERO);
    sigma0_search(k, lo, hi, &mut out)?;
    let Sigma0Out::Count(count) = out else {
        unreachable!("count output")
    };
    Ok(count)
}

enum Sigma0Out {
    List(Vec<BigUint>),
    Count(BigUint),
}

/// Primes in `lo..=hi`: the native sieve below 2^64, else `bigint::families`.
fn primes_between(lo: &BigUint, hi: &BigUint) -> Vec<BigUint> {
    match (u64::try_from(lo), u64::try_from(hi)) {
        (Ok(a), Ok(b)) => crate::primes(a, b).into_iter().map(BigUint::from).collect(),
        _ => super::families::primes(lo, hi),
    }
}

/// Number of primes in `lo..=hi`: the native count below 2^64.
fn prime_count_between(lo: &BigUint, hi: &BigUint) -> Result<BigUint, BigUint> {
    if lo > hi {
        return Ok(BigUint::ZERO);
    }
    match (u64::try_from(lo), u64::try_from(hi)) {
        (Ok(a), Ok(b)) => Ok(BigUint::from(
            crate::prime_count(b) - crate::prime_count(a.max(1) - 1),
        )),
        _ => super::family_counts::prime_count(lo, hi),
    }
}

// `PP.pm::inverse_sigma0` and `inverse_sigma0_count`: the early returns, then
// the signature generator for wide ranges or a factored scan for narrow ones.
fn sigma0_search(
    k: &BigUint,
    lo: &BigUint,
    hi: &BigUint,
    out: &mut Sigma0Out,
) -> Result<(), BigUint> {
    let one = BigUint::from(1_u8);
    if k.bits() == 0 || *hi < one {
        return Ok(());
    }
    if *k == one {
        if *lo <= one {
            sigma0_emit(out, &one);
        }
        return Ok(());
    }
    if *k == BigUint::from(2_u8) {
        match out {
            Sigma0Out::List(list) => list.extend(primes_between(lo, hi)),
            Sigma0Out::Count(count) => *count += prime_count_between(lo, hi)?,
        }
        return Ok(());
    }
    let lo = lo.max(&one);
    if lo > hi || k > hi {
        return Ok(());
    }
    if let (Ok(k), true) = (u64::try_from(k), *lo <= hi >> 1_usize) {
        let mut minsum = BTreeMap::new();
        return sigma0_generate(&one, &BigUint::from(2_u8), k, (lo, hi), &mut minsum, out);
    }
    let mut n = lo.clone();
    while n <= *hi {
        let tau: BigUint = factor_exp(&n)?
            .iter()
            .map(|(_, e)| BigUint::from(e + 1))
            .product();
        if tau == *k {
            sigma0_emit(out, &n);
        }
        n += 1_u8;
    }
    Ok(())
}

fn sigma0_emit(out: &mut Sigma0Out, n: &BigUint) {
    match out {
        Sigma0Out::List(list) => list.push(n.clone()),
        Sigma0Out::Count(count) => *count += 1_u8,
    }
}

// `PP.pm::_inverse_sigma0_generate`: `n` so far, the least prime left, and the
// divisor count `rem` still to place. Each divisor `d` of `rem` puts `p^(d-1)`.
fn sigma0_generate(
    n: &BigUint,
    pmin: &BigUint,
    rem: u64,
    range: (&BigUint, &BigUint),
    minsum: &mut BTreeMap<u64, u64>,
    out: &mut Sigma0Out,
) -> Result<(), BigUint> {
    let (low, high) = range;
    if rem == 1 {
        if n >= low {
            sigma0_emit(out, n);
        }
        return Ok(());
    }
    let exp = |e: u64| u32::try_from(e).unwrap_or(u32::MAX);
    let ds: Vec<u64> = crate::divisors(rem, None)
        .into_iter()
        .filter(|&d| d >= 2)
        .collect();
    let maxpow = high / n;
    if ds.len() == 1 {
        // `rem` is prime: the last exponent is rem - 1 on one prime.
        let e = exp(rem - 1);
        let minpow = if n >= low {
            BigUint::from(1_u8)
        } else {
            (low + n - 1_u8) / n
        };
        let (plo, phi) = if e == 1 {
            (minpow, maxpow)
        } else {
            (crootint(&minpow, e), rootint(&maxpow, e))
        };
        let plo = plo.max(pmin.clone());
        if plo > phi {
            return Ok(());
        }
        match out {
            Sigma0Out::List(list) => {
                list.extend(primes_between(&plo, &phi).into_iter().map(|p| n * p.pow(e)));
            }
            Sigma0Out::Count(count) => *count += prime_count_between(&plo, &phi)?,
        }
        return Ok(());
    }
    for d in ds {
        let (e, next) = (d - 1, rem / d);
        let least = *minsum
            .entry(next)
            .or_insert_with(|| crate::factor(next).iter().map(|f| f - 1).sum());
        let phi = rootint(&maxpow, exp(e.saturating_add(least)));
        if *pmin > phi {
            continue;
        }
        for p in primes_between(pmin, &phi) {
            let v = n * p.pow(exp(e));
            sigma0_generate(&v, &(p + 1_u8), next, range, minsum, out)?;
        }
    }
    Ok(())
}
