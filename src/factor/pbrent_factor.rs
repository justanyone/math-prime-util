use super::prho_factor::{add, dispatch, multiply, split};
use crate::gcd;

/// Splits `n` with Pollard rho and Brent updates.
///
/// This ports `factor.c`'s `pbrent_factor` in MPU 0.75 and its XS wrapper.
/// `None` uses 4,000,000 rounds and parameter 1. Cofactors can be composite.
/// Failure returns the unsplit cofactor. Zero returns `[0]`; one returns `[]`.
///
/// ```
/// use math_prime_util::pbrent_factor;
/// assert_eq!(pbrent_factor(403, None, None), vec![13, 31]);
/// ```
pub fn pbrent_factor(n: u64, rounds: Option<u64>, a: Option<u64>) -> Vec<u64> {
    dispatch(n, |n| brent(n, rounds.unwrap_or(4_000_000), a.unwrap_or(1)))
}

fn brent(n: u64, mut rounds: u64, mut a: u64) -> Vec<u64> {
    let inner = match n.ilog2() + 1 {
        0..=31 => 32,
        32..=35 => 64,
        36..=40 => 160,
        41..=52 => 256,
        _ => 320,
    };
    let (mut xi, mut r, mut fails) = (1_u64, 1_u64, 6);
    a %= n;
    while rounds > 0 {
        let mut left = r.min(rounds);
        let xm = xi;
        let mut xs = xi;
        let mut f = 1;
        while left > 0 {
            let count = left.min(inner);
            left -= count;
            rounds -= count;
            xs = xi;
            let mut m = 1;
            for _ in 0..count {
                xi = step(xi, a, n);
                m = multiply(m, xi.abs_diff(xm), n);
            }
            f = gcd(m, n);
            if f != 1 {
                break;
            }
        }
        if f == 1 {
            r = r.saturating_mul(2);
            continue;
        }
        if f == n {
            xi = xs;
            loop {
                xi = step(xi, a, n);
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
            xi = 1;
            a = add(a, 11, n);
        } else {
            return split(n, f);
        }
    }
    vec![n]
}

fn step(x: u64, a: u64, n: u64) -> u64 {
    // MPU's 64-bit Montgomery path multiplies X by X+a in these cases.
    if n < (1 << 63) || a == 1 {
        multiply(x, add(x, a, n), n)
    } else {
        add(multiply(x, x, n), a, n)
    }
}
