use super::{addmod_u as add, invmod, mulmod_u as mul, powmod, submod};
use crate::{factor_exp, gcd, sqrtint};

pub(super) fn power(a: u64, k: u64, n: u64) -> u64 {
    powmod(a, k, n).unwrap_or(0)
}
pub(super) fn sub(a: u64, b: u64, n: u64) -> u64 {
    submod(a, b, n).unwrap_or(0)
}
pub(super) fn divide(a: u64, b: u64, n: u64) -> u64 {
    let g = gcd(a, b);
    mul(a / g, invmod(b / g, n).unwrap_or(0), n)
}

// rootmod.c::_sqrtmod_prime, using its non-Montgomery u128 path.
pub(super) fn prime(a: u64, p: u64) -> u64 {
    if p == 2 || a <= 1 { return a; }
    if p % 4 == 3 { return power(a, (p >> 2) + 1, p); }
    let a2 = add(a, a, p);
    if p % 8 == 5 {
        let alpha = power(a2, (p - 5) >> 3, p);
        let beta = mul(a2, mul(alpha, alpha, p), p);
        return mul(alpha, mul(a, sub(beta, 1, p), p), p);
    }
    if p % 16 == 9 {
        let mut alpha = power(a2, (p - 9) >> 4, p);
        let mut beta = mul(a2, mul(alpha, alpha, p), p);
        let mut d = 1;
        if mul(beta, beta, p) != p - 1 {
            d = 3;
            while power(d, (p - 1) / 2, p) != p - 1 { d += 2; }
            alpha = mul(alpha, power(d, (p - 9) >> 3, p), p);
            beta = mul(a2, mul(mul(d,d,p), mul(alpha,alpha,p),p),p);
        }
        return mul(alpha, mul(a, mul(d,sub(beta,1,p),p),p),p);
    }
    if power(a, (p - 1) / 2, p) != 1 { return 0; }
    let e = (p - 1).trailing_zeros();
    let q = (p - 1) >> e;
    let mut t = 3;
    while power(t, (p - 1) / 2, p) != p - 1 { t += 2; }
    let mut z = power(t, q, p);
    let mut b = power(a, q, p);
    let mut r = e;
    let mut x = power(a, q.div_ceil(2), p);
    while b != 1 {
        t = b;
        let mut m = 0;
        while m < r && t != 1 { t = mul(t,t,p); m += 1; }
        if m >= r { break; }
        t = power(z, 1 << (r-m-1), p);
        x = mul(x,t,p);
        z = mul(t,t,p);
        b = mul(b,z,p);
        r = m;
    }
    x
}

pub(super) fn prime_power(a: u64, p: u64, e: u32) -> Option<u64> {
    let n = p.pow(e);
    if a % n == 0 { return Some(0); }
    if e == 1 {
        let r = prime(a % p,p);
        return (mul(r,r,p) == a % p).then_some(r.min(p-r));
    }
    if a % (p*p) == 0 { return prime_power(a/(p*p),p,e-2).map(|s| s*p); }
    if a % p == 0 { return None; }
    let ered = if p > 2 || e < 5 { (e+1)>>1 } else { (e+3)>>1 };
    let s = prime_power(a,p,ered)?;
    let np = if p == 2 { n.checked_mul(p).unwrap_or(n) } else { n };
    let r = add(s,divide(sub(a,mul(s,s,np),np),add(s,s,np),n),n);
    (mul(r,r,n) == a % n).then_some(r.min(n-r))
}

pub(super) fn crt(r: u64, n: u64, s: u64, m: u64) -> u64 {
    let t = mul(invmod(n,m).unwrap_or(0),sub(s % m,r % m,m),m);
    r + n*t
}

/// Return a square root of `a` modulo `n`, or `None`.
/// Port of MPU 0.75 `rootmod.c::sqrtmod` and `_sqrtmod_composite`.
///
/// ```
/// use math_prime_util::sqrtmod;
/// assert_eq!(sqrtmod(58, 101), Some(19));
/// assert_eq!(sqrtmod(3, 7), None);
/// ```
pub fn sqrtmod(a: u64, n: u64) -> Option<u64> {
    if n == 0 { return None; }
    let a = a % n;
    if n <= 2 || a <= 1 { return Some(a); }
    // Preserve MPU's small-root lookup table.
    const SMALL: [&[u64]; 14] = [
        &[0], &[0,0], &[0,0,2], &[0,3,2,0], &[3,0,2,0,0],
        &[0,0,2,0,0,0], &[0,0,2,0,0,4,0], &[0,0,2,5,4,0,0,3],
        &[0,5,2,4,0,0,0,3,0], &[0,0,2,0,0,0,0,3,0,0],
        &[0,4,2,0,0,0,0,3,6,0,5], &[4,0,2,0,0,7,6,3,0,5,0,0],
        &[0,0,2,0,6,0,0,3,5,0,0,0,0], &[0,0,2,0,0,0,0,3,0,0,0,0,0,0]];
    if n <= 16 {
        let r = SMALL[usize::try_from(n-3).unwrap()][usize::try_from(a-2).unwrap()];
        return (r != 0).then_some(r);
    }
    let s = sqrtint(a);
    if s*s == a { return Some(s); }
    let (mut r,mut modulus) = (0,1);
    for (p,e) in factor_exp(n) {
        let m = p.pow(e);
        r = crt(r,modulus,prime_power(a,p,e)?,m);
        modulus *= m;
    }
    Some(r.min(n-r))
}
