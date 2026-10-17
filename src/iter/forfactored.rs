use std::ops::{ControlFlow, RangeInclusive};
use super::run;

/// Iterator over `(n, factors)` for each `n` in a range, where `factors` are the
/// `(prime, exponent)` pairs of `n`. See `XS.xs`, `forfactored`.
#[derive(Debug, Clone)]
pub struct FactoredIn {
    range: RangeInclusive<u64>,
    segment: std::vec::IntoIter<(u64, Vec<(u64, u32)>)>,
    sieve: bool,
}

impl Iterator for FactoredIn {
    type Item = (u64, Vec<(u64, u32)>);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.sieve {
            return self.range.find(|&n| n > 0).map(|n| (n, crate::factor_exp(n)));
        }
        if let Some(value) = self.segment.next() {
            return Some(value);
        }
        let lo = self.range.next()?;
        let end = *self.range.end();
        let hi = lo.saturating_add(256 * 1024 - 1).min(end);
        self.range = if hi == u64::MAX { 1..=0 } else { hi + 1..=end };
        self.segment = factor_segment(lo, hi).into_iter();
        self.segment.next()
    }
}

/// Iterate over `(n, factors)` for each `n` in a range. `0` is not visited.
/// See `XS.xs`, `forfactored`.
/// ```
/// let v: Vec<_> = math_prime_util::factored_in(1..=4).collect();
/// assert_eq!(v[3], (4, vec![(2, 2)]));
/// ```
#[must_use]
pub fn factored_in(range: RangeInclusive<u64>) -> FactoredIn {
    let range = if range.is_empty() { 1..=0 } else { (*range.start()).max(1)..=*range.end() };
    let sieve = !range.is_empty() && range.end() - range.start() >= 100;
    FactoredIn { range, sieve, segment: Vec::new().into_iter() }
}

/// Visit `n` and its factors for each `n` in an inclusive range. See `XS.xs`, `forfactored`.
/// ```
/// use std::ops::ControlFlow;
/// let mut seen = Vec::new();
/// let _: ControlFlow<()> = math_prime_util::forfactored(12..=12, |n, f| {
///     seen.push((n, f));
///     ControlFlow::Continue(())
/// });
/// assert_eq!(seen, [(12, vec![(2, 2), (3, 1)])]);
/// ```
pub fn forfactored<B>(
    range: RangeInclusive<u64>,
    mut f: impl FnMut(u64, Vec<(u64, u32)>) -> ControlFlow<B>,
) -> ControlFlow<B> {
    run(factored_in(range), |(n, factors)| f(n, factors))
}


// MPU 0.75 factor.c::_fr_full_sieve. Bounds and thresholds are unchanged.
fn full_sieve(root: u64, span: u64) -> bool {
    match root {
        0..10_000_000 => true,
        10_000_000..35_000_000 => span > 900,
        35_000_000..100_000_000 => span > 1700,
        100_000_000..350_000_000 => span > 3400,
        350_000_000..1_000_000_000 => span > 5500,
        1_000_000_000..3_500_000_000 => span > 17000,
        _ => span > 19000,
    }
}

// MPU 0.75 factor.c::_vec_factor, with prime powers stored as exponent pairs.
fn factor_segment(lo: u64, hi: u64) -> Vec<(u64, Vec<(u64, u32)>)> {
    let root = crate::sqrtint(hi);
    let limit = if full_sieve(root, hi - lo) { root } else { crate::rootint(hi, 3) };
    let mut values: Vec<_> = (lo..=hi).map(|n| (n, Vec::new())).collect();
    let mut products = vec![1_u64; values.len()];
    for p in crate::primes_in(2..=limit) {
        let mut power = p;
        while power <= hi / p {
            let offset = (power - lo % power) % power;
            let mut i = offset;
            while i <= hi - lo {
                let j = usize::try_from(i).expect("bounded factor segment");
                let factors: &mut Vec<(u64, u32)> = &mut values[j].1;
                if let Some((last, exponent)) = factors.last_mut().filter(|(last, _)| *last == p) {
                    let _ = last;
                    *exponent += 1;
                } else {
                    factors.push((p, 1));
                }
                products[j] *= p;
                i += power;
            }
            power *= p;
        }
    }
    for ((n, factors), product) in values.iter_mut().zip(products) {
        if product != *n {
            for (p, e) in crate::factor_exp(*n / product) {
                if let Some((_, exponent)) = factors.iter_mut().find(|(q, _)| *q == p) {
                    *exponent += e;
                } else {
                    factors.push((p, e));
                }
            }
            factors.sort_unstable();
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::full_sieve;

    #[test]
    fn sieve_thresholds() {
        assert!(full_sieve(9_999_999, 0));
        for (root, span) in [(10_000_000, 900), (35_000_000, 1700),
            (100_000_000, 3400), (350_000_000, 5500),
            (1_000_000_000, 17000), (3_500_000_000, 19000)] {
            assert!(!full_sieve(root, span));
            assert!(full_sieve(root, span + 1));
        }
    }
}
