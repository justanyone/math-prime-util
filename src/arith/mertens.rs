use super::{moebius, moebius_range};
use crate::{rootint, sqrtint};

// MPU moebius.c::_mertens_context_create clamps the prefix allocation.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn prefix_limit(n: u64) -> u64 {
    let mut limit = rootint(n, 3).pow(2);
    if limit > 100_000_000 {
        let factor = 1.0 + 7.0 * (1.0 - (-(limit as f64) / 8_000_000_000.0).exp());
        limit = (limit as f64 / factor) as u64;
    }
    limit.min(7_613_644_883).max(sqrtint(n))
}

pub(super) struct Mertens {
    prefix: Vec<i16>,
    cache: Vec<(u64, i64)>,
}
impl Mertens {
    pub(super) fn new(n: u64) -> Self {
        let limit = prefix_limit(n);
        let mut prefix = Vec::with_capacity(usize::try_from(limit + 1).unwrap());
        let mut sum = 0;
        // moebius.c::mertens_array uses segments of 32,768 values.
        let mut lo = 0;
        while lo <= limit {
            let hi = (lo + 32_767).min(limit);
            for value in moebius_range(lo, hi) {
                sum += i64::from(value);
                prefix.push(i16::try_from(sum).unwrap());
            }
            lo = hi + 1;
        }
        Self {
            prefix,
            cache: {
                let cube = rootint(n, 3);
                let mut size = crate::next_prime(100 + 8 * cube).unwrap();
                if cube * cube > 100_000_000 {
                    size = crate::next_prime(size * 16).unwrap();
                }
                vec![(0, 0); usize::try_from(size).unwrap()]
            },
        }
    }
    fn cached(&self, n: u64) -> Option<i64> {
        let mut index = usize::try_from(n % u64::try_from(self.cache.len()).unwrap()).unwrap();
        for _ in 0..8 {
            let (key, value) = self.cache[index];
            if key == 0 {
                return None;
            }
            if key == n {
                return Some(value);
            }
            index = (index + 1) % self.cache.len();
        }
        None
    }

    fn insert(&mut self, n: u64, value: i64) {
        let base = usize::try_from(n % u64::try_from(self.cache.len()).unwrap()).unwrap();
        let mut index = base;
        for _ in 0..8 {
            if self.cache[index].0 == 0 || self.cache[index].0 == n {
                self.cache[index] = (n, value);
                return;
            }
            index = (index + 1) % self.cache.len();
        }
        self.cache[base] = (n, value);
    }

    pub(super) fn get(&mut self, n: u64) -> i64 {
        if let Ok(i) = usize::try_from(n) {
            if let Some(&v) = self.prefix.get(i) {
                return i64::from(v);
            }
        }
        if let Some(value) = self.cached(n) {
            return value;
        }
        let s = sqrtint(n);
        let ns = n / (s + 1);
        let mut sum = 1 - i128::from(n / 2) - i128::from(n & 1);
        for k in 2..=ns {
            sum -= i128::from(self.get(n / k))
                + i128::from(self.prefix[usize::try_from(k).unwrap()])
                    * i128::from(n / k - n / (k + 1));
        }
        if s > ns {
            sum -= i128::from(self.prefix[usize::try_from(s).unwrap()])
                * i128::from(n / s - n / (s + 1));
        }
        let sum = i64::try_from(sum).unwrap();
        self.insert(n, sum);
        sum
    }
}

/// Return the sum of Moebius values from one through `n`.
///
/// Port of MPU 0.75 `moebius.c::mertens`, `_small_mertens`,
/// `_mertens_context_create`, and `_rmertens`.
///
/// # Panics
/// Panics if the prefix array is too large to allocate.
///
/// ```
/// assert_eq!(math_prime_util::mertens(10), -1);
/// assert_eq!(math_prime_util::mertens(0), 0);
/// ```
pub fn mertens(n: u64) -> i64 {
    if n <= 512 {
        const MV16: [i8; 33] = [
            0, -1, -4, -3, -1, -4, 2, -4, -2, -1, 0, -4, -5, -3, 3, -1, -1, -3, -7, -2, -4, 2, 1,
            -1, -2, 1, 1, -3, -6, -6, -6, -5, -4,
        ];
        let block = n / 16;
        return i64::from(MV16[usize::try_from(block).unwrap()])
            + (block * 16 + 1..=n)
                .map(|j| i64::from(moebius(j)))
                .sum::<i64>();
    }
    Mertens::new(n).get(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_bounds() {
        assert_eq!(prefix_limit(0), 0);
        assert_eq!(prefix_limit(1_000_000), 10_000);
        assert!(prefix_limit(1_001_000_000_000) < rootint(1_001_000_000_000, 3).pow(2));
        assert_eq!(prefix_limit(u64::MAX), 7_613_644_883);
    }

    #[test]
    fn bounded_cache_collisions() {
        let mut ctx = Mertens {
            prefix: vec![],
            cache: vec![(0, 0); 11],
        };
        assert_eq!(ctx.cached(10), None);
        for k in 0..8 {
            ctx.insert(10 + 11 * k, -1);
        }
        assert_eq!(ctx.cached(87), Some(-1));
        assert_eq!(ctx.cached(98), None);
        ctx.insert(87, 2);
        assert_eq!(ctx.cached(87), Some(2));
        ctx.insert(98, 3);
        assert_eq!(ctx.cached(98), Some(3));
        assert_eq!(ctx.cached(10), None);
    }

    #[test]
    fn segmented_prefix() {
        let ctx = Mertens::new(8_000_000);
        assert_eq!(ctx.prefix.len(), 40_001);
        assert_eq!(
            i64::from(ctx.prefix[40_000]),
            (1..=40_000).map(|n| i64::from(moebius(n))).sum::<i64>()
        );
    }

    #[test]
    fn cached_quotients() {
        let mut ctx = Mertens::new(10_000);
        assert_eq!(ctx.get(10_000), -23);
        assert_eq!(ctx.get(10_000), -23);
        assert_eq!(ctx.get(100), 1);
    }
}
