/// Error from [`permtonum`] when its input is not a permutation.
///
/// Port of the `croak` in `perm_to_num` in `util.c` (MPU 0.75).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermutationError {
    /// A value repeats or is `>= perm.len()`.
    Invalid,
}

impl std::fmt::Display for PermutationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("permtonum: invalid permutation array")
    }
}

impl std::error::Error for PermutationError {}

/// Rank of `perm` in lexicographic order, `Ok(None)` on `u64` overflow.
///
/// Port of `perm_to_num` in `util.c` (MPU 0.75). The method is the factorial
/// number system (Lehmer code). `perm` must list each value in `0..perm.len()`
/// once. An empty permutation has rank 0. The native factorial fits only
/// through `20!`, so a length above 21 returns `Ok(None)`, as in the C function.
///
/// # Errors
///
/// Returns [`PermutationError::Invalid`] if `perm` is not a permutation of
/// `0..perm.len()`. MPU dies on an invalid permutation array.
///
/// ```
/// use math_prime_util::PermutationError;
/// assert_eq!(math_prime_util::permtonum(&[2, 1, 0]), Ok(Some(5)));
/// assert_eq!(math_prime_util::permtonum(&[]), Ok(Some(0)));
/// assert_eq!(math_prime_util::permtonum(&[1, 1]), Err(PermutationError::Invalid));
/// ```
pub fn permtonum(perm: &[u64]) -> Result<Option<u64>, PermutationError> {
    let n = perm.len();
    let mut seen = vec![false; n];
    for &value in perm {
        let index = usize::try_from(value).unwrap_or(n);
        if index >= n || seen[index] {
            return Err(PermutationError::Invalid);
        }
        seen[index] = true;
    }
    Ok(rank_checked(perm))
}

/// Lehmer-code rank of a validated permutation, `None` on `u64` overflow.
fn rank_checked(perm: &[u64]) -> Option<u64> {
    let n = perm.len();
    if n == 0 {
        return Some(0);
    }
    // `factorial(n - 1)` in util.c. On a 64-bit word it is 0 for n - 1 > 20.
    let mut fact = 1_u64;
    for factor in 2..n {
        let factor = u64::try_from(factor).ok()?;
        fact = fact.checked_mul(factor)?;
    }
    let mut rank = 0_u64;
    for left in 0..n - 1 {
        let mut smaller = 0_u64;
        for right in left + 1..n {
            if perm[right] < perm[left] {
                smaller += 1;
            }
        }
        // util.c: if (k > (UV_MAX - num) / f) return 0.
        let term = smaller.checked_mul(fact)?;
        rank = rank.checked_add(term)?;
        let divisor = u64::try_from(n - left - 1).ok()?;
        fact /= divisor;
    }
    Some(rank)
}
