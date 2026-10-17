use crate::modular::{addmod_u, mulmod_u};
use crate::{gcd, is_prime, sqrtint};

/// Attempts Chebyshev factoring and returns factors in ascending order.
///
/// Port of `cheb_factor` in MPU `factor.c`, with small-factor handling from
/// `trial_factor` in `XS.xs`. A composite cofactor can remain unsplit.
/// Set `bound` to zero for the MPU bound, and `init_x` to zero to start at 72.
/// Zero returns `[0]`. One returns an empty vector.
///
/// ```
/// assert_eq!(math_prime_util::cheb_factor(403, 0, 0), vec![13, 31]);
/// ```
pub fn cheb_factor(mut n: u64, bound: u64, init_x: u64) -> Vec<u64> {
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
    if n == 1 {
        return factors;
    }
    if is_prime(n) != 0 {
        factors.push(n);
        return factors;
    }
    let mut b = bound;
    if b == 0 {
        b = 8 * u64::from(n.ilog2()).pow(2);
    }
    b = b.min(sqrtint(n));
    let sqrt_b = sqrtint(b);
    let inv = n / 2 + 1;
    let mut x = if init_x == 0 { 72 } else { init_x % n };
    let mut f = 1;
    for p in crate::primes_in(2..=b) {
        let mut power = p;
        if p <= sqrt_b {
            while power <= b / p {
                power *= p;
            }
        }
        let twice_x = addmod_u(x % n, x % n, n);
        x = mulmod_u(lucas_v(twice_x, power, n), inv, n);
        f = gcd(if x > 0 { x - 1 } else { n - 1 }, n);
        if f > 1 {
            break;
        }
    }
    if f > 1 && f < n {
        factors.push(f.min(n / f));
        factors.push(f.max(n / f));
    } else {
        factors.push(n);
    }
    factors
}

// Unsigned Q=1 ladder from lucas_seq.c, lucasvmod.
fn lucas_v(p: u64, k: u64, n: u64) -> u64 {
    let sub = |a, b| if a >= b { a - b } else { n - (b - a) };
    let mut v = p;
    let mut w = sub(mulmod_u(p, p, n), 2);
    for bit in (0..k.ilog2()).rev() {
        let t = sub(mulmod_u(v, w, n), p);
        if k >> bit & 1 == 1 {
            v = t;
            w = sub(mulmod_u(w, w, n), 2);
        } else {
            w = t;
            v = sub(mulmod_u(v, v, n), 2);
        }
    }
    v
}
