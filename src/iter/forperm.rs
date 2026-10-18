use super::run;
use std::ops::ControlFlow;

/// Lexical permutation state from `reference/mpu/XS.xs`, `_comb_iterate`.
#[derive(Debug, Clone)]
pub struct PermutationsOf<T> {
    current: Option<Vec<T>>,
}
fn advance<T: Ord>(a: &mut [T]) -> bool {
    let Some(i) = (1..a.len()).rev().find(|&i| a[i - 1] < a[i]) else {
        return false;
    };
    let j = (i..a.len())
        .rev()
        .find(|&j| a[i - 1] < a[j])
        .expect("greater suffix element");
    a.swap(i - 1, j);
    a[i..].reverse();
    true
}
impl<T: Ord + Clone> Iterator for PermutationsOf<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.current.take()?;
        let mut next = out.clone();
        if advance(&mut next) {
            self.current = Some(next);
        }
        Some(out)
    }
}
/// Iterate over permutations of indices. See `reference/mpu/XS.xs`, `forperm`, `_comb_init`, and `_comb_iterate`.
/// ```
/// assert_eq!(math_prime_util::permutations_of(3).count(), 6);
/// ```
pub fn permutations_of(n: usize) -> PermutationsOf<usize> {
    PermutationsOf {
        current: Some((0..n).collect()),
    }
}

/// Visit each arrangement. See `reference/mpu/XS.xs`, `forperm`.
/// ```
/// assert!(math_prime_util::forperm(3, |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forperm<B>(n: usize, f: impl FnMut(Vec<usize>) -> ControlFlow<B>) -> ControlFlow<B> {
    run(permutations_of(n), f)
}
