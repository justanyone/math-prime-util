//! Integer predicates on arbitrary precision unsigned values.
//!
//! Factor-based tests return an unsplit cofactor as an error.
use super::{arith, divisors, factor, modular, primality};
use num_bigint::{BigInt, BigUint};

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
/// Port of MPU 0.75 `util.c::is_semiprime`.
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

/// Test whether all prime exponents are less than `k`.
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

/// Test whether all prime exponents are at least `k`.
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

/// Test whether all prime factors are at most `k`. Zero is smooth.
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
    if n.bit(0) {
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

/// Test whether `n` is a congruent number: the area of a right triangle with
/// rational sides.
///
/// Port of MPU 0.75 `PP.pm::is_congruent_number`, the path MPU takes past
/// 2^64. The square part is removed first, and `n = 5, 6, 7 (mod 8)` is
/// congruent. Primes, twice a prime, and some products of two primes (or
/// twice that) are then decided by the Lagrange, Bastien and Monsky filters.
/// Every other `n` goes to Tunnell's count, which takes time linear in `n`:
/// this is practical to about 2^40 only, as in MPU. Tunnell's test and the
/// `5, 6, 7 (mod 8)` rule assume the weak Birch and Swinnerton-Dyer
/// conjecture.
///
/// # Errors
/// Returns the cofactor that could not be split.
///
/// ```
/// use math_prime_util::bigint::predicates::is_congruent_number;
/// use num_bigint::BigUint;
/// assert_eq!(is_congruent_number(&BigUint::from(6_u8)), Ok(true));
/// assert_eq!(is_congruent_number(&BigUint::from(3_u8)), Ok(false));
/// // A prime p = 3 (mod 8) is not congruent; 2p is.
/// let p: BigUint = "18446744073709551667".parse().unwrap();
/// assert_eq!(is_congruent_number(&p), Ok(false));
/// assert_eq!(is_congruent_number(&(p * 2_u8)), Ok(true));
/// ```
pub fn is_congruent_number(n: &BigUint) -> Result<bool, BigUint> {
    if *n < big(13) {
        return Ok(*n >= big(5) && *n <= big(7));
    }
    let n8 = n % 8_u8;
    if n8 >= big(5) {
        return Ok(true);
    }
    if !divisors::is_square_free(n)? {
        let core: BigUint = factor::factor_exp(n)?
            .into_iter()
            .filter(|(_, e)| e % 2 == 1)
            .map(|(p, _)| p)
            .product();
        return is_congruent_number(&core);
    }
    let half = n >> 1_usize;
    let kr = |a: &BigUint, b: &BigUint| {
        arith::kronecker(&BigInt::from(a.clone()), &BigInt::from(b.clone()))
    };
    if !n.bit(0) && primality::is_prime(&half) != 0 {
        let p8 = &half % 8_u8;
        if p8 == big(3) || p8 == big(7) {
            return Ok(true);
        }
        if p8 == big(5) || &half % 16_u8 == big(9) {
            return Ok(false);
        }
    } else if primality::is_prime(n) != 0 {
        if n8 == big(3) {
            return Ok(false);
        }
        if let Some(r) = modular::sqrtmod(&big(2), n) {
            if kr(&(r + 1_u8), n) == -1 {
                return Ok(false);
            }
        }
    } else {
        let f = factor::factor(n)?;
        let (pair, q8) = if f.len() == 2 {
            ((&f[0], &f[1]), 3)
        } else if f.len() == 3 && f[0] == big(2) {
            ((&f[1], &f[2]), 5)
        } else {
            return Ok(tunnell(n));
        };
        // Two odd primes p < q (n = pq or n = 2pq); `q8` is 3 or 5 (mod 8).
        let (p, q) = pair;
        let (pm, qm) = (p % 8_u8, q % 8_u8);
        let (one, odd) = (big(1), big(q8));
        if (pm == odd && qm == odd)
            || (pm == one && qm == odd && kr(p, q) == -1)
            || (pm == odd && qm == one && kr(q, p) == -1)
        {
            return Ok(false);
        }
    }
    Ok(tunnell(n))
}

// Tunnell's count of MPU 0.75 `PP.pm::is_congruent_number` (general test).
// Inputs that fit `u64` use the native loop; larger ones take impractical time.
fn tunnell(n: &BigUint) -> bool {
    if let Ok(small) = u64::try_from(n) {
        return crate::combinat::is_congruent_number::tunnell(small);
    }
    let odd = n.bit(0);
    let target = if odd { n.clone() } else { n >> 1_usize };
    let mut sols = [big(0), big(0)];
    let mut z = big(0);
    let limz = arith::sqrtint(&(&target >> 3_usize));
    while z <= limz {
        let rem = &target - &z * &z * 8_u8;
        let weight = |y_pos: bool| big(1) << (1 + usize::from(y_pos) + usize::from(z != big(0)));
        let mut zsols = big(0);
        if odd {
            let mut y = big(0);
            let limy = arith::sqrtint(&(&rem >> 1_usize));
            while y <= limy {
                if arith::is_square(&(&rem - &y * &y * 2_u8)) {
                    zsols += weight(y != big(0));
                }
                y += 1_u8;
            }
        } else {
            let mut x = big(1);
            let limx = arith::sqrtint(&rem);
            while x <= limx {
                let y = &rem - &x * &x;
                if arith::is_square(&y) {
                    zsols += weight(y != big(0));
                }
                x += 2_u8;
            }
        }
        sols[usize::from(z.bit(0))] += zsols;
        z += 1_u8;
    }
    sols[0] == sols[1]
}
