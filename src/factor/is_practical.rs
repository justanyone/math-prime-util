use crate::{gcd, is_prime};

/// Return whether `n` is a practical number.
///
/// Port of `reference/mpu/util.c`, `is_practical` and `_divsum1`.
/// The divisor sums use `u128` to avoid overflow.
///
/// ```
/// assert!(math_prime_util::is_practical(12));
/// assert!(!math_prime_util::is_practical(10));
/// ```
pub fn is_practical(n: u64) -> bool {
    if n == 0 || n & 1 != 0 {
        return n == 1;
    }
    if n.is_power_of_two() {
        return true;
    }
    if [6, 20, 28, 88, 104, 16].iter().all(|p| n % p != 0) {
        return false;
    }
    let mut factors = Vec::new();
    factor_into(n, &mut factors);
    factors.sort_unstable();
    let mut product = 1u128;
    let mut i = 0;
    while i < factors.len() {
        let p = factors[i];
        if u128::from(p) > product + 1 {
            return false;
        }
        let mut power = 1u128;
        let mut sum = 1u128;
        while i < factors.len() && factors[i] == p {
            power *= u128::from(p);
            sum += power;
            i += 1;
        }
        product *= sum;
    }
    true
}

// Private support until the sibling factor port provides factorint.
pub(super) fn factor_into(n: u64, factors: &mut Vec<u64>) {
    if n <= 1 {
        return;
    }
    if n % 2 == 0 {
        factors.push(2);
        factor_into(n / 2, factors);
    } else if is_prime(n) != 0 {
        factors.push(n);
    } else {
        let d = brent(n);
        factor_into(d, factors);
        factor_into(n / d, factors);
    }
}

// reference/mpu/factor.c, pbrent_factor (non-Montgomery path).
fn brent(n: u64) -> u64 {
    for a in 1u64.. {
        if let Some(factor) = brent_attempt(n, a, 500_000) {
            return factor;
        }
    }
    unreachable!()
}

fn brent_attempt(n: u64, mut a: u64, mut rounds: u64) -> Option<u64> {
    let inner = if n <= 4_000_000_000 { 32 } else { 160 };
    let (mut x, mut anchor, mut r) = (1, 1, 1u64);
    let mut fails = 6;
    while rounds > 0 {
        let step = |x: u64| {
            u64::try_from((u128::from(x) * u128::from(x) + u128::from(a)) % u128::from(n)).unwrap()
        };
        let mut left = r.min(rounds);
        let mut f = 1;
        let mut saved = x;
        while left > 0 {
            let count = left.min(inner);
            saved = x;
            left -= count;
            rounds -= count;
            let mut product = 1u128;
            for _ in 0..count {
                x = step(x);
                product = product * u128::from(x.abs_diff(anchor)) % u128::from(n);
            }
            f = gcd(u64::try_from(product).unwrap(), n);
            if f != 1 {
                break;
            }
        }
        if f == 1 {
            r *= 2;
            anchor = x;
            continue;
        }
        if f == n {
            x = saved;
            loop {
                x = step(x);
                f = gcd(x.abs_diff(anchor), n);
                if f != 1 || r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        if f == n || f == 0 {
            if fails == 0 {
                break;
            }
            fails -= 1;
            anchor = u64::try_from((u128::from(anchor) + 11) % u128::from(n)).unwrap();
            x = anchor;
            a += 1;
            continue;
        }
        return Some(f);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{brent, brent_attempt, factor_into};

    #[test]
    fn brent_limits_and_retries() {
        assert_eq!(brent_attempt(9, 1, 0), None);
        assert_eq!(brent_attempt(101, 1, 500_000), None);
        assert_eq!(brent_attempt(18_446_744_073_709_551_557, 1, 16), None);
        for n in [
            9,
            15,
            25,
            49,
            77,
            121,
            169,
            221,
            1_000_003 * 1_000_033,
            4_294_967_291 * 4_294_967_279,
        ] {
            let f = brent(n);
            assert!(f > 1 && f < n);
            assert_eq!(n % f, 0);
        }
    }

    #[test]
    fn private_factor_support() {
        let mut factors = Vec::new();
        factor_into(0, &mut factors);
        factor_into(1, &mut factors);
        assert_eq!(factors, [] as [u64; 0]);
        factor_into(2 * 3 * 3 * 7, &mut factors);
        factors.sort_unstable();
        assert_eq!(factors, [2, 3, 3, 7]);
    }
}
