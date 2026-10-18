#[cfg(test)]
use super::prho_factor::multiply;
use super::prho_factor::{add, dispatch, split};
use crate::gcd;
use crate::int::mont::Mont;

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

/// Pollard-Brent in Montgomery form. Port of `factor.c::pbrent_factor`
/// under `USE_MONTMATH`: `Xi` and `a` stay in Montgomery form, and the
/// 63-bit path evaluates `Xi * (Xi + a)`.
fn brent(n: u64, mut rounds: u64, a: u64) -> Vec<u64> {
    let ctx = Mont::new(n);
    let bits = 64 - n.leading_zeros();
    let inner: u64 = match bits {
        0..=31 => 32,
        32..=35 => 64,
        36..=40 => 160,
        41..=52 => 256,
        _ => 320,
    };
    let (mut xi, mut r, mut fails) = (ctx.one, 1_u64, 6_u32);
    let mut a_m = ctx.to_mont(a % n);
    while rounds > 0 {
        let mut left = r.min(rounds);
        let xm = xi;
        let mut xs = xi;
        let mut f = 1_u64;
        while left > 0 {
            let mut count = left.min(inner);
            left -= count;
            rounds -= count;
            xs = xi;
            xi = mont_step(&ctx, xi, a_m);
            let mut m = xi.abs_diff(xm);
            while count > 1 {
                count -= 1;
                xi = mont_step(&ctx, xi, a_m);
                m = ctx.mul(m, xi.abs_diff(xm));
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
                xi = mont_step(&ctx, xi, a_m);
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
            xi = ctx.one;
            a_m = add(a_m, ctx.to_mont(11), n);
        } else {
            return split(n, f);
        }
    }
    vec![n]
}

/// One Brent iteration. `n < 2^63` or `a ≡ 1` uses `Xi*(Xi+a)`; the other
/// 64-bit case uses `Xi^2 + a`, matching the two arms in `pbrent_factor`.
fn mont_step(ctx: &Mont, xi: u64, a_m: u64) -> u64 {
    if ctx.n < 1 << 63 || a_m == ctx.one {
        let (wrapped, overflow) = xi.overflowing_add(a_m);
        let mut sum = if overflow {
            add(wrapped, ctx.one, ctx.n)
        } else {
            wrapped
        };
        if sum >= ctx.n {
            sum -= ctx.n;
        }
        ctx.mul(xi, sum)
    } else {
        add(ctx.mul(xi, xi), a_m, ctx.n)
    }
}

#[cfg(test)]
fn step(x: u64, a: u64, n: u64) -> u64 {
    // MPU's 64-bit Montgomery path multiplies X by X+a in these cases.
    if n < (1 << 63) || a == 1 {
        multiply(x, add(x, a, n), n)
    } else {
        add(multiply(x, x, n), a, n)
    }
}

#[cfg(test)]
mod tests {
    use super::{brent, step};

    #[test]
    fn retries_and_polynomial_paths() {
        assert_eq!(brent(7, 4096, 0), vec![7]);
        assert_eq!(brent(49, 4096, 1), vec![7, 7]);
        assert_eq!(step(7, 3, 101), 70);
        assert_eq!(step(7, 1, u64::MAX - 58), 56);
        assert_eq!(step(7, 3, u64::MAX - 58), 52);
        for n in [(1_u64 << 33) + 1, (1 << 38) + 1, (1 << 48) + 1, u64::MAX] {
            let factors = brent(n, 512, 2);
            assert_eq!(
                factors.iter().map(|&f| u128::from(f)).product::<u128>(),
                u128::from(n)
            );
        }
    }
}
