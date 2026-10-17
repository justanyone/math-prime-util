use super::allsqrtmod::{check_count, combine, roots_pk};
use super::sqrtmod::{divide, power, sub};
use super::{addmod_u as add, invmod, mulmod_u as mul};
use crate::{factor, factor_exp, gcd};

// rootmod.c::_ts_prime. Returns a root and a root of unity.
fn ts_prime(a: u64, k: u64, p: u64) -> (u64,u64) {
    let mut r = p-1;
    while r % k == 0 { r /= k; }
    let mut ke = (p-1)/r;
    let mut x = power(a,invmod(k % r,r).unwrap_or(0),p);
    let mut b = mul(power(x,k,p),invmod(a,p).unwrap_or(0),p);
    let (mut candidate,mut y,mut t) = (2,1,0);
    while y == 1 { t = power(candidate,r,p); y = power(t,ke/k,p); candidate += 1; }
    while ke != k {
        ke /= k;
        let old_t = t;
        t = power(t,k,p);
        let mut a = power(b,ke/k,p);
        while a != 1 { x = mul(x,old_t,p); b = mul(b,t,p); a = mul(a,y,p); }
    }
    (x,t)
}
fn prime_roots(a: u64, k: u64, p: u64) -> Vec<u64> {
    let a = a % p;
    if p == 2 || a == 0 { return vec![a]; }
    let g = gcd(k,p-1);
    if g == 1 { return vec![power(a,invmod(k % (p-1),p-1).unwrap_or(0),p)]; }
    if power(a,(p-1)/g,p) != 1 { return vec![]; }
    let (r,z) = ts_prime(a,k,p);
    check_count(k);
    let mut roots = vec![r];
    let mut next = mul(r,z,p);
    while next != r && (roots.len() as u64) < k { roots.push(next); next = mul(next,z,p); }
    roots
}
// rootmod.c::_allrootmod_prime_power.
fn prime_power(a: u64, k: u64, p: u64, e: u32) -> Vec<u64> {
    if k == 2 { return roots_pk(a,p,e); }
    if e == 1 { return prime_roots(a,k,p); }
    let n = p.pow(e);
    if a % n == 0 {
        let t = u64::from(e-1)/k+1;
        let t = u32::try_from(t).unwrap();
        let count = p.pow(e-t);
        check_count(count);
        return (0..count).map(|i| i*p.pow(t)).collect();
    }
    if let Some(pk) = u32::try_from(k).ok().and_then(|k| p.checked_pow(k)) {
        if a % pk == 0 {
            let k32 = u32::try_from(k).unwrap();
            let roots = prime_power(a/pk,k,p,e-k32);
            let pe1 = p.pow(k32-1);
            let pek = p.pow(e-k32+1);
            check_count(roots.len() as u64 * pe1);
            return roots.iter().flat_map(|&r| (0..pe1).map(move |j| r*p+j*pek)).collect();
        }
    }
    if a % p == 0 { return vec![]; }
    let np = if k == p { n.checked_mul(p).unwrap_or(n) } else { n };
    let ered = if p > 2 || e < 5 { (e+1)>>1 } else { (e+3)>>1 };
    let roots = prime_power(a,k,p,ered);
    let mut result = vec![];
    for s in roots {
        let t = power(s,k-1,np);
        let r = add(s,divide(sub(a,mul(t,s,np),np),mul(k,t,np),n),n);
        if k != p { result.push(r); }
        else if power(r,k,n) == a % n {
            check_count(result.len() as u64 + k);
            for i in 0..k { result.push(mul(r,add(mul(i,n/p,n),1,n),n)); }
        }
    }
    result.sort_unstable(); result.dedup(); result
}
fn kprime(a: u64, k: u64, factors: &[(u64,u32)]) -> Vec<u64> {
    let (mut roots,mut modulus) = (vec![0],1);
    for &(p,e) in factors {
        let n = p.pow(e);
        roots = combine(&roots,modulus,&prime_power(a,k,p,e),n);
        if roots.is_empty() { break; }
        modulus *= n;
    }
    roots
}

/// Return all `k`th roots in ascending order. Return an empty list if none exist.
/// Port of MPU 0.75 `rootmod.c::allrootmod` and its prime-power helpers.
///
/// # Panics
/// Panics if the result exceeds MPU's limit of 600 million roots.
///
/// ```
/// use math_prime_util::allrootmod;
/// assert_eq!(allrootmod(13, 6, 107), vec![24, 83]);
/// ```
pub fn allrootmod(a: u64, k: u64, n: u64) -> Vec<u64> {
    if n == 0 { return vec![]; }
    let a = a % n;
    if n == 1 { return vec![0]; }
    if k == 0 {
        if a != 1 { return vec![]; }
        check_count(n); return (0..n).collect();
    }
    if n == 2 || k == 1 { return vec![a]; }
    let factors = factor_exp(n);
    let mut roots = vec![a];
    for prime in factor(k) {
        let mut next = vec![];
        for r in roots {
            let more = kprime(r,prime,&factors);
            check_count(next.len() as u64 + more.len() as u64);
            next.extend(more);
        }
        roots = next;
        if roots.is_empty() { break; }
    }
    roots.sort_unstable(); roots
}
