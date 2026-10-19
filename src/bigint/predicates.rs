//! Integer predicates on arbitrary precision unsigned values.
//!
//! Factor-based tests return an unsplit cofactor as an error.
use super::{arith, divisors, factor, primality};
use num_bigint::BigUint;

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

/// Test for k prime factors.
///
/// Port of MPU 0.75 `util.c::is_almost_prime`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_almost_prime;
/// use num_bigint::BigUint;
/// assert_eq!(is_almost_prime(&BigUint::from(3_u8), &BigUint::from(12_u8)), Ok(true));
/// ```
pub fn is_almost_prime(k: &BigUint, n: &BigUint) -> Result<bool, BigUint> {
    if *n == big(0) {
        return Ok(false);
    }
    Ok(BigUint::from(divisors::prime_bigomega(n)?) == *k)
}

/// Test for k distinct prime factors.
///
/// Port of MPU 0.75 `omega_primes.c::is_omega_prime`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_omega_prime;
/// use num_bigint::BigUint;
/// assert_eq!(is_omega_prime(&BigUint::from(2_u8), &BigUint::from(12_u8)), Ok(true));
/// ```
pub fn is_omega_prime(k: &BigUint, n: &BigUint) -> Result<bool, BigUint> {
    if *n == big(0) {
        return Ok(false);
    }
    Ok(BigUint::from(divisors::prime_omega(n)?) == *k)
}

/// Test for exactly two prime factors, with multiplicity.
///
/// Port of MPU 0.75 `semi_primes.c::is_semiprime`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_semiprime;
/// use num_bigint::BigUint;
/// assert_eq!(is_semiprime(&BigUint::from(6_u8)), Ok(true));
/// ```
pub fn is_semiprime(n: &BigUint) -> Result<bool, BigUint> {
    is_almost_prime(&big(2), n)
}

/// Test whether every group of order `n` is cyclic.
///
/// Port of MPU 0.75 `util.c::is_cyclic`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_cyclic;
/// use num_bigint::BigUint;
/// assert_eq!(is_cyclic(&BigUint::from(35_u8)), Ok(true));
/// ```
pub fn is_cyclic(n: &BigUint) -> Result<bool, BigUint> {
    if *n == big(0) {
        return Ok(false);
    }
    Ok(arith::gcd(n, &divisors::euler_phi(n)?) == big(1))
}

/// Test whether `n` is a positive fundamental discriminant.
///
/// Port of MPU 0.75 `util.c::is_fundamental`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_fundamental;
/// use num_bigint::BigUint;
/// assert_eq!(is_fundamental(&BigUint::from(5_u8)), Ok(true));
/// ```
pub fn is_fundamental(n: &BigUint) -> Result<bool, BigUint> {
    let r = n % 16_u8;
    if r != big(0) && &r % 4_u8 == big(0) && r != big(4) {
        divisors::is_square_free(&(n >> 2_usize))
    } else if &r % 4_u8 == big(1) {
        divisors::is_square_free(n)
    } else {
        Ok(false)
    }
}

/// Test whether the proper divisors sum to `n`.
///
/// Port of MPU 0.75 `util.c::is_perfect_number and factor.c::divisor_sum`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_perfect_number;
/// use num_bigint::BigUint;
/// assert_eq!(is_perfect_number(&BigUint::from(28_u8)), Ok(true));
/// ```
pub fn is_perfect_number(n: &BigUint) -> Result<bool, BigUint> {
    if *n < big(2) {
        return Ok(false);
    }
    if n.bit(0) {
        return Ok(divisors::divisor_sum(n, 1)? == n * 2_u8);
    }
    let v = n.trailing_zeros().unwrap_or(0);
    let m = n >> v;
    if (&m >> v) != big(1) || (&m & (&m + 1_u8)) != big(0) {
        return Ok(false);
    }
    Ok(primality::is_prob_prime(&m) != 0)
}

/// Test for a perfect power. Zero and one are included.
///
/// Port of MPU 0.75 `perfect_powers.c::is_perfect_power`.
///
/// ```
/// use math_prime_util::bigint::predicates::is_perfect_power;
/// use num_bigint::BigUint;
/// assert_eq!(is_perfect_power(&BigUint::from(64_u8)), true);
/// ```
pub fn is_perfect_power(n: &BigUint) -> bool {
    *n <= big(1) || arith::is_power_exp(n) > 1
}

/// Return the prime-power exponent, or zero.
///
/// Port of MPU 0.75 `prime_powers.c::prime_power`.
///
/// ```
/// use math_prime_util::bigint::predicates::is_prime_power;
/// use num_bigint::BigUint;
/// assert_eq!(is_prime_power(&BigUint::from(8_u8)), 3);
/// ```
pub fn is_prime_power(n: &BigUint) -> u32 {
    if *n < big(2) {
        return 0;
    }
    if primality::is_prob_prime(n) != 0 {
        return 1;
    }
    let e = arith::is_power_exp(n);
    if e > 1 && primality::is_prob_prime(&arith::rootint(n, e)) != 0 {
        e
    } else {
        0
    }
}

