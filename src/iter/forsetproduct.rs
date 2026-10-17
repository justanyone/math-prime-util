use super::run;
use std::ops::ControlFlow;

/// Cartesian product state from `reference/mpu/XS.xs`, `forsetproduct`.
#[derive(Debug, Clone)]
pub struct SetProduct<T> {
    sets: Vec<Vec<T>>,
    indices: Vec<usize>,
    done: bool,
}
impl<T: Clone> Iterator for SetProduct<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let out = self
            .sets
            .iter()
            .zip(&self.indices)
            .map(|(s, &i)| s[i].clone())
            .collect();
        self.done = true;
        for i in (0..self.indices.len()).rev() {
            self.indices[i] += 1;
            if self.indices[i] < self.sets[i].len() {
                self.done = false;
                break;
            }
            self.indices[i] = 0;
        }
        Some(out)
    }
}
/// Iterate over a snapshot of the sets. See `reference/mpu/XS.xs`, `forsetproduct`.
/// ```
/// assert_eq!(math_prime_util::set_product(&[vec![1,2],vec![3]]).collect::<Vec<_>>(), [vec![1,3],vec![2,3]]);
/// ```
pub fn set_product<T: Clone>(sets: &[Vec<T>]) -> SetProduct<T> {
    SetProduct {
        sets: sets.to_vec(),
        indices: vec![0; sets.len()],
        done: sets.is_empty() || sets.iter().any(Vec::is_empty),
    }
}

/// Visit a Cartesian product. See `reference/mpu/XS.xs`, `forsetproduct`.
/// ```
/// assert!(math_prime_util::forsetproduct(&[vec![1,2]], |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forsetproduct<T: Clone, B>(
    sets: &[Vec<T>],
    f: impl FnMut(Vec<T>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(set_product(sets), f)
}
