//! Exact family counts for arbitrary precision integers.
//!
//! Prefix root sums take time proportional to the integer kth root.
//! Keep range forms short. Factor failures return the unresolved cofactor.
use super::{arith::rootint, divisors};
use num_bigint::{BigInt, BigUint};

/// Local zero and one helpers: `num-traits` is not a direct dependency.
trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

/// Local one helper.
trait One {
    fn one() -> Self;
}

impl Zero for BigInt {
    fn zero() -> Self {
        Self::from(0_u8)
    }
    fn is_zero(&self) -> bool {
        *self == Self::from(0_u8)
    }
}

impl Zero for BigUint {
    fn zero() -> Self {
        Self::from(0_u8)
    }
    fn is_zero(&self) -> bool {
        *self == Self::from(0_u8)
    }
}

impl One for BigUint {
    fn one() -> Self {
        Self::from(1_u8)
    }
}

impl One for BigInt {
    fn one() -> Self {
        Self::from(1_u8)
    }
}

/// Count positive perfect powers at most `n`. Include one once.
///
/// Port of MPU 0.75 `perfect_powers.c::perfect_power_count`.
///
/// ```
/// use math_prime_util::bigint::family_counts::perfect_power_count;
/// use num_bigint::BigUint;
/// assert_eq!(perfect_power_count(&BigUint::from(16_u8)), BigUint::from(5_u8));
/// ```
pub fn perfect_power_count(n: &BigUint) -> BigUint {
    if n.is_zero() {
        return BigUint::zero();
    }
    let mut sum = BigInt::one();
    for k in 2..n.bits() {
        let Ok(exponent) = u32::try_from(k) else {
            break;
        };
        let mu = crate::moebius(k);
        if mu != 0 {
            sum -= BigInt::from(mu) * BigInt::from(rootint(n, exponent) - 1_u8);
        }
    }
    sum.magnitude().clone()
}

fn powerfree(n: &BigUint, k: u32, weighted: bool) -> Result<BigUint, BigUint> {
    if k < 2 {
        return Ok(BigUint::from(u8::from(!n.is_zero())));
    }
    let mut i = BigUint::one();
    let top = rootint(n, k);
    let mut sum = BigInt::zero();
    while i <= top {
        let mu = divisors::moebius(&i)?;
        if mu != 0 {
            let power = i.pow(k);
            let q = n / &power;
            let term = if weighted {
                power * (&q * (&q + 1_u8) / 2_u8)
            } else {
                q
            };
            sum += BigInt::from(mu) * BigInt::from(term);
        }
        i += 1_u8;
    }
    Ok(sum.magnitude().clone())
}

/// Count positive kth-powerfree integers at most `n`.
///
/// Port of MPU 0.75 `powerfree.c::powerfree_count`.
/// Cost grows with the kth root. Use large `k` for large inputs.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::powerfree_count;
/// use num_bigint::BigUint;
/// assert_eq!(powerfree_count(&10_u8.into(), 2).expect("factors"), BigUint::from(7_u8));
/// ```
pub fn powerfree_count(n: &BigUint, k: u32) -> Result<BigUint, BigUint> {
    powerfree(n, k, false)
}

/// Sum positive kth-powerfree integers at most `n`.
///
/// Port of MPU 0.75 `powerfree.c::powerfree_sum`.
/// Cost grows with the kth root. Use large `k` for large inputs.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::powerfree_sum;
/// use num_bigint::BigUint;
/// assert_eq!(powerfree_sum(&10_u8.into(), 2).expect("factors"), BigUint::from(34_u8));
/// ```
pub fn powerfree_sum(n: &BigUint, k: u32) -> Result<BigUint, BigUint> {
    powerfree(n, k, true)
}

fn phi(n: &BigUint, primes: &[u64]) -> BigUint {
    if n.is_zero() {
        return BigUint::zero();
    }
    if primes.last().is_some_and(|&p| *n <= BigUint::from(p)) {
        return BigUint::one();
    }
    let mut count = n.clone();
    for (i, &p) in primes.iter().enumerate() {
        count -= phi(&(n / p), &primes[..i]);
    }
    count
}

