use crate::{factor_exp, is_square_free};

/// Test if no prime exponent reaches `k`.
/// Port of `powerfree.c` / `is_powerfree`.
/// ```
/// assert!(math_prime_util::is_powerfree(12, 3));
/// ```
pub fn is_powerfree(n: u64, k: u32) -> bool {
    if k < 2 || n <= 1 {
        return n == 1;
    }
    if k >= 64 || n < (1_u64 << (k - 1)) {
        return true;
    }
    if n.trailing_zeros() >= k {
        return false;
    }
    if k == 2 {
        return is_square_free(n);
    }
    if k == 3 {
        if [27, 125, 343, 1331, 2197].iter().any(|p| n % p == 0) {
            return false;
        }
        if n < 4913 {
            return true;
        }
    }
    factor_exp(n).iter().all(|&(_, e)| e < k)
}
