/// Rank of `perm` in lexicographic order, or `None` on `u64` overflow.
///
/// Port of `perm_to_num` in `util.c` (MPU 0.75). The method is the factorial
/// number system (Lehmer code). `perm` must list each value in `0..perm.len()`
/// once. An empty permutation has rank 0. The native factorial fits only
/// through `20!`, so a length above 21 returns `None`, as in the C function.
///
/// # Panics
///
/// Panics if `perm` is not a permutation of `0..perm.len()`. MPU dies on an
/// invalid permutation array.
///
/// ```
/// assert_eq!(math_prime_util::permtonum(&[2, 1, 0]), Some(5));
/// assert_eq!(math_prime_util::permtonum(&[]), Some(0));
/// ```
pub fn permtonum(perm: &[u64]) -> Option<u64> {
    let n = perm.len();
    if n == 0 {
        return Some(0);
    }
    let mut seen = vec![false; n];
    for &value in perm {
        let Some(index) = usize::try_from(value).ok().filter(|&index| index < n) else {
            panic!("permtonum: invalid permutation array");
        };
        if seen[index] {
            panic!("permtonum: invalid permutation array");
        }
        seen[index] = true;
    }
    // `factorial(n - 1)` in util.c. On a 64-bit word it is 0 for n - 1 > 20.
    let mut fact = 1_u64;
    for factor in 2..n {
        fact = fact.checked_mul(u64::try_from(factor).expect("length fits in u64"))?;
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
        let divisor = u64::try_from(n - left - 1).expect("divisor fits in u64");
        fact /= divisor;
    }
    Some(rank)
}
