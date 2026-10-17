//! Exact prime counts and Legendre's partial sieve.

use std::collections::HashMap;

use crate::{primes, rootint, sqrtint};

mod prime_count_approx;
mod prime_count_lower;
mod prime_count_upper;
mod tables;

pub use prime_count_approx::prime_count_approx;
pub use prime_count_lower::prime_count_lower;
pub use prime_count_upper::prime_count_upper;

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
    // The shortcut above bounds a by 203,280,221, which fits usize.
    #[allow(clippy::cast_possible_truncation)]
    let index = a as usize;
    counter.phi(x, index)
}

/// Return the `n`th prime, with the first prime as index 1.
///
/// Port of MPU 0.75 `prime_counts.c`, `nth_prime`. MPU narrows the
/// search with table lookups and a segment sieve. This port uses a
/// binary search on the exact count from [`prime_count`].
///
/// Returns `None` for `n = 0`, where MPU returns `undef`.
///
/// ```
/// use math_prime_util::nth_prime;
/// assert_eq!(nth_prime(1), Some(2));
/// assert_eq!(nth_prime(100), Some(541));
/// assert_eq!(nth_prime(0), None);
/// ```
#[must_use]
pub fn nth_prime(n: u64) -> Option<u64> {
    const SMALL: [u64; 5] = [2, 3, 5, 7, 11];
    if n == 0 {
        return None;
    }
    if let Some(&p) = usize::try_from(n - 1).ok().and_then(|i| SMALL.get(i)) {
        return Some(p);
    }
    // Rosser's bound: p_n < n (ln n + ln ln n) for n >= 6.
    #[allow(clippy::cast_precision_loss)]
    let fnn = n as f64;
    let ln = fnn.ln();
    let mut lo = SMALL[4] + 1;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let mut hi = (fnn * (ln + ln.ln())) as u64 + 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if prime_count(mid) < n {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    Some(lo)
}

/// Return the sum of `ln p` over the primes `p <= n`, the Chebyshev theta function.
///
/// Port of MPU 0.75 `real.c`, `chebyshev_theta`. MPU sums in long double.
/// This port sums in `f64`, so results can differ in the last digits.
///
/// ```
/// use math_prime_util::chebyshev_theta;
/// assert!((chebyshev_theta(10) - 210.0_f64.ln()).abs() < 1e-12);
/// ```
#[must_use]
pub fn chebyshev_theta(n: u64) -> f64 {
    primes(2, n).iter().map(|&p| ln_u64(p)).sum()
}

/// Return the Chebyshev psi function, the sum of `theta(n^(1/k))` over `k >= 1`.
///
/// Port of MPU 0.75 `real.c`, `chebyshev_psi`. The sum runs over
/// `k = log2(n)` down to `1`, as in MPU.
///
/// ```
/// use math_prime_util::{chebyshev_psi, chebyshev_theta};
/// let psi = chebyshev_psi(100);
/// assert!(psi > 80.0 && psi < 100.0);
/// assert!(chebyshev_theta(100) < psi);
/// ```
#[must_use]
pub fn chebyshev_psi(n: u64) -> f64 {
    if n < 2 {
        return 0.0;
    }
    (1..=n.ilog2())
        .rev()
        .map(|k| chebyshev_theta(rootint(n, k)))
        .sum()
}

#[allow(clippy::cast_precision_loss)]
fn ln_u64(n: u64) -> f64 {
    (n as f64).ln()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_prime_known_values() {
        for (n, p) in [(1, 2), (5, 11), (6, 13), (100, 541), (10_000, 104_729)] {
            assert_eq!(nth_prime(n), Some(p));
        }
        assert_eq!(nth_prime(0), None);
    }

    #[test]
    fn chebyshev_known_values() {
        assert_eq!(chebyshev_theta(1), 0.0);
        assert_eq!(chebyshev_psi(1), 0.0);
        assert_eq!(chebyshev_psi(0), 0.0);
        assert!((chebyshev_theta(10) - 210.0_f64.ln()).abs() < 1e-12);
    }

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
