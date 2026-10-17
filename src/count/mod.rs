//! Exact prime counts and Legendre's partial sieve.

use std::collections::HashMap;

use crate::{primes, rootint, sqrtint};

mod tables;

struct Counter {
    primes: Vec<u64>,
    limit: u64,
    counts: HashMap<u64, u64>,
    phis: HashMap<(u64, usize), u64>,
}

impl Counter {
    fn new(limit: u64) -> Self {
        Self {
            primes: primes(2, limit),
            limit,
            counts: HashMap::new(),
            phis: HashMap::new(),
        }
    }

    fn phi(&mut self, x: u64, a: usize) -> u64 {
        if a == 0 {
            return x;
        }
        if x == 0 {
            return 0;
        }
        if a == 1 {
            return x - x / 2;
        }
        if a == 2 {
            return x - x / 2 - x / 3 + x / 6;
        }
        if a == 3 {
            return (x / 30) * 8 + u64::from(tables::PHI3[(x % 30) as usize]);
        }
        if a == 4 {
            return (x / 210) * 48 + u64::from(tables::PHI4[(x % 210) as usize]);
        }
        let p = self.primes[a - 1];
        if p >= x {
            return 1;
        }
        if x <= self.limit && p > x / p {
            return self.count(x) - a as u64 + 1;
        }
        if let Some(&value) = self.phis.get(&(x, a)) {
            return value;
        }
        let value = self.phi(x, a - 1) - self.phi(x / p, a - 1);
        // Bound the cache independently of the input size.
        if a <= 100 && x <= 100_000 {
            self.phis.insert((x, a), value);
        }
        value
    }

    // MPU lehmer.c, lehmer_prime_count: the fourth-root split and P2/P3 sums.
    fn count(&mut self, n: u64) -> u64 {
        if n <= self.limit {
            return self.primes.partition_point(|&p| p <= n) as u64;
        }
        if let Some(&value) = self.counts.get(&n) {
            return value;
        }
        let a = self.count(sqrtint(sqrtint(n)));
        let b = self.count(sqrtint(n));
        let c = self.count(rootint(n, 3));
        let mut sum = self.phi(n, usize::try_from(a).expect("prime index fits usize"))
            + (b + a - 2) * (b - a + 1) / 2;
        for i in (a..b).rev() {
            let w = n / self.primes[usize::try_from(i).expect("prime index fits usize")];
            sum -= self.count(w);
            if i < c {
                let bi = self.count(sqrtint(w));
                for j in i..bi {
                    let p = self.primes[usize::try_from(j).expect("prime index fits usize")];
                    sum -= self.count(w / p) - j;
                }
            }
        }
        self.counts.insert(n, sum);
        sum
    }
}

/// Count the primes less than or equal to `n`.
///
/// Port of MPU 0.75 `lehmer.c`, `lehmer_prime_count`.
/// This uses the Lehmer split; the LMO dispatcher is not yet ported.
///
/// ```
/// use math_prime_util::prime_count;
/// assert_eq!(prime_count(100), 25);
/// ```
#[must_use]
pub fn prime_count(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }
    Counter::new(sqrtint(n).max(100)).count(n)
}

/// Count positive integers at most `x` with no factor among the first `a` primes.
///
/// Port of MPU 0.75 `legendre_phi.c`, `legendre_phi` and its phi recurrence.
///
/// ```
/// use math_prime_util::legendre_phi;
/// assert_eq!(legendre_phi(100, 3), 26);
/// assert_eq!(legendre_phi(100, 0), 100);
/// ```
#[must_use]
pub fn legendre_phi(x: u64, a: u64) -> u64 {
    if a == 0 {
        return x;
    }
    if x == 0 || a >= x || a > x / 2 {
        return u64::from(x > 0);
    }
    if a == 1 {
        return x - x / 2;
    }
    if a >= sqrtint(x) || a > 203_280_221 {
        return prime_count(x).saturating_sub(a) + 1;
    }
    let mut limit = 100;
    let mut counter = Counter::new(limit);
    while (counter.primes.len() as u64) < a {
        limit *= 2;
        counter = Counter::new(limit);
    }
    counter.phi(x, usize::try_from(a).expect("prime index fits usize"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_known_values() {
        for (n, count) in [
            (0, 0),
            (1, 0),
            (2, 1),
            (100, 25),
            (10_000, 1229),
            (1_000_000, 78_498),
            (100_000_000, 5_761_455),
        ] {
            assert_eq!(prime_count(n), count);
        }
    }

    #[test]
    fn independent_calls() {
        let threads: Vec<_> = (0..4)
            .map(|_| std::thread::spawn(|| prime_count(100_000)))
            .collect();
        for thread in threads {
            assert_eq!(thread.join().unwrap(), 9592);
        }
        assert_eq!(prime_count(100_000), 9592);
    }
}
