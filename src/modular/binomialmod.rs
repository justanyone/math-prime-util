use super::native::{crt, factor_exp, valuation};
use super::{invmod, mulmod_u, powmod_u};
use crate::is_prime;

// reference/mpu/factmod.c: _factorialmod_without_prime.
fn unit_factorial(mut n: u64, p: u64, m: u64) -> u64 {
    let mut r = 1;
    if n >= m {
        if (n / m) % 2 == 1 && (p > 2 || m == 4) {
            r = m - 1;
        }
        n %= m;
    }
    for i in 2..=n {
        if i % p != 0 {
            r = mulmod_u(r, i, m);
        }
    }
    r
}

fn unit_factorial_powers(mut n: u64, p: u64, m: u64) -> u64 {
    let mut r = 1;
    while n > 1 {
        r = mulmod_u(r, unit_factorial(n, p, m), m);
        n /= p;
    }
    r
}

// reference/mpu/factmod.c: _binomial_mod_prime_power.
fn binomial_prime_power(n: u64, mut k: u64, p: u64, e: u32) -> u64 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    k = k.min(n - k);
    let b = valuation(n, p) - valuation(k, p) - valuation(n - k, p);
    if b >= u64::from(e) {
        return 0;
    }
    let m = p.pow(e);
    let mut den = unit_factorial_powers(k, p, m);
    let mut num = 1;
    if k >= m {
        num = unit_factorial_powers(n, p, m);
        den = mulmod_u(den, unit_factorial_powers(n - k, p, m), m);
    } else {
        for i in n - k + 1..=n {
            let mut v = i;
            while v % p == 0 {
                v /= p;
            }
            num = mulmod_u(num, v, m);
        }
    }
    mulmod_u(
        mulmod_u(num, invmod(den, m).unwrap(), m),
        powmod_u(p, b, m),
        m,
    )
}

fn binomial_lucas(mut n: u64, mut k: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 || k > 0 {
        r = mulmod_u(r, binomial_prime_power(n % p, k % p, p, 1), p);
        if r == 0 {
            break;
        }
        n /= p;
        k /= p;
    }
    r
}

// reference/mpu/factmod.c: _binomial_lucas_mod_prime_power (Granville).
fn binomial_granville(n: u64, k: u64, p: u64, mut q: u32) -> u64 {
    let m = p.pow(q);
    let (mut nn, mut kk) = (n, k);
    let mut carries = Vec::new();
    let mut carry = 0;
    while nn > 0 {
        carry = u64::from(nn % p < kk % p + carry);
        carries.push(carry);
        nn /= p;
        kk /= p;
    }
    for i in (1..carries.len()).rev() {
        carries[i - 1] += carries[i];
    }
    if carries[0] >= u64::from(q) {
        return 0;
    }
    q -= u32::try_from(carries[0]).unwrap();
    let m1 = p.pow(q);
    let mut r =
        if (p > 2 || q < 3) && (q as usize) < carries.len() && carries[q as usize - 1] % 2 == 1 {
            m - 1
        } else {
            1
        };
    r = mulmod_u(r, powmod_u(p, carries[0], m), m);
    (nn, kk) = (n, k);
    let mut rr = n - k;
    while nn > 0 {
        let num = unit_factorial(nn % m1, p, m);
        let den = mulmod_u(
            unit_factorial(kk % m1, p, m),
            unit_factorial(rr % m1, p, m),
            m,
        );
        r = mulmod_u(r, mulmod_u(num, invmod(den, m).unwrap(), m), m);
        nn /= p;
        kk /= p;
        rr /= p;
    }
    r
}

/// Return `binomial(n, k) mod m`. Return `None` for a zero modulus.
///
/// Port of `reference/mpu/factmod.c`: `binomialmod`, with Lucas and CRT.
/// ```
/// use math_prime_util::binomialmod;
/// assert_eq!(binomialmod(10, 3, 12), Some(0));
/// ```
pub fn binomialmod(n: u64, k: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }
    if m == 1 {
        return Some(0);
    }
    if k == 0 || k >= n {
        return Some(u64::from(k == 0 || k == n));
    }
    if m == 2 {
        return Some(u64::from(k & !n == 0));
    }
    if is_prime(m) != 0 {
        return Some(binomial_lucas(n, k, m));
    }
    let parts: Vec<_> = factor_exp(m)
        .into_iter()
        .map(|(p, e)| {
            let pm = p.pow(e);
            let b = if e == 1 {
                binomial_lucas(n, k, p)
            } else if k.min(n - k) <= 100_000 && k.min(n - k) < pm {
                binomial_prime_power(n, k, p, e)
            } else {
                binomial_granville(n, k, p, e)
            };
            (b, pm)
        })
        .collect();
    Some(crt(&parts))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::factorialmod;


}
