use crate::{gcd, is_prime, rootint, sqrtint};

/// Return prime factors with multiplicity, in ascending order.
///
/// Port of MPU 0.75 `factor.c::factor`, `_small_trial_factor`, and
/// `_power_factor`. The split helpers use `holf32` and `pbrent_factor`.
/// One returns an empty list. Zero returns `[0]`.
///
/// ```
/// use math_prime_util::factor;
/// assert_eq!(factor(360), vec![2, 2, 2, 3, 3, 5]);
/// assert_eq!(factor(1), Vec::<u64>::new());
/// ```
pub fn factor(n: u64) -> Vec<u64> {
    if n < 4 {
        return if n == 1 { Vec::new() } else { vec![n] };
    }
    let mut factors = super::trial_factor(n, 421);
    let remainder = factors.pop().unwrap_or(1);
    let limit = if remainder < 2017 * 2017 { 2011 } else { 421 };
    let mut rest = super::trial_factor::trial_from(remainder, 431, limit);
    let remainder = rest.pop().unwrap_or(1);
    factors.extend(rest);
    let mut stack = vec![remainder];
    while let Some(n) = stack.pop() {
        if n == 1 {
            continue;
        }
        if is_prime(n) != 0 {
            factors.push(n);
            continue;
        }
        let power = crate::int::is_power_exp(n);
        if power > 1 {
            let root = rootint(n, power);
            let roots = factor(root);
            for _ in 0..power {
                factors.extend_from_slice(&roots);
            }
            continue;
        }
        let divisor = split(n);
        stack.push(divisor);
        stack.push(n / divisor);
    }
    factors.sort_unstable();
    factors
}

// MPU factor.c::holf32, with its 5040 multiplier and 10000-round bound.
fn holf(n: u64) -> Option<u64> {
    let step = n * 5040;
    let mut ni = step;
    for _ in 0..10_000 {
        let s = sqrtint(ni) + 1;
        let m = s * s - ni;
        let root = sqrtint(m);
        if root * root == m {
            let f = gcd(n, s - root);
            if f > 1 && f < n {
                return Some(f);
            }
        }
        ni = ni.checked_add(step)?;
    }
    None
}

fn split(n: u64) -> u64 {
    if n <= u64::from(u32::MAX) {
        if let Some(f) = holf(n) {
            return f;
        }
    }
    let bits = u64::from(u64::BITS - n.leading_zeros());
    let rounds = 100 + 100 * bits.saturating_sub(45);
    for (rounds, a) in [(rounds, 1), (2 * rounds, 3), (500_000, 5)] {
        if let Some(f) = brent(n, rounds, a) {
            return f;
        }
    }
    // Other method families are separate port rows. Continue Brent with
    // distinct polynomials until this remaining composite splits.
    let mut a = 7;
    loop {
        if let Some(f) = brent(n, 500_000, a) {
            return f;
        }
        a += 2;
    }
}

// MPU factor.c::pbrent_factor, non-Montgomery u128 arithmetic path.
fn brent(n: u64, mut rounds: u64, mut a: u64) -> Option<u64> {
    let inner = if n <= 4_000_000_000 { 32 } else { 160 };
    let mut fails = 6;
    let (mut r, mut xi, mut xm) = (1u64, 1u64, 1u64);
    while rounds > 0 {
        let mut left = r.min(rounds);
        let mut saved = xi;
        let mut f = 1;
        while left > 0 {
            let count = left.min(inner);
            saved = xi;
            left -= count;
            rounds -= count;
            xi = square_add(xi, a, n);
            let mut m = xi.abs_diff(xm);
            for _ in 1..count {
                xi = square_add(xi, a, n);
                m = mul_mod(m, xi.abs_diff(xm), n);
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
                xi = square_add(xi, a, n);
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

#[allow(clippy::cast_possible_truncation)] // A residue modulo a u64 fits u64.
fn mul_mod(a: u64, b: u64, n: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(n)) as u64
}

#[allow(clippy::cast_possible_truncation)] // A residue modulo a u64 fits u64.
fn square_add(x: u64, a: u64, n: u64) -> u64 {
    ((u128::from(x) * u128::from(x) + u128::from(a)) % u128::from(n)) as u64
}
