use super::sqrtmod::{crt, power, prime, sub};
use super::{invmod, mulmod_u as mul};
use crate::{factor_exp, gcd, is_power_root, is_prime};

// rootmod.c::_find_ts_generator and _ts_rootmod.
fn ts_root(mut a: u64, k: u64, p: u64, mut y: u64, mut m: u64) -> u64 {
    let (mut e, mut r) = (0_u32, p-1);
    while r % k == 0 { r /= k; e += 1; }
    let mut x = power(a, invmod(k % r,r).unwrap_or(0),p);
    a = mul(power(x,k,p),invmod(a,p).unwrap_or(0),p);
    while a != 1 {
        let (mut l,mut t,mut z) = (1_u32,a,0);
        while t != 1 {
            if l >= e { return 0; }
            z = t; t = power(t,k,p); l += 1;
        }
        let Some(log) = super::znlog(z,m,p) else { return 0; };
        let kz = sub(0,log % k,k);
        m = power(m,kz,p);
        let t = power(y,kz*k.pow(e-l),p);
        e = l-1;
        x = mul(x,t,p);
        y = power(t,k,p);
        if y <= 1 { return 0; }
        a = mul(a,y,p);
    }
    x
}

pub(super) fn split_prime(mut a: u64, k: u64, p: u64) -> u64 {
    a %= p;
    if a <= 1 { return a; }
    if k == 2 { return prime(a,p); }
    let g = gcd(k,p-1);
    for (f,e) in factor_exp(g) {
        let (mut v,mut r) = (0,p-1);
        while r % f == 0 { r /= f; v += 1; }
        let (mut x,mut y,mut m) = (2,0,1);
        while m == 1 {
            y = power(x,r,p);
            if y != 1 { m = power(y,f.pow(v-1),p); }
            x += 1;
        }
        for _ in 0..e { a = ts_root(a,f,p,y,m); if a == 0 { break; } }
        if a == 0 { break; }
    }
    if g != k {
        let pg = (p-1)/g;
        a = power(a,invmod((k/g) % pg,pg).unwrap_or(0),p);
    }
    a
}
fn lift(r: u64, a: u64, k: u64, n: u64) -> Option<u64> {
    let f = sub(power(r,k,n),a,n);
    if f == 0 { return Some(r); }
    let fp = mul(k,power(r,k-1,n),n);
    let d = mul(f,invmod(fp,n).unwrap_or(0),n);
    (d != 0).then(|| sub(r,d,n))
}

/// Return a `k`th root of `a` modulo `n`, or `None`.
/// Port of MPU 0.75 `rootmod.c::rootmod`, `_rootmod_prime_splitk`,
/// and `_rootmod_composite1`.
///
/// ```
/// use math_prime_util::rootmod;
/// assert_eq!(rootmod(13, 6, 107), Some(83));
/// assert_eq!(rootmod(2, 3, 7), None);
/// ```
pub fn rootmod(a: u64, k: u64, n: u64) -> Option<u64> {
    if n == 0 { return None; }
    let a = a % n;
    let mut r;
    if n <= 2 || a <= 1 { r = a; }
    else if k <= 1 { r = if k == 0 { 1 } else { a }; }
    else if let Some(s) = u32::try_from(k).ok().filter(|&k| k < 64).and_then(|k| is_power_root(a,k)) { r = s; }
    else if is_prime(n) != 0 { r = split_prime(a,k,n); }
    else {
        let (mut result,mut modulus) = (0,1);
        for (p,e) in factor_exp(n) {
            r = split_prime(a % p,k,p);
            if power(r,k,p) != a % p { return None; }
            let mut fe = p;
            for _ in 2..=e {
                fe *= p;
                if let Some(s) = lift(r,a,k,fe) { r = s; continue; }
                let m = fe / p / p;
                let alternate = (1..p).find_map(|t| lift(r+t*m,a,k,fe));
                if let Some(s) = alternate { r = s; }
                else {
                    let base = r % p;
                    r = (0..fe/p).map(|t| base+t*p).find(|&s| power(s,k,fe) == a % fe)?;
                }
            }
            result = crt(result,modulus,r,fe);
            modulus *= fe;
        }
        r = result;
    }
    if k == 2 { r = r.min(n-r); }
    (power(r,k,n) == a).then_some(r)
}