/// Count integers at most `n` coprime to the first `a` primes.
///
/// Port of MPU 0.75 `legendre_phi.c::legendre_phi`, by its recurrence.
/// Keep `a` small. Cost grows exponentially with `a`.
///
/// ```
/// use math_prime_util::bigint::family_counts::legendre_phi;
/// use num_bigint::BigUint;
/// assert_eq!(legendre_phi(&100_u8.into(), 3), BigUint::from(26_u8));
/// ```
pub fn legendre_phi(n: &BigUint, a: u64) -> BigUint {
    let mut primes = Vec::new();
    let mut p = 2_u64;
    for _ in 0..a {
        primes.push(p);
        p = crate::next_prime(p).unwrap_or(u64::MAX);
    }
    phi(n, &primes)
}

/// Count integers at most `n` with no prime factor below `k`.
///
/// Port of MPU 0.75 `legendre_phi.c::legendre_phi` and `PP.pm::rough_count`.
/// Keep the prime bound small.
///
/// ```
/// use math_prime_util::bigint::family_counts::rough_count;
/// use num_bigint::BigUint;
/// assert_eq!(rough_count(&30_u8.into(), 5), BigUint::from(10_u8));
/// ```
pub fn rough_count(n: &BigUint, k: u64) -> BigUint {
    if k <= 2 {
        return n.clone();
    }
    phi(n, &crate::primes(2, k - 1))
}

fn psi(n: &BigUint, primes: &[u64]) -> BigUint {
    if n.is_zero() {
        return BigUint::zero();
    }
    let Some((&p, rest)) = primes.split_last() else {
        return BigUint::one();
    };
    let mut x = n.clone();
    let mut sum = BigUint::zero();
    while !x.is_zero() {
        sum += psi(&x, rest);
        x /= p;
    }
    sum
}

/// Count integers at most `n` with prime factors at most `k`.
///
/// Port of MPU 0.75 `util.c::debruijn_psi`, by its prime-power recurrence.
/// Keep the prime bound small.
///
/// ```
/// use math_prime_util::bigint::family_counts::smooth_count;
/// use num_bigint::BigUint;
/// assert_eq!(smooth_count(&100_u8.into(), 5), BigUint::from(34_u8));
/// ```
pub fn smooth_count(n: &BigUint, k: u64) -> BigUint {
    psi(n, &crate::primes(2, k))
}

fn fold<T>(
    low: &BigUint,
    high: &BigUint,
    mut sum: T,
    mut step: impl FnMut(&BigUint, &mut T) -> Result<(), BigUint>,
) -> Result<T, BigUint> {
    let mut n = low.clone();
    while n <= *high {
        step(&n, &mut sum)?;
        n += 1_u8;
    }
    Ok(sum)
}

/// Return the count or sum over the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `semi_primes.c::_range_semiprime_count_iterate`, using direct iteration.
/// Keep ranges short. Prime tests follow the BPSW contract of `bigint::primality`.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::semiprime_count;
/// use num_bigint::BigUint;
/// assert_eq!(semiprime_count(&1_u8.into(), &0_u8.into()).expect("empty"), BigUint::from(0_u8));
/// ```
pub fn semiprime_count(low: &BigUint, high: &BigUint) -> Result<BigUint, BigUint> {
    fold(low, high, BigUint::zero(), |n, sum| {
        *sum += BigUint::from(u8::from(!n.is_zero() && divisors::prime_bigomega(n)? == 2));
        Ok(())
    })
}

/// Return the count or sum over the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `prime_counts.c::prime_count_range`, using direct iteration.
/// Keep ranges short. Prime tests follow the BPSW contract of `bigint::primality`.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::prime_count;
/// use num_bigint::BigUint;
/// assert_eq!(prime_count(&1_u8.into(), &0_u8.into()).expect("empty"), BigUint::from(0_u8));
/// ```
pub fn prime_count(low: &BigUint, high: &BigUint) -> Result<BigUint, BigUint> {
    fold(low, high, BigUint::zero(), |n, sum| {
        *sum += BigUint::from(u8::from(super::primality::is_prime(n) != 0));
        Ok(())
    })
}

