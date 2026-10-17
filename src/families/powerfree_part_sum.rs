//! Sum of powerfree parts.
//! Port of MPU 0.75 `powerfree.c`, `powerfree_part_sum` and `fprod`.

use super::powerful::factor_exponents;
use crate::rootint;

/// Block length of the range factoring, as in `powerfree.c` `factor_range_init`.
const RANGE_BLOCK: u64 = 1 << 16;

/// Triangular number `T(n) = n*(n+1)/2`.
/// Port of `powerfree.c` `T`.
fn triangular(n: u64) -> u128 {
    u128::from(n) * (u128::from(n) + 1) / 2
}

/// Primes up to `limit`, by trial division.
fn small_primes(limit: u64) -> Vec<u64> {
    (2..=limit)
        .filter(|&p| (2..).take_while(|d| d * d <= p).all(|d| p % d != 0))
        .collect()
}

/// Visit `(j, fprod(j, k), distinct prime count)` for `j` from 2 to `nk`.
///
/// Port of MPU 0.75 `powerfree.c`, `powerfree_part_sum`. For `nk <= 100`
/// each `j` is factored on its own. Above that, the port uses a segmented
/// range factoring like `factor_range_init` and `factor_range_next`
/// in `powerfree.c`: blocks of `RANGE_BLOCK` values, with the primes up to
/// `sqrt(nk)` removed from each block. A value left over after that is a
/// single prime, because `j <= nk`.
fn for_each_fprod(nk: u64, k: u32, mut visit: impl FnMut(u64, u128, usize)) {
    if nk <= 100 {
        for j in 2..=nk {
            let factors = factor_exponents(j);
            let product: u128 = factors
                .iter()
                .map(|&(p, _)| u128::from(p.pow(k)) - 1)
                .product();
            visit(j, product, factors.len());
        }
        return;
    }
    let primes = small_primes(rootint(nk, 2));
    let mut lo = 2_u64;
    while lo <= nk {
        let hi = nk.min(lo + RANGE_BLOCK - 1);
        let len = usize::try_from(hi - lo + 1).expect("block fits usize");
        let mut rem: Vec<u64> = (lo..=hi).collect();
        let mut prod = vec![1_u128; len];
        let mut count = vec![0_usize; len];
        for &p in &primes {
            let mut m = lo.div_ceil(p) * p;
            while m <= hi {
                let idx = usize::try_from(m - lo).expect("index fits usize");
                prod[idx] *= u128::from(p.pow(k)) - 1;
                count[idx] += 1;
                while rem[idx] % p == 0 {
                    rem[idx] /= p;
                }
                m += p;
            }
        }
        for idx in 0..len {
            if rem[idx] > 1 {
                prod[idx] *= u128::from(rem[idx].pow(k)) - 1;
                count[idx] += 1;
            }
            visit(lo + idx as u64, prod[idx], count[idx]);
        }
        lo = hi + 1;
    }
}

/// Sum `powerfree_part(i, k)` for `i` from 1 to `n`.
///
/// Port of MPU 0.75 `powerfree.c`, `powerfree_part_sum` and `fprod`.
/// Each term is `T(n / j^k)` times the product of `(1 - p^k)` over the
/// distinct primes `p` that divide `j`. The sum uses `u128`.
/// For `k < 2` the result is 1 when `n >= 1`, and 0 otherwise.
///
/// # Examples
///
/// ```
/// assert_eq!(math_prime_util::powerfree_part_sum(10, 2), 38);
/// ```
#[must_use]
pub fn powerfree_part_sum(n: u64, k: u32) -> u128 {
    if k < 2 || n <= 1 {
        return u128::from(n >= 1);
    }
    let mut sum = triangular(n);
    let mut subtract = 0_u128;
    for_each_fprod(rootint(n, k), k, |j, product, count| {
        let term = product * triangular(n / j.pow(k));
        if count % 2 == 0 {
            sum += term;
        } else {
            subtract += term;
        }
    });
    sum - subtract
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Direct sum of powerfree parts: `i / q^k`, where `q^k` is the largest
    /// `k`-th power dividing `i`.
    fn brute(n: u64, k: u32) -> u128 {
        (1..=n)
            .map(|i| {
                let q = (1..=rootint(i, k))
                    .filter(|&j| i % j.pow(k) == 0)
                    .max()
                    .unwrap_or(1);
                u128::from(i / q.pow(k))
            })
            .sum()
    }

    #[test]
    fn range_branch_matches_brute_force_at_root_boundary() {
        // Root 100 uses the per-integer branch; root 101 switches to range factoring.
        assert_eq!(powerfree_part_sum(10_200, 2), brute(10_200, 2));
        assert_eq!(powerfree_part_sum(10_201, 2), brute(10_201, 2));
        assert_eq!(powerfree_part_sum(10_404, 2), brute(10_404, 2));
    }

    #[test]
    fn range_branch_matches_per_integer_products() {
        let mut seen = Vec::new();
        for_each_fprod(150, 2, |j, product, count| seen.push((j, product, count)));
        assert_eq!(seen.len(), 149);
        for (j, product, count) in seen {
            let factors = factor_exponents(j);
            let expect: u128 = factors
                .iter()
                .map(|&(p, _)| u128::from(p.pow(2)) - 1)
                .product();
            assert_eq!((product, count), (expect, factors.len()), "j = {j}");
        }
    }

    #[test]
    fn range_branch_crosses_block_boundary() {
        let nk = RANGE_BLOCK + 10;
        let mut last = 0;
        for_each_fprod(nk, 3, |j, _, _| last = j);
        assert_eq!(last, nk);
        assert_eq!(small_primes(10), vec![2, 3, 5, 7]);
    }
}
