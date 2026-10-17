use super::run;
use std::ops::ControlFlow;

/// Iterate over distinct multiset permutations in lexical order.
/// See `reference/mpu/lib/Math/Prime/Util/PP.pm`, `formultiperm` and `_multiset_permutations`.
/// ```
/// assert_eq!(math_prime_util::multipermutations_of(&[1,1,2]).count(), 3);
/// ```
pub fn multipermutations_of<T: Ord + Clone>(values: &[T]) -> MultipermutationsOf<T> {
    let mut sorted = values.to_vec();
    sorted.sort();
    let mut counts: Vec<(T, usize)> = Vec::new();
    for value in sorted {
        if let Some(last) = counts.last_mut().filter(|last| last.0 == value) {
            last.1 += 1;
        } else {
            counts.push((value, 1));
        }
    }
    MultipermutationsOf {
        counts,
        prefix: Vec::new(),
        frames: vec![0],
        size: values.len(),
        done: false,
    }
}

/// Explicit recursion stack for `reference/mpu/lib/Math/Prime/Util/PP.pm`, `_multiset_permutations`.
#[derive(Debug, Clone)]
pub struct MultipermutationsOf<T> {
    counts: Vec<(T, usize)>,
    prefix: Vec<usize>,
    frames: Vec<usize>,
    size: usize,
    done: bool,
}
impl<T: Clone> Iterator for MultipermutationsOf<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.done {
            if self.prefix.len() == self.size {
                let out = self
                    .prefix
                    .iter()
                    .map(|&i| self.counts[i].0.clone())
                    .collect();
                self.frames.pop();
                if let Some(i) = self.prefix.pop() {
                    self.counts[i].1 += 1;
                } else {
                    self.done = true;
                }
                return Some(out);
            }
            let depth = self.prefix.len();
            let start = self.frames[depth];
            if let Some(i) = (start..self.counts.len()).find(|&i| self.counts[i].1 > 0) {
                self.frames[depth] = i + 1;
                self.counts[i].1 -= 1;
                self.prefix.push(i);
                self.frames.push(0);
            } else {
                self.frames.pop();
                if let Some(i) = self.prefix.pop() {
                    self.counts[i].1 += 1;
                } else {
                    self.done = true;
                }
            }
        }
        None
    }
}

/// Visit distinct multiset permutations. See `reference/mpu/lib/Math/Prime/Util/PP.pm`, `formultiperm`.
/// ```
/// assert!(math_prime_util::formultiperm(&[1,1,2], |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn formultiperm<T: Ord + Clone, B>(
    values: &[T],
    f: impl FnMut(Vec<T>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(multipermutations_of(values), f)
}