/// Return the count or sum over the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `prime_sums.c::sum_primes`, using direct iteration.
/// Keep ranges short. Prime tests follow the BPSW contract of `bigint::primality`.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::sum_primes;
/// use num_bigint::BigUint;
/// assert_eq!(sum_primes(&1_u8.into(), &0_u8.into()).expect("empty"), BigUint::from(0_u8));
/// ```
pub fn sum_primes(low: &BigUint, high: &BigUint) -> Result<BigUint, BigUint> {
    fold(low, high, BigUint::zero(), |n, sum| {
        *sum += if super::primality::is_prime(n) != 0 {
            n.clone()
        } else {
            BigUint::zero()
        };
        Ok(())
    })
}

/// Return the count or sum over the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `totients.c::sumtotient`, using direct iteration.
/// Keep ranges short. Prime tests follow the BPSW contract of `bigint::primality`.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::sumtotient;
/// use num_bigint::BigUint;
/// assert_eq!(sumtotient(&1_u8.into(), &0_u8.into()).expect("empty"), BigUint::from(0_u8));
/// ```
pub fn sumtotient(low: &BigUint, high: &BigUint) -> Result<BigUint, BigUint> {
    fold(low, high, BigUint::zero(), |n, sum| {
        *sum += divisors::euler_phi(n)?;
        Ok(())
    })
}

/// Return the count or sum over the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `moebius.c::mertens`, using direct iteration.
/// Keep ranges short. Prime tests follow the BPSW contract of `bigint::primality`.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::mertens;
/// use num_bigint::BigInt;
/// assert_eq!(mertens(&1_u8.into(), &0_u8.into()).expect("empty"), BigInt::from(0_u8));
/// ```
pub fn mertens(low: &BigUint, high: &BigUint) -> Result<BigInt, BigUint> {
    fold(low, high, BigInt::zero(), |n, sum| {
        *sum += BigInt::from(divisors::moebius(n)?);
        Ok(())
    })
}

/// Return the count or sum over the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `PP.pm::sumliouville`, using direct iteration.
/// Keep ranges short. Prime tests follow the BPSW contract of `bigint::primality`.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::sumliouville;
/// use num_bigint::BigInt;
/// assert_eq!(sumliouville(&1_u8.into(), &0_u8.into()).expect("empty"), BigInt::from(0_u8));
/// ```
pub fn sumliouville(low: &BigUint, high: &BigUint) -> Result<BigInt, BigUint> {
    fold(low, high, BigInt::zero(), |n, sum| {
        *sum += if n.is_zero() {
            BigInt::zero()
        } else {
            BigInt::from(if divisors::prime_bigomega(n)? % 2 == 0 {
                1
            } else {
                -1
            })
        };
        Ok(())
    })
}

/// Return the family count or sum in the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `almost_primes.c::almost_prime_count_range`, using direct iteration.
/// Keep ranges short.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::almost_prime_count;
/// use num_bigint::BigUint;
/// assert_eq!(almost_prime_count(2, &1_u8.into(), &0_u8.into()).expect("empty"), BigUint::from(0_u8));
/// ```
pub fn almost_prime_count(k: u64, low: &BigUint, high: &BigUint) -> Result<BigUint, BigUint> {
    fold(low, high, BigUint::zero(), |n, sum| {
        *sum += BigUint::from(u8::from(!n.is_zero() && divisors::prime_bigomega(n)? == k));
        Ok(())
    })
}

