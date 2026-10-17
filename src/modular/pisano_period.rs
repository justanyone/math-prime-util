use super::native::factor_exp;
use super::{addmod_u, mulmod_u};

// Fibonacci doubling supplies lucasumod(1,-1,k,n) without signed moduli.
fn fibonacci(k: u128, n: u64) -> (u64, u64) {
    if k == 0 {
        return (0, 1 % n);
    }
    let (a, b) = fibonacci(k / 2, n);
    let c = mulmod_u(a, super::submod(addmod_u(b, b, n), a, n).unwrap(), n);
    let d = addmod_u(mulmod_u(a, a, n), mulmod_u(b, b, n), n);
    if k % 2 == 0 {
        (c, d)
    } else {
        (d, addmod_u(c, d, n))
    }
}

/// Return the period of the Fibonacci sequence modulo `n`.
///
/// Port of `reference/mpu/util.c`: `_pisano_prime_power` and `pisano_period`.
/// The result uses `u128` since a native modulus can have a larger period.
/// ```
/// use math_prime_util::pisano_period;
/// assert_eq!(pisano_period(10), 60);
/// ```
pub fn pisano_period(n: u64) -> u128 {
    if n <= 1 {
        return u128::from(n);
    }
    let mut k = 1u128;
    for (p, e) in factor_exp(n) {
        let base = match p {
            2 => 3,
            3 => 8,
            5 => 20,
            7 => 16,
            _ if p < 300 => {
                let (mut a, mut b, mut j) = (1, 1, 1);
                while a != 0 || b != 1 {
                    (a, b) = (b, addmod_u(a, b, p));
                    j += 1;
                }
                j
            }
            _ => {
                let mut j = if p % 5 == 1 || p % 5 == 4 {
                    p - 1
                } else {
                    p + 1
                };
                for (q, e) in factor_exp(j) {
                    for _ in 0..e {
                        if fibonacci(u128::from(j / q), p).0 != 0 {
                            break;
                        }
                        j /= q;
                    }
                }
                j
            }
        };
        let part = u128::from(base) * u128::from(p).pow(e - 1);
        let (mut x, mut y) = (k, part);
        while y != 0 {
            (x, y) = (y, x % y);
        }
        k = k / x * part;
    }
    let mut r = k;
    while fibonacci(r - 1, n).0 != 1 {
        r += k;
    }
    r
}

