use super::{invmod, mulmod_u, powmod_u};
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

fn factors(mut n: u64) -> Vec<u64> {
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

pub(super) fn factor_exp(n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    for p in factors(n) {
        if let Some((q, e)) = out.last_mut() {
            if *q == p {
                *e += 1;
                continue;
            }
        }
        out.push((p, 1));
    }
    out
}

// reference/mpu/util.c: znorder, with Euler's phi as an order multiple.
pub(super) fn order(g: u64, n: u64) -> u64 {
    let mut k = n;
    for (p, _) in factor_exp(n) {
        k = k / p * (p - 1);
    }
    for (p, _) in factor_exp(k) {
        while k % p == 0 && powmod_u(g, k / p, n) == 1 {
            k /= p;
        }
    }
    k
}

// Coprime CRT, reference/mpu/util.c: chinese.
pub(super) fn crt(parts: &[(u64, u64)]) -> u64 {
    let (mut x, mut m) = (0, 1);
    for &(a, n) in parts {
        let d = super::submod(a, x % n, n).unwrap();
        let t = mulmod_u(d, invmod(m % n, n).unwrap(), n);
        x += m * t;
        m *= n;
    }
    x
}

pub(super) fn valuation(mut n: u64, p: u64) -> u64 {
    let mut e = 0;
    while n >= p {
        n /= p;
        e += n;
    }
    e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorization_handles_units_powers_and_semiprimes() {
        assert_eq!(factor_exp(1), Vec::<(u64, u32)>::new());
        assert_eq!(factor_exp(1024), vec![(2, 10)]);
        assert_eq!(
            factor_exp(1_000_003 * 1_000_033),
            vec![(1_000_003, 1), (1_000_033, 1)]
        );
    }
}
