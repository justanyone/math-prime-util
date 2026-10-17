/// Return trial factors and the remaining cofactor.
///
/// Port of `factor.c::trial_factor` in MPU 0.75, with `first = 2`.
/// The bound is inclusive. A zero bound means no limit.
/// The last value can be composite. One returns an empty list; zero returns `[0]`.
///
/// ```
/// use math_prime_util::trial_factor;
/// assert_eq!(trial_factor(175, 5), vec![5, 5, 7]);
/// assert_eq!(trial_factor(403, 10), vec![403]);
/// ```
pub fn trial_factor(n: u64, limit: u64) -> Vec<u64> {
    trial_from(n, 2, limit)
}

pub(super) fn trial_from(mut n: u64, first: u64, mut last: u64) -> Vec<u64> {
    let first = first.max(2);
    if last == 0 || last > n / last {
        last = u64::MAX;
    }
    if n < 4 || last < first {
        return if n == 1 { Vec::new() } else { vec![n] };
    }
    let mut factors = Vec::new();
    for p in [2, 3, 5] {
        if first <= p && p <= last {
            while n % p == 0 {
                factors.push(p);
                n /= p;
            }
        }
    }
    // MPU's small prime table ends at 2011.
    let mut f = first.max(7);
    while f <= 2011 {
        if crate::is_prime(f) != 0 {
            if f > n / f || f > last {
                break;
            }
            while n % f == 0 {
                factors.push(f);
                n /= f;
            }
        }
        f += 1;
    }
    if f > 2011 {
        const DISTANCE: [u64; 30] = [
            1, 0, 5, 4, 3, 2, 1, 0, 3, 2, 1, 0, 1, 0, 3, 2, 1, 0, 1, 0, 3, 2, 1, 0, 5, 4, 3, 2, 1,
            0,
        ];
        const ADVANCE: [u64; 30] = [
            1, 6, 5, 4, 3, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 2, 1, 4, 3, 2, 1, 6, 5, 4, 3, 2, 1,
            2,
        ];
        if let Some(start) = f.checked_add(DISTANCE[(f % 30) as usize]) {
            f = start;
            while f <= n / f && f <= last {
                while n % f == 0 {
                    factors.push(f);
                    n /= f;
                }
                f += ADVANCE[(f % 30) as usize];
            }
        }
    }
    if n != 1 {
        factors.push(n);
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::trial_from;

    #[test]
    fn first_bound_and_wheel() {
        assert_eq!(trial_from(30, 3, 0), [3, 10]);
        assert_eq!(trial_from(49, 0, 0), [7, 7]);
        assert_eq!(trial_from(25, 7, 5), [25]);
        assert_eq!(trial_from(2017 * 2027, 2012, 0), [2017, 2027]);
        assert_eq!(trial_from(u64::MAX, u64::MAX, 0), [u64::MAX]);
        assert_eq!(trial_from(u64::MAX, u64::MAX - 1, 0), [u64::MAX]);
    }
}
