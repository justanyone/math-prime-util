use super::{addmod_u, invmod, mulmod_u, powmod_u};
use crate::{gcd, is_prime, sqrtint};

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


fn factor_exp(n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    for p in factors(n) {
        if let Some((q, e)) = out.last_mut() {
            if *q == p { *e += 1; continue; }
        }
        out.push((p, 1));
    }
    out
}

// reference/mpu/util.c: znorder, with Euler's phi as an order multiple.
fn order(g: u64, n: u64) -> u64 {
    let mut k = n;
    for (p, _) in factor_exp(n) { k = k / p * (p - 1); }
    for (p, _) in factor_exp(k) {
        while k % p == 0 && powmod_u(g, k / p, n) == 1 { k /= p; }
    }
    k
}

// Coprime CRT, reference/mpu/util.c: chinese.
fn crt(parts: &[(u64, u64)]) -> u64 {
    let (mut x, mut m) = (0, 1);
    for &(a, n) in parts {
        let d = super::submod(a, x % n, n).unwrap();
        let t = mulmod_u(d, invmod(m % n, n).unwrap(), n);
        x += m * t;
        m *= n;
    }
    x
}

/// Return `n! mod m`. Return `None` for a zero modulus.
///
/// Port of `reference/mpu/factmod.c`: `factorialmod` and `_facmod`.
/// ```
/// use math_prime_util::factorialmod;
/// assert_eq!(factorialmod(10, 101), Some(72));
/// ```
pub fn factorialmod(n: u64, m: u64) -> Option<u64> {
    if m == 0 { return None; }
    if n >= m || m == 1 { return Some(0); }
    if n <= 1 { return Some(1); }
    let prime = is_prime(m) != 0;
    let d = if prime && n > m / 2 { m - n - 1 } else { n };
    if d < 2 { return Some(if d == 0 { m - 1 } else { 1 }); }
    if d > 100 && !prime && factor_exp(m).iter().all(|&(p,e)| valuation(n,p) >= u64::from(e)) {
        return Some(0);
    }
    let mut r = 1;
    if d < 1000 {
        for i in 2..=d { r = mulmod_u(r,i,m); if r == 0 { break; } }
    } else {
        // _facmod's prime valuation product, using the crate's segmented sieve.
        for p in crate::primes_in(2..=d) { r = mulmod_u(r,powmod_u(p,valuation(d,p),m),m); }
    }
    if d != n && r != 0 {
        if d % 2 == 0 { r = m - r; }
        return invmod(r,m);
    }
    Some(r)
}

fn valuation(mut n: u64, p: u64) -> u64 {
    let mut e = 0;
    while n >= p { n /= p; e += n; }
    e
}

// reference/mpu/factmod.c: _factorialmod_without_prime.
fn unit_factorial(mut n: u64, p: u64, m: u64) -> u64 {
    let mut r = 1;
    if n >= m {
        if (n / m) % 2 == 1 && (p > 2 || m == 4) { r = m - 1; }
        n %= m;
    }
    for i in 2..=n { if i % p != 0 { r = mulmod_u(r,i,m); } }
    r
}

fn unit_factorial_powers(mut n: u64, p: u64, m: u64) -> u64 {
    let mut r = 1;
    while n > 1 { r = mulmod_u(r,unit_factorial(n,p,m),m); n /= p; }
    r
}

// reference/mpu/factmod.c: _binomial_mod_prime_power.
fn binomial_prime_power(n: u64, mut k: u64, p: u64, e: u32) -> u64 {
    if k > n { return 0; }
    if k == 0 || k == n { return 1; }
    k = k.min(n-k);
    let b = valuation(n,p) - valuation(k,p) - valuation(n-k,p);
    if b >= u64::from(e) { return 0; }
    let m = p.pow(e);
    let mut den = unit_factorial_powers(k,p,m);
    let mut num = 1;
    if k >= m {
        num = unit_factorial_powers(n,p,m);
        den = mulmod_u(den,unit_factorial_powers(n-k,p,m),m);
    } else {
        for i in n-k+1..=n {
            let mut v = i;
            while v % p == 0 { v /= p; }
            num = mulmod_u(num,v,m);
        }
    }
    mulmod_u(mulmod_u(num,invmod(den,m).unwrap(),m),powmod_u(p,b,m),m)
}

fn binomial_lucas(mut n: u64, mut k: u64, p: u64) -> u64 {
    let mut r = 1;
    while n > 0 || k > 0 {
        r = mulmod_u(r,binomial_prime_power(n%p,k%p,p,1),p);
        if r == 0 { break; }
        n /= p; k /= p;
    }
    r
}

// reference/mpu/factmod.c: _binomial_lucas_mod_prime_power (Granville).
fn binomial_granville(n: u64, k: u64, p: u64, mut q: u32) -> u64 {
    let m = p.pow(q);
    let (mut nn, mut kk, mut rr) = (n,k,n-k);
    let mut carries = Vec::new();
    let mut carry = 0;
    while nn > 0 {
        carry = u64::from(nn%p < kk%p + carry);
        carries.push(carry);
        nn /= p; kk /= p; rr /= p;
    }
    for i in (1..carries.len()).rev() { carries[i-1] += carries[i]; }
    if carries[0] >= u64::from(q) { return 0; }
    q -= u32::try_from(carries[0]).unwrap();
    let m1 = p.pow(q);
    let mut r = if (p > 2 || q < 3) && (q as usize) < carries.len() && carries[q as usize-1] % 2 == 1 { m-1 } else { 1 };
    r = mulmod_u(r,powmod_u(p,carries[0],m),m);
    (nn,kk,rr) = (n,k,n-k);
    while nn > 0 {
        let num = unit_factorial(nn%m1,p,m);
        let den = mulmod_u(unit_factorial(kk%m1,p,m),unit_factorial(rr%m1,p,m),m);
        r = mulmod_u(r,mulmod_u(num,invmod(den,m).unwrap(),m),m);
        nn /= p; kk /= p; rr /= p;
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
    if m == 0 { return None; }
    if m == 1 { return Some(0); }
    if k == 0 || k >= n { return Some(u64::from(k == 0 || k == n)); }
    if m == 2 { return Some(u64::from(k & !n == 0)); }
    if is_prime(m) != 0 { return Some(binomial_lucas(n,k,m)); }
    let parts: Vec<_> = factor_exp(m).into_iter().map(|(p,e)| {
        let pm = p.pow(e);
        let b = if e == 1 { binomial_lucas(n,k,p) }
        else if k.min(n-k) <= 100_000 && k.min(n-k) < pm { binomial_prime_power(n,k,p,e) }
        else { binomial_granville(n,k,p,e) };
        (b,pm)
    }).collect();
    Some(crt(&parts))
}
