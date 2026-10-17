use super::urandomm;
use crate::{gcd, is_prime};

// reference/mpu/factor.c: pbrent_factor, non-Montgomery path.
#[allow(clippy::cast_possible_truncation)] // Each remainder is below the u64 modulus.
fn brent(n: u64, mut a: u64) -> Option<u64> {
    let step =
        |x: u64, a: u64| ((u128::from(x) * u128::from(x) + u128::from(a)) % u128::from(n)) as u64;
    let inner = if n <= 4_000_000_000 { 32 } else { 160 };
    let (mut r, mut f, mut xi, mut xm) = (1u64, 1, 1, 1);
    let mut rounds = 500_000;
    let mut fails = 6;
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = xi;
        while left > 0 {
            let count = left.min(inner);
            saved = xi;
            left -= count;
            rounds -= count;
            xi = step(xi, a);
            let mut m = xi.abs_diff(xm);
            for _ in 1..count {
                xi = step(xi, a);
                m = (u128::from(m) * u128::from(xi.abs_diff(xm)) % u128::from(n)) as u64;
            }
            f = gcd(m, n);
            if f != 1 {
                break;
            }
        }
        if f == 1 {
            r *= 2;
            xm = xi;
            continue;
        }
        if f == n {
            xi = saved;
            loop {
                xi = step(xi, a);
                f = gcd(xi.abs_diff(xm), n);
                if f != 1 || r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        if f == 0 || f == n {
            if fails == 0 {
                break;
            }
            fails -= 1;
            xm = ((u128::from(xm) + 11) % u128::from(n)) as u64;
            xi = xm;
            a += 1;
            continue;
        }
        return Some(f);
    }
    None
}

pub(super) fn factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        while n % p == 0 && n > 1 {
            out.push(p);
            n /= p;
        }
    }
    let mut stack = vec![n];
    while let Some(n) = stack.pop() {
        if n <= 1 {
            continue;
        }
        if is_prime(n) != 0 {
            out.push(n);
            continue;
        }
        let mut a = 1;
        let d = loop {
            if let Some(d) = brent(n, a) {
                break d;
            }
            a += 2;
        };
        stack.extend([d, n / d]);
    }
    out.sort_unstable();
    out
}

/// Returns a uniform integer in `1..=n` and its sorted prime factors.
///
/// Port of `reference/mpu/util.c`: `random_factored_integer` (native fast path).
///
/// # Panics
/// Panics if `n` is zero or an unseeded thread cannot read OS entropy.
///
/// ```
/// math_prime_util::srand(15);
/// let (n, factors) = math_prime_util::random_factored_integer(100);
/// assert_eq!(factors.iter().product::<u64>(), n);
/// ```
pub fn random_factored_integer(n: u64) -> (u64, Vec<u64>) {
    assert!(n > 0, "n must be positive");
    let r = 1 + urandomm(n);
    (r, factors(r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_paths() {
        for n in [
            1,
            2,
            4,
            9,
            41,
            41 * 41,
            101 * 103,
            203_389_829 * 2_969_096_527,
            u64::MAX,
        ] {
            let f = factors(n);
            assert_eq!(f.iter().product::<u64>(), n);
            assert!(f.iter().all(|&p| is_prime(p) != 0));
        }
        assert_eq!(brent(101 * 103, 1).map(|d| (101 * 103) % d), Some(0));
    }

    #[test]
    #[should_panic(expected = "n must be positive")]
    fn zero_bound() {
        random_factored_integer(0);
    }
}