/// Test the integer predicate.
///
/// Port of MPU 0.75 `powerfree.c::is_powerfree`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_powerfree;
/// use num_bigint::BigUint;
/// assert_eq!(is_powerfree(&BigUint::from(12_u8), &BigUint::from(3_u8)), Ok(true));
/// ```
pub fn is_powerfree(n: &BigUint, k: &BigUint) -> Result<bool, BigUint> {
    if *k < big(2) || *n <= big(1) {
        return Ok(*n == big(1));
    }
    Ok(factor::factor_exp(n)?
        .iter()
        .all(|(_, e)| BigUint::from(*e) < *k))
}

/// Test the integer predicate.
///
/// Port of MPU 0.75 `powerful.c::is_powerful`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_powerful;
/// use num_bigint::BigUint;
/// assert_eq!(is_powerful(&BigUint::from(72_u8), &BigUint::from(2_u8)), Ok(true));
/// ```
pub fn is_powerful(n: &BigUint, k: &BigUint) -> Result<bool, BigUint> {
    if *n <= big(1) {
        return Ok(*n == big(1));
    }
    if *k <= big(1) {
        return Ok(true);
    }
    Ok(factor::factor_exp(n)?
        .iter()
        .all(|(_, e)| BigUint::from(*e) >= *k))
}

/// Test whether `n` is a practical number.
///
/// Port of MPU 0.75 `util.c::is_practical and _divsum1`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_practical;
/// use num_bigint::BigUint;
/// assert_eq!(is_practical(&BigUint::from(12_u8)), Ok(true));
/// ```
pub fn is_practical(n: &BigUint) -> Result<bool, BigUint> {
    if *n == big(0) || n.bit(0) {
        return Ok(*n == big(1));
    }
    let mut product = big(1);
    for (p, e) in factor::factor_exp(n)? {
        if p > &product + 1_u8 {
            return Ok(false);
        }
        product *= (p.pow(e + 1) - 1_u8) / (&p - 1_u8);
    }
    Ok(true)
}

/// Test whether every prime factor is at least `k`.
///
/// Port of MPU 0.75 `util.c::is_rough`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_rough;
/// use num_bigint::BigUint;
/// assert_eq!(is_rough(&BigUint::from(77_u8), &BigUint::from(7_u8)), Ok(true));
/// ```
pub fn is_rough(n: &BigUint, k: &BigUint) -> Result<bool, BigUint> {
    if *n == big(0) {
        return Ok(*k == big(0));
    }
    if *n == big(1) || *k <= big(2) {
        return Ok(true);
    }
    Ok(factor::factor_exp(n)?.iter().all(|(p, _)| p >= k))
}

/// Test the integer predicate.
///
/// Port of MPU 0.75 `util.c::is_smooth`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_smooth;
/// use num_bigint::BigUint;
/// assert_eq!(is_smooth(&BigUint::from(72_u8), &BigUint::from(3_u8)), Ok(true));
/// ```
pub fn is_smooth(n: &BigUint, k: &BigUint) -> Result<bool, BigUint> {
    if *n <= big(1) {
        return Ok(true);
    }
    if *k <= big(1) {
        return Ok(false);
    }
    Ok(factor::factor_exp(n)?.iter().all(|(p, _)| p <= k))
}

/// Test for a sum of k squares.
///
/// Port of MPU 0.75 `util.c::is_sum_of_two_squares`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_sum_of_squares;
/// use num_bigint::BigUint;
/// assert_eq!(is_sum_of_squares(&BigUint::from(5_u8), &BigUint::from(2_u8)), Ok(true));
/// ```
pub fn is_sum_of_squares(n: &BigUint, k: &BigUint) -> Result<bool, BigUint> {
    if *k == big(0) {
        return Ok(*n == big(0));
    }
    if *k == big(1) {
        return Ok(arith::is_square(n));
    }
    if *k > big(3) || *n < big(3) {
        return Ok(true);
    }
    if *k == big(3) {
        let v = n.trailing_zeros().unwrap_or(0);
        return Ok(v % 2 == 1 || (n >> v) % 8_u8 != big(7));
    }
    Ok(factor::factor_exp(n)?
        .iter()
        .all(|(p, e)| e % 2 == 0 || p % 4_u8 != big(3)))
}

/// Test for a totient.
///
/// Port of MPU 0.75 `totients.c::is_totient`.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_totient;
/// use num_bigint::BigUint;
/// assert_eq!(is_totient(&BigUint::from(12_u8)), Ok(true));
/// ```
pub fn is_totient(n: &BigUint) -> Result<bool, BigUint> {
    if *n == big(0) || n.bit(0) {
        return Ok(*n == big(1));
    }
    totpred(n, n)
}

// MPU 0.75 totients.c::_totpred. Each recursion reduces n and the bound.
fn totpred(n: &BigUint, maxd: &BigUint) -> Result<bool, BigUint> {
    if *maxd <= big(1) || n.bit(0) {
        return Ok(false);
    }
    if (n & (n - 1_u8)) == big(0) {
        return Ok(true);
    }
    let half = n >> 1_usize;
    if half < *maxd && primality::is_prob_prime(&(&half * 2_u8 + 1_u8)) != 0 {
        return Ok(true);
    }
    for d in divisors::divisors(&half, None)? {
        if d >= *maxd {
            break;
        }
        let p = &d * 2_u8 + 1_u8;
        if primality::is_prob_prime(&p) == 0 {
            continue;
        }
        let mut r = &half / &d;
        loop {
            if r == p || totpred(&r, &d)? {
                return Ok(true);
            }
            if &r % &p != big(0) {
                break;
            }
            r /= &p;
        }
    }
    Ok(false)
}