/// Return the family count or sum in the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `omega_primes.c::omega_prime_count`, using direct iteration.
/// Keep ranges short.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::omega_prime_count;
/// use num_bigint::BigUint;
/// assert_eq!(omega_prime_count(2, &1_u8.into(), &0_u8.into()).expect("empty"), BigUint::from(0_u8));
/// ```
pub fn omega_prime_count(k: u64, low: &BigUint, high: &BigUint) -> Result<BigUint, BigUint> {
    fold(low, high, BigUint::zero(), |n, sum| {
        *sum += BigUint::from(u8::from(!n.is_zero() && divisors::prime_omega(n)? == k));
        Ok(())
    })
}

/// Return the family count or sum in the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `powerful.c::powerful_count`, using direct iteration.
/// Keep ranges short.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::powerful_count;
/// use num_bigint::BigUint;
/// assert_eq!(powerful_count(2, &1_u8.into(), &0_u8.into()).expect("empty"), BigUint::from(0_u8));
/// ```
pub fn powerful_count(k: u32, low: &BigUint, high: &BigUint) -> Result<BigUint, BigUint> {
    fold(low, high, BigUint::zero(), |n, sum| {
        *sum += BigUint::from(u8::from(
            !n.is_zero() && super::factor::factor_exp(n)?.iter().all(|(_, e)| *e >= k),
        ));
        Ok(())
    })
}

/// Return the family count or sum in the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `powerful.c::sumpowerful`, using direct iteration.
/// Keep ranges short.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::sumpowerful;
/// use num_bigint::BigUint;
/// assert_eq!(sumpowerful(2, &1_u8.into(), &0_u8.into()).expect("empty"), BigUint::from(0_u8));
/// ```
pub fn sumpowerful(k: u32, low: &BigUint, high: &BigUint) -> Result<BigUint, BigUint> {
    fold(low, high, BigUint::zero(), |n, sum| {
        *sum += if !n.is_zero() && super::factor::factor_exp(n)?.iter().all(|(_, e)| *e >= k) {
            n.clone()
        } else {
            BigUint::zero()
        };
        Ok(())
    })
}

