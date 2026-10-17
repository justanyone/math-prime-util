use std::ops::ControlFlow;
use super::run;

/// Combination state from `XS.xs`, `_comb_init` and `_comb_iterate`.
#[derive(Debug, Clone)]
pub struct CombinationsOf {
    n: usize,
    k: usize,
    end: usize,
    current: Option<Vec<usize>>,
}
impl Iterator for CombinationsOf {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.k > self.end || self.k > self.n {
            return None;
        }
        let out = self.current.take().unwrap_or_else(|| (0..self.k).collect());
        let mut next = out.clone();
        if let Some(i) = (0..self.k).rev().find(|&i| next[i] < self.n - self.k + i) {
            next[i] += 1;
            for j in i + 1..self.k {
                next[j] = next[j - 1] + 1;
            }
            self.current = Some(next);
        } else {
            self.k += 1;
        }
        Some(out)
    }
}
/// Iterate over combinations of indices. `None` visits all lengths.
/// See `XS.xs`, `forcomb` and `_comb_iterate`.
/// ```
/// assert_eq!(math_prime_util::combinations_of(3, Some(2)).collect::<Vec<_>>(), [vec![0,1],vec![0,2],vec![1,2]]);
/// ```
pub fn combinations_of(n: usize, k: Option<usize>) -> CombinationsOf {
    CombinationsOf {
        n,
        k: k.unwrap_or(0),
        end: k.unwrap_or(n),
        current: None,
    }
}

/// Visit combinations. See `XS.xs`, `forcomb`.
/// ```
/// assert!(math_prime_util::forcomb(3, Some(2), |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forcomb<B>(
    n: usize,
    k: Option<usize>,
    f: impl FnMut(Vec<usize>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(combinations_of(n, k), f)
}
