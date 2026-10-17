use crate::{gcd, is_prime};

pub(super) fn multiply(a: u64, b: u64, n: u64) -> u64 {
    u64::try_from(u128::from(a) * u128::from(b) % u128::from(n)).unwrap()
}

pub(super) fn add(a: u64, b: u64, n: u64) -> u64 {
    u64::try_from((u128::from(a) + u128::from(b)) % u128::from(n)).unwrap()
}

pub(super) fn split(n: u64, f: u64) -> Vec<u64> {
    if f <= 1 || f == n {
        vec![n]
    } else {
        vec![f.min(n / f), f.max(n / f)]
    }
}

// Port of the small-factor and primality dispatch in MPU XS.xs.
pub(super) fn dispatch(mut n: u64, method: impl FnOnce(u64) -> Vec<u64>) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut factors = Vec::new();
    for p in [2, 3, 5] {
        while n % p == 0 {
            factors.push(p);
            n /= p;
        }
    }
    if n > 1 {
        if is_prime(n) != 0 {
            factors.push(n);
        } else {
            factors.extend(method(n));
        }
    }
    factors
}

/// Splits `n` with Pollard rho. Cofactors can be composite.
///
/// This ports `factor.c`'s `prho_factor` in MPU 0.75 and its XS wrapper.
/// `None` uses 4,000,000 rounds. Failure returns the unsplit cofactor.
/// Zero returns `[0]`. One returns an empty vector.
///
/// ```
/// use math_prime_util::prho_factor;
/// assert_eq!(prho_factor(403, None), vec![13, 31]);
/// ```
pub fn prho_factor(n: u64, rounds: Option<u64>) -> Vec<u64> {
    dispatch(n, |n| rho(n, rounds.unwrap_or(4_000_000)))
}

fn rho(n: u64, rounds: u64) -> Vec<u64> {
    let (mut u, mut v, mut a) = (7, 7, 1);
    let mut fails = 3;
    for _ in 0..rounds.div_ceil(64) {
        let (old_u, old_v) = (u, v);
        let mut m = 1;
        for _ in 0..64 {
            u = add(multiply(u, u, n), a, n);
            v = add(multiply(v, v, n), a, n);
            v = add(multiply(v, v, n), a, n);
            m = multiply(m, u.abs_diff(v), n);
        }
        let mut f = gcd(m, n);
        if f == 1 {
            continue;
        }
        if f == n {
            u = old_u;
            v = old_v;
            for _ in 0..=64 {
                u = add(multiply(u, u, n), a, n);
                v = add(multiply(v, v, n), a, n);
                v = add(multiply(v, v, n), a, n);
                f = gcd(u.abs_diff(v), n);
                if f != 1 {
                    break;
                }
            }
        }
        if f == 0 || f == n {
            if fails == 0 {
                break;
            }
            fails -= 1;
            u = add(u, 2, n);
            v = u;
            a += 2;
        } else {
            return split(n, f);
        }
    }
    vec![n]
}
