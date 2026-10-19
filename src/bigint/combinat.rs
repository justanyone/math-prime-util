//! Exact combinatorial products and permutation ranks.

use num_bigint::BigUint;

use crate::PermutationError;

/// Returns the product of `m` descending integers from `n`.
///
/// Port of MPU 0.75 `util.c::falling_factorial`, without word overflow.
/// The empty product is one. If `m > n`, the result is zero.
///
/// ```
/// use math_prime_util::bigint::combinat::falling_factorial;
/// use num_bigint::BigUint;
/// assert_eq!(falling_factorial(&5_u8.into(), &3_u8.into()), BigUint::from(60_u8));
/// ```
#[must_use]
pub fn falling_factorial(n: &BigUint, m: &BigUint) -> BigUint {
    if m == &BigUint::ZERO {
        return BigUint::from(1_u8);
    }
    if m > n {
        return BigUint::ZERO;
    }
    let mut result = n.clone();
    let mut i = BigUint::from(1_u8);
    while &i < m {
        result *= n - &i;
        i += 1_u8;
    }
    result
}

/// Returns the product of `m` ascending integers from `n`.
///
/// Port of MPU 0.75 `util.c::rising_factorial`, without word overflow.
///
/// ```
/// use math_prime_util::bigint::combinat::rising_factorial;
/// use num_bigint::BigUint;
/// assert_eq!(rising_factorial(&5_u8.into(), &3_u8.into()), BigUint::from(210_u16));
/// ```
#[must_use]
pub fn rising_factorial(n: &BigUint, m: &BigUint) -> BigUint {
    if m == &BigUint::ZERO {
        return BigUint::from(1_u8);
    }
    falling_factorial(&(n + m - 1_u8), m)
}

/// Returns `n * (n-k) * (n-2k) * ...`.
///
/// Port of MPU 0.75 `util.c::multifactorial`, without word overflow.
///
/// # Panics
/// Panics if `k` is zero, as the native function does.
///
/// ```
/// use math_prime_util::bigint::combinat::multifactorial;
/// use num_bigint::BigUint;
/// assert_eq!(multifactorial(&9_u8.into(), &2_u8.into()), BigUint::from(945_u16));
/// ```
#[must_use]
pub fn multifactorial(n: &BigUint, k: &BigUint) -> BigUint {
    assert!(*k != BigUint::ZERO, "multifactorial: k must be positive");
    let mut result = BigUint::from(1_u8);
    let mut i = n.clone();
    while i != BigUint::ZERO {
        result *= &i;
        if &i <= k {
            break;
        }
        i -= k;
    }
    result
}

/// Returns the permutation of `0..n` at zero-based lexicographic `rank`.
///
/// Port of MPU 0.75 `util.c::num_to_perm`. The rank wraps modulo `n!`.
/// The factorial digits follow `PP.pm::numtoperm` after native overflow.
///
/// ```
/// use math_prime_util::bigint::combinat::numtoperm;
/// assert_eq!(numtoperm(5, &15_u8.into()), [0, 3, 2, 4, 1]);
/// ```
#[must_use]
pub fn numtoperm(n: usize, rank: &BigUint) -> Vec<usize> {
    let mut result: Vec<_> = (0..n).collect();
    if n < 2 {
        return result;
    }
    let mut f = (2..n).fold(BigUint::from(1_u8), |a, b| a * b);
    let mut rank = rank % (&f * n);
    for i in 0..n - 1 {
        // rank < (n-i)!, so this digit is a valid slice offset.
        let digit = &rank / &f;
        let offset = usize::try_from(&digit).unwrap_or(0);
        rank %= &f;
        f /= n - i - 1;
        result[i..=i + offset].rotate_right(1);
    }
    result
}

/// Returns the zero-based lexicographic rank of a permutation.
///
/// Port of MPU 0.75 `util.c::perm_to_num`, with `PP.pm::permtonum`
/// validation and exact factorial arithmetic.
///
/// # Errors
/// Returns [`PermutationError::Invalid`] for duplicate or out-of-range entries.
///
/// ```
/// use math_prime_util::bigint::combinat::permtonum;
/// assert_eq!(permtonum(&[2_u8.into(), 1_u8.into(), 0_u8.into()]), Ok(5_u8.into()));
/// ```
pub fn permtonum(perm: &[BigUint]) -> Result<BigUint, PermutationError> {
    let n = perm.len();
    let mut seen = vec![false; n];
    for value in perm {
        let index = usize::try_from(value).map_err(|_| PermutationError::Invalid)?;
        if index >= n || seen[index] {
            return Err(PermutationError::Invalid);
        }
        seen[index] = true;
    }
    let mut rank = BigUint::ZERO;
    let mut f = (2..n).fold(BigUint::from(1_u8), |a, b| a * b);
    for i in 0..n.saturating_sub(1) {
        let smaller = perm[i + 1..].iter().filter(|v| *v < &perm[i]).count();
        rank += &f * smaller;
        f /= n - i - 1;
    }
    Ok(rank)
}