/// Sum the kth-powerfree parts of positive integers at most `n`.
///
/// Port of MPU 0.75 `powerfree.c::powerfree_part_sum` and `fprod`.
/// Cost grows with the kth root. Use large `k` for large inputs.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::powerfree_part_sum;
/// use num_bigint::BigUint;
/// assert_eq!(powerfree_part_sum(&10_u8.into(), 2).expect("factors"), BigUint::from(38_u8));
/// ```
pub fn powerfree_part_sum(n: &BigUint, k: u32) -> Result<BigUint, BigUint> {
    if k < 2 {
        return Ok(BigUint::from(u8::from(!n.is_zero())));
    }
    let mut sum = BigInt::from(n * (n + 1_u8) / 2_u8);
    let top = rootint(n, k);
    let mut j = BigUint::from(2_u8);
    while j <= top {
        let mut product = BigInt::one();
        for (p, _) in super::factor::factor_exp(&j)? {
            product *= BigInt::one() - BigInt::from(p.pow(k));
        }
        let q = n / j.pow(k);
        sum += product * BigInt::from(&q * (&q + 1_u8) / 2_u8);
        j += 1_u8;
    }
    Ok(sum.magnitude().clone())
}
/// Count the twin prime pairs `(p, p + 2)` with `low <= p <= high`.
///
/// Port of MPU 0.75 `PP.pm::twin_prime_count`: the lesser prime must lie in
/// the range, so `twin_prime_count(0, n)` equals the native
/// `twin_prime_count(n)`. Uses direct iteration, so keep ranges short.
/// Prime tests follow the BPSW contract of `bigint::primality`.
///
/// ```
/// use math_prime_util::bigint::family_counts::twin_prime_count;
/// use num_bigint::BigUint;
/// assert_eq!(twin_prime_count(&0_u8.into(), &10_u8.into()), BigUint::from(2_u8));
/// ```
#[must_use]
pub fn twin_prime_count(low: &BigUint, high: &BigUint) -> BigUint {
    let mut count = BigUint::zero();
    let mut n = low.clone();
    while n <= *high {
        if super::primality::is_prime(&n) != 0 && super::primality::is_prime(&(&n + 2_u8)) != 0 {
            count += 1_u8;
        }
        n += 1_u8;
    }
    count
}
/// Return true if `n` is a power `p^e` of a prime `p` with `e >= 1`.
fn is_prime_power(n: &BigUint) -> bool {
    if *n < BigUint::from(2_u8) {
        return false;
    }
    // The largest exponent leaves a root that is not itself a power.
    let e = super::arith::is_power_exp(n).max(1);
    super::primality::is_prime(&rootint(n, e)) != 0
}
/// Count the prime powers `p^e` (`e >= 1`) in the inclusive range `low..=high`.
///
/// Port of MPU 0.75 `PP.pm::prime_power_count`; one is not a prime power, so
/// `prime_power_count(0, n)` equals the native `prime_power_count(n)`.
/// Uses direct iteration, so keep ranges short.
///
/// ```
/// use math_prime_util::bigint::family_counts::prime_power_count;
/// use num_bigint::BigUint;
/// assert_eq!(prime_power_count(&1_u8.into(), &16_u8.into()), BigUint::from(10_u8));
/// ```
#[must_use]
pub fn prime_power_count(low: &BigUint, high: &BigUint) -> BigUint {
    let mut count = BigUint::zero();
    let mut n = low.clone();
    while n <= *high {
        if is_prime_power(&n) {
            count += 1_u8;
        }
        n += 1_u8;
    }
    count
}
/// Return the `n`th perfect power (1, 4, 8, 9, 16, ...), or `None` for `n = 0`.
///
/// Port of MPU 0.75 `perfect_powers.c::nth_perfect_power`, found by bisection
/// on [`perfect_power_count`], which takes one integer root per exponent.
///
/// ```
/// use math_prime_util::bigint::family_counts::nth_perfect_power;
/// use num_bigint::BigUint;
/// assert_eq!(nth_perfect_power(&6_u8.into()), Some(BigUint::from(25_u8)));
/// assert_eq!(nth_perfect_power(&0_u8.into()), None);
/// ```
#[must_use]
pub fn nth_perfect_power(n: &BigUint) -> Option<BigUint> {
    if n.is_zero() {
        return None;
    }
    // Rank n lies at or below n^2, which is the nth square.
    let mut lo = BigUint::one();
    let mut hi = n * n;
    while lo < hi {
        let mid: BigUint = (&lo + &hi) / 2_u8;
        if perfect_power_count(&mid) >= *n {
            hi = mid;
        } else {
            lo = mid + 1_u8;
        }
    }
    Some(lo)
}
/// Return the `n`th `k`-powerfree integer, or `None` for `n = 0` or `k < 2`.
///
/// Port of MPU 0.75 `powerfree.c::nth_powerfree`, found by bisection on
/// [`powerfree_count`], whose cost grows with the kth root. Use large `k`
/// for large inputs.
///
/// # Errors
/// Return an unresolved cofactor if factorization fails.
///
/// ```
/// use math_prime_util::bigint::family_counts::nth_powerfree;
/// use num_bigint::BigUint;
/// assert_eq!(nth_powerfree(&7_u8.into(), 2).expect("factors"), Some(BigUint::from(10_u8)));
/// ```
pub fn nth_powerfree(n: &BigUint, k: u32) -> Result<Option<BigUint>, BigUint> {
    if n.is_zero() || k < 2 {
        return Ok(None);
    }
    // Every integer below 2^k is kth-powerfree.
    if u64::from(k) >= n.bits() || *n < BigUint::from(4_u8) {
        return Ok(Some(n.clone()));
    }
    let mut lo = n.clone();
    let mut hi = n * 2_u8;
    while powerfree_count(&hi, k)? < *n {
        lo = hi.clone();
        hi *= 2_u8;
    }
    while lo < hi {
        let mid: BigUint = (&lo + &hi) / 2_u8;
        if powerfree_count(&mid, k)? >= *n {
            hi = mid;
        } else {
            lo = mid + 1_u8;
        }
    }
    Ok(Some(lo))
}
