use super::run;
use std::ops::ControlFlow;

/// Derangement state. See `reference/mpu/XS.xs`, `_comb_init` and `_comb_iterate`.
#[derive(Debug, Clone)]
pub struct DerangementsOf {
    current: Option<Vec<usize>>,
}
impl Iterator for DerangementsOf {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut cm = self.current.take()?;
        let n = cm.len();
        let out = cm.iter().rev().map(|v| v - 1).collect();
        // Keep the C array reversed. This also preserves its early skip.
        while let Some(j) = (1..n).find(|&j| cm[j] <= cm[j - 1]) {
            let m = (0..n)
                .find(|&m| cm[j] <= cm[m])
                .expect("greater suffix element");
            cm.swap(j, m);
            if cm[j] == n - j {
                continue;
            }
            cm[..j].reverse();
            if cm.iter().rev().enumerate().all(|(i, &v)| v - 1 != i) {
                self.current = Some(cm);
                break;
            }
        }
        Some(out)
    }
}
/// Iterate over permutations with no fixed points. See `XS.xs`, `forderange`.
/// ```
/// assert_eq!(math_prime_util::derangements_of(4).count(), 9);
/// ```
pub fn derangements_of(n: usize) -> DerangementsOf {
    let current = if n == 1 {
        None
    } else {
        let mut cm = vec![0; n];
        for i in 0..n {
            cm[n - i - 1] = if i % 2 == 1 { i } else { i + 2 };
        }
        if n % 2 == 1 {
            cm[0] = n - 2;
            cm[1] = n;
        }
        Some(cm)
    };
    DerangementsOf { current }
}

/// Visit each arrangement. See `reference/mpu/XS.xs`, `forderange`.
/// ```
/// assert!(math_prime_util::forderange(3, |_| math_prime_util::lastfor()).is_break());
/// ```
pub fn forderange<B>(n: usize, f: impl FnMut(Vec<usize>) -> ControlFlow<B>) -> ControlFlow<B> {
    run(derangements_of(n), f)
}
