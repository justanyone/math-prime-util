use std::collections::HashMap;
use super::{shuffle, urandomm};

/// Returns an ordered sample of `min(k, n)` distinct values from `0..n`.
///
/// Port of `reference/mpu/util.c`: `randperm` and its sparse Fisher-Yates maps.
/// Pass `n` as `k` for a full permutation.
///
/// # Panics
/// Panics if the result cannot fit in a native allocation, or if an unseeded
/// thread cannot read OS entropy.
///
/// ```
/// math_prime_util::srand(15);
/// assert_eq!(math_prime_util::randperm(5, 5), vec![3, 0, 1, 4, 2]);
/// ```
pub fn randperm(n: u64, k: u64) -> Vec<u64> {
    let k = k.min(n);
    assert!(k <= (isize::MAX as u64) / 8, "requested permutation is too large");
    let len = usize::try_from(k).expect("requested permutation is too large");
    if k == 0 { return Vec::new(); }
    if k == 1 { return vec![urandomm(n)]; }
    if k == n {
        let mut result: Vec<_> = (0..n).collect();
        shuffle(&mut result);
        return result;
    }
    if k == 2 || k == 3 {
        let i = urandomm(n);
        let j = 1 + urandomm(n - 1);
        let mut result = vec![i, if j == i { 0 } else { j }];
        if k == 3 {
            let m = 2 + urandomm(n - 2);
            result.push(if m == j { u64::from(i != 1) } else if m == i { 0 } else { m });
        }
        return result;
    }
    let mut result = Vec::with_capacity(len);
    if k <= 16 || (n >= 32768 && k <= 64) || (n >= 2_147_483_647 && k <= n / 4) || (n >= 100_000 && k <= n / 64) {
        let mut map = HashMap::with_capacity(len);
        for i in 0..k {
            let r = i + urandomm(n - i);
            result.push(*map.get(&r).unwrap_or(&r));
            let vi = *map.get(&i).unwrap_or(&i);
            if r != i { map.insert(r, vi); }
        }
    } else {
        let mut table: Vec<_> = (0..n).collect();
        for i in 0..len {
            let j = i + usize::try_from(urandomm(n - i as u64)).unwrap();
            result.push(table[j]);
            table[j] = table[i];
        }
    }
    result
}
