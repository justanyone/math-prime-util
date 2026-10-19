//! Big integer versions of MPU's factoring and number family iterators.
//!
//! These routines scan inclusive ranges and use the bigint factorization and
//! divisor code. Large ranges can take a long time.
use super::{divisors, factor};
use num_bigint::BigUint;

fn range(lo: &BigUint, hi: &BigUint, minimum: u64) -> impl Iterator<Item = BigUint> {
    let start = lo.clone().max(BigUint::from(minimum));
    let end = hi.clone();
    std::iter::successors((start <= end).then_some(start), |n| Some(n + 1_u8))
        .take_while(move |n| n <= &end)
}

/// Return divisors of `n` in ascending order.
///
/// Port of MPU 0.75 `XS.xs::fordivisors`, using `PP.pm::divisors` for BigInt.
/// Zero has no divisors.
/// ```
/// use math_prime_util::bigint::iter::fordivisors;
/// use num_bigint::BigUint;
/// assert_eq!(fordivisors(&BigUint::from(6_u8)).unwrap(), [1_u8, 2, 3, 6].map(BigUint::from));
/// ```
pub fn fordivisors(n: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    divisors::divisors(n, None)
}

/// Return values in an inclusive range with exactly `k` prime factors, counted with multiplicity.
///
/// Port of MPU 0.75 `XS.xs::foralmostprimes` and `PP.pm::foralmostprimes`.
/// ```
/// use math_prime_util::bigint::iter::foralmostprimes;
/// use num_bigint::BigUint;
/// assert_eq!(foralmostprimes(2, &BigUint::from(1_u8), &BigUint::from(10_u8)).unwrap(), [4_u8, 6, 9, 10].map(BigUint::from));
/// ```
pub fn foralmostprimes(k: u64, lo: &BigUint, hi: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    if k == 0 {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for n in range(lo, hi, 1) {
        let factors = factor::factor(&n)?;
        if factors.len() as u64 == k {
            out.push(n);
        }
    }
    Ok(out)
}

/// Return semiprimes in an inclusive range.
///
/// Port of MPU 0.75 `XS.xs::forsemiprimes`, using `PP.pm::is_semiprime`.
/// ```
/// use math_prime_util::bigint::iter::forsemiprimes;
/// use num_bigint::BigUint;
/// assert_eq!(forsemiprimes(&BigUint::from(1_u8), &BigUint::from(10_u8)).unwrap(), [4_u8, 6, 9, 10].map(BigUint::from));
/// ```
pub fn forsemiprimes(lo: &BigUint, hi: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    foralmostprimes(2, lo, hi)
}

/// Return values with no repeated prime factor in an inclusive range.
///
/// Port of MPU 0.75 `XS.xs::forfactored` (alias `forsquarefree`) using
/// `PP.pm::is_square_free`.
/// ```
/// use math_prime_util::bigint::iter::forsquarefree;
/// use num_bigint::BigUint;
/// assert_eq!(forsquarefree(&BigUint::from(1_u8), &BigUint::from(6_u8)).unwrap(), [1_u8, 2, 3, 5, 6].map(BigUint::from));
/// ```
pub fn forsquarefree(lo: &BigUint, hi: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    let mut out = Vec::new();
    for n in range(lo, hi, 1) {
        if divisors::is_square_free(&n)? {
            out.push(n);
        }
    }
    Ok(out)
}

/// Return factorizations for values in an inclusive range.
///
/// Port of MPU 0.75 `XS.xs::forfactored`, using `PP.pm::factor`.
/// ```
/// use math_prime_util::bigint::iter::forfactored;
/// use num_bigint::BigUint;
/// assert_eq!(forfactored(&BigUint::from(2_u8), &BigUint::from(4_u8)).unwrap(), vec![vec![2_u8.into()], vec![3_u8.into()], vec![2_u8.into(), 2_u8.into()]]);
/// ```
pub fn forfactored(lo: &BigUint, hi: &BigUint) -> Result<Vec<Vec<BigUint>>, BigUint> {
    let mut out = Vec::new();
    for n in range(lo, hi, 2) {
        out.push(factor::factor(&n)?);
    }
    Ok(out)
}

/// Return square-free integers in an inclusive range.
///
/// Port of MPU 0.75 `XS.xs::forsquarefreeint`, using `PP.pm::is_square_free`.
/// ```
/// use math_prime_util::bigint::iter::forsquarefreeint;
/// use num_bigint::BigUint;
/// assert_eq!(forsquarefreeint(&BigUint::from(1_u8), &BigUint::from(6_u8)).unwrap(), [1_u8, 2, 3, 5, 6].map(BigUint::from));
/// ```
pub fn forsquarefreeint(lo: &BigUint, hi: &BigUint) -> Result<Vec<BigUint>, BigUint> {
    forsquarefree(lo, hi)
}
